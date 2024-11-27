use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use unionlabs::ethereum::config::{
    BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES, SYNC_COMMITTEE_SIZE,
};

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
        + HasHeightType
        + HasBeaconPreset<BeaconPreset = Preset>
        + HasErrorType,
    Preset: SYNC_COMMITTEE_SIZE + BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES,
{
    async fn build_update_client_payload(
        _chain: &Chain,
        _trusted_height: &Chain::Height,
        _target_height: &Chain::Height,
        _client_state: Chain::ClientState,
    ) -> Result<EthUpdateClientPayload<Preset>, Chain::Error> {
        todo!()
    }
}
