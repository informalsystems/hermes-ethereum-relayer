use beacon_api::errors::Error as BeaconError;
use cgp::prelude::{CanRaiseError, HasErrorType};
use eth_protos::union::ibc::lightclients::ethereum::v1::{
    LightClientUpdate as LightClientUpdateProto, SyncCommittee as SyncCommitteeProto,
};
use hermes_chain_components::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use unionlabs::ethereum::config::{
    BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES, SYNC_COMMITTEE_SIZE,
};
use unionlabs::ibc::core::client::height::Height;
use unionlabs::ibc::lightclients::ethereum::account_update::AccountUpdate;
use unionlabs::ibc::lightclients::ethereum::header::Header;
use unionlabs::ibc::lightclients::ethereum::light_client_update::{
    LightClientUpdate, TryFromLightClientUpdateError, UnboundedLightClientUpdate,
};
use unionlabs::ibc::lightclients::ethereum::sync_committee::{
    SyncCommittee, TryFromSyncCommitteeError,
};
use unionlabs::ibc::lightclients::ethereum::trusted_sync_committee::{
    ActiveSyncCommittee, TrustedSyncCommittee,
};

use crate::impls::account_proof::CanBuildAccountProof;
use crate::traits::fields::beacon::HasBeaconApiClient;
use crate::traits::fields::provider::HasAlloyProvider;
use crate::traits::types::beacon_preset::HasBeaconPreset;
use crate::types::eth::update_client_payload::EthUpdateClientPayload;

pub struct BuildEthUpdateClientPayload;

impl<Chain, Counterparty, Preset> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildEthUpdateClientPayload
where
    Chain: HasUpdateClientPayloadType<
            Counterparty,
            UpdateClientPayload = EthUpdateClientPayload<Preset>,
        > + HasClientStateType<Counterparty>
        + HasHeightType<Height = Height>
        + HasBeaconPreset<BeaconPreset = Preset>
        + HasErrorType
        + CanRaiseError<&'static str>
        + CanRaiseError<String>
        + CanRaiseError<TryFromLightClientUpdateError>
        + CanRaiseError<TryFromSyncCommitteeError>
        + CanRaiseError<BeaconError>
        + CanBuildAccountProof
        + HasBeaconApiClient
        + HasAlloyProvider,
    Preset: SYNC_COMMITTEE_SIZE + BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES + Clone,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &Chain::Height,
        // TODO(rano): maybe this should be called minimum target height
        target_height: &Chain::Height,
        _client_state: Chain::ClientState,
    ) -> Result<EthUpdateClientPayload<Preset>, Chain::Error> {
        if target_height.revision_number != trusted_height.revision_number {
            return Err(Chain::raise_error("revision number mismatch"));
        }

        // need to know the finality update at the target height. so, fetching the latest one.
        // so, it is possible that the update is in the future of the target height.

        let finality_update = chain
            .beacon_api_client()
            .finality_update()
            .await
            .map_err(Chain::raise_error)?
            .data;

        if !(target_height.revision_height <= finality_update.finalized_header.beacon.slot) {
            return Err(Chain::raise_error("target height is not finalized yet"));
        }

        let target_height = Chain::Height {
            revision_number: trusted_height.revision_number,
            revision_height: finality_update.finalized_header.beacon.slot,
        };

        // we only update at finality headers.
        // we can't skip header updates more than a period, because sync committee changes every period.
        // this is similar to validator set change in Tendermint light client beyond trust level.

        let spec = chain
            .beacon_api_client()
            .spec()
            .await
            .map_err(Chain::raise_error)?
            .data;

        let mut trusted_sync_committee = {
            let trusted_header = chain
                .beacon_api_client()
                .header(trusted_height.revision_height.into())
                .await
                .map_err(Chain::raise_error)?
                .data;

            let trusted_bootstrap = chain
                .beacon_api_client()
                .bootstrap(trusted_header.root)
                .await
                .map_err(Chain::raise_error)?
                .data;

            let light_client_update = {
                let current_period = trusted_height.revision_height / spec.period();

                let light_client_updates = chain
                    .beacon_api_client()
                    .light_client_updates(current_period, 1)
                    .await
                    .map_err(Chain::raise_error)?;

                let [update] = <[_; 1]>::try_from(light_client_updates.0).map_err(|x| {
                    Chain::raise_error(format!("length should be 1 but got {}", x.len()))
                })?;

                if !(update.data.finalized_header.beacon.slot <= trusted_height.revision_height
                    && trusted_height.revision_height
                        < update.data.finalized_header.beacon.slot + spec.period())
                {
                    return Err(Chain::raise_error(
                        "period change update is not for the current period",
                    ));
                }

                update.data.clone()
            };

            let sync_committee = if light_client_update.finalized_header.beacon.slot
                == trusted_height.revision_height
            {
                // trusted slot with a period change
                ActiveSyncCommittee::Next(
                    SyncCommittee::try_from(SyncCommitteeProto::from(
                        light_client_update
                            .next_sync_committee
                            .ok_or_else(|| Chain::raise_error("missing finality update"))?,
                    ))
                    .map_err(Chain::raise_error)?,
                )
            } else {
                // trusted slot within an period
                ActiveSyncCommittee::Current(
                    SyncCommittee::try_from(SyncCommitteeProto::from(
                        trusted_bootstrap.current_sync_committee,
                    ))
                    .map_err(Chain::raise_error)?,
                )
            };

            TrustedSyncCommittee {
                trusted_height: *trusted_height,
                sync_committee,
            }
        };

        // we can only skip headers within a period. this is similar to validator set change in Tendermint light client.
        //
        // examples when the network period is 64:
        // 1. the trusted height is in [1, 64) and the target height is 217,
        // we perform updates for 64, 128, 192, 217
        // where 64, 128 and 192 are the period changes (with next sync committee)
        // and 217 is the target height (without next sync committee, i.e. current sync committee is trusted).
        //
        // 2. if the trusted height is in [64, 128) and the target height is 192,
        // we perform updates for 128 and 192 (with next sync committee).
        //
        // 3. if the trusted height i in [64, 128) and the target height is 120,
        // we perform update only for 120.

        let headers = {
            let trust_period = trusted_height.revision_height / spec.period();
            let target_period = target_height.revision_height / spec.period();

            let light_client_updates = if trust_period == target_period {
                // within the same period, no next sync committee changes
                vec![]
            } else {
                let updates = chain
                    .beacon_api_client()
                    .light_client_updates(trust_period + 1, target_period - trust_period)
                    .await
                    .map_err(Chain::raise_error)?
                    .0
                    .into_iter()
                    .map(|x| x.data)
                    .collect::<Vec<_>>();

                if updates.len() as u64 != (target_period - trust_period + 1) {
                    return Err(Chain::raise_error("missing light client updates"));
                }

                // there is at least one update

                let first_update_slot = updates.first().unwrap().finalized_header.beacon.slot;

                let last_update_slot = updates.last().unwrap().finalized_header.beacon.slot;

                if !(trusted_height.revision_height < first_update_slot
                    && first_update_slot <= target_height.revision_height + spec.period())
                {
                    return Err(Chain::raise_error(
                        "first update is not for the next period of trusted height",
                    ));
                }

                if !(target_height.revision_height <= last_update_slot
                    && last_update_slot < target_height.revision_height + spec.period())
                {
                    return Err(Chain::raise_error(
                        "last update is not for the previous period of target height",
                    ));
                }

                updates
            };

            let n_headers = if target_height.revision_height == last_update_slot {
                light_client_updates.len()
            } else {
                light_client_updates.len() + 1
            };

            let mut headers = Vec::with_capacity(n_headers);

            for update in light_client_updates {
                let next_sync_committee = update
                    .next_sync_committee
                    .as_ref()
                    .ok_or_else(|| {
                        Chain::raise_error("missing next sync committee after period change")
                    })?
                    .clone();

                let new_trusted_sync_committee = TrustedSyncCommittee {
                    trusted_height: Height {
                        revision_number: trusted_height.revision_number,
                        revision_height: update.finalized_header.beacon.slot,
                    },
                    sync_committee: ActiveSyncCommittee::Next(
                        SyncCommittee::try_from(SyncCommitteeProto::from(next_sync_committee))
                            .map_err(Chain::raise_error)?,
                    ),
                };

                let account_update = AccountUpdate {
                    account_proof: chain
                        .account_proof(update.finalized_header.beacon.slot, [])
                        .await?
                        .0,
                };

                let consensus_update =
                    LightClientUpdate::try_from(LightClientUpdateProto::from(update))
                        .map_err(Chain::raise_error)?;

                headers.push(Header {
                    trusted_sync_committee,
                    consensus_update,
                    account_update,
                });

                trusted_sync_committee = new_trusted_sync_committee;
            }

            if target_height.revision_height > last_update_slot {
                let new_trusted_sync_committee = TrustedSyncCommittee {
                    trusted_height: Height {
                        revision_number: trusted_height.revision_number,
                        revision_height: target_height.revision_height,
                    },
                    sync_committee: ActiveSyncCommittee::Current(
                        trusted_sync_committee.sync_committee.get().clone(),
                    ),
                };

                let unbounded_consensus_update = UnboundedLightClientUpdate {
                    attested_header: finality_update.attested_header,
                    next_sync_committee: None,
                    next_sync_committee_branch: None,
                    finalized_header: finality_update.finalized_header,
                    finality_branch: finality_update.finality_branch,
                    sync_aggregate: finality_update.sync_aggregate,
                    signature_slot: finality_update.signature_slot,
                };

                let consensus_update = LightClientUpdate::try_from(LightClientUpdateProto::from(
                    unbounded_consensus_update,
                ))
                .map_err(Chain::raise_error)?;

                let account_update = AccountUpdate {
                    account_proof: chain
                        .account_proof(target_height.revision_height, [])
                        .await?
                        .0,
                };

                headers.push(Header {
                    trusted_sync_committee,
                    consensus_update,
                    account_update,
                });

                trusted_sync_committee = new_trusted_sync_committee;
            }

            headers
        };

        Ok(EthUpdateClientPayload {
            headers,
            trusted_committee: trusted_sync_committee,
        })
    }
}
