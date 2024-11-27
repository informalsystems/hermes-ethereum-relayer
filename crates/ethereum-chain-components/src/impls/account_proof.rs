use alloy::primitives::Address;
use alloy::transports::{RpcError, TransportErrorKind};
use beacon_api::client::BlockId;
use beacon_api::errors::Error as BeaconError;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use ics008_wasm_client::MerklePath;
use unionlabs::ethereum::{ibc_commitment_key_v2, IBC_HANDLER_COMMITMENTS_SLOT};
use unionlabs::ibc::lightclients::ethereum::account_proof::AccountProof;
use unionlabs::ibc::lightclients::ethereum::storage_proof::StorageProof;
use unionlabs::uint::U256;

use crate::traits::fields::beacon::HasBeaconApiClient;
use crate::traits::fields::ibc_contract_address::HasIbcContractAddress;
use crate::traits::fields::provider::HasAlloyProvider;

#[async_trait]
pub trait CanBuildAccountProof: HasErrorType {
    async fn account_proof<const N: usize>(
        &self,
        slot: u64,
        merkle_paths: [MerklePath; N],
    ) -> Result<(AccountProof, [StorageProof; N]), Self::Error>;
}

#[derive(Debug)]
pub struct EmptyMerkleKeyPath;

impl<Chain> CanBuildAccountProof for Chain
where
    Chain: Async
        + HasAddressType<Address = Address>
        + HasBeaconApiClient
        + HasAlloyProvider
        + HasIbcContractAddress
        + CanRaiseError<BeaconError>
        + CanRaiseError<RpcError<TransportErrorKind>>
        + CanRaiseError<EmptyMerkleKeyPath>
        + CanRaiseError<&'static str>
        + CanRaiseError<String>
        ,
{
    async fn account_proof<const N: usize>(
        &self,
        slot: u64,
        merkle_paths: [MerklePath; N],
    ) -> Result<(AccountProof, [StorageProof; N]), Self::Error> {
        let beacon = self.beacon_api_client();
        let provider = self.provider();
        let ibc_contract_address = self.ibc_contract_address();

        let execution_height = beacon
            .execution_height(BlockId::Slot(slot))
            .await
            .map_err(Chain::raise_error)?;

        let keys = merkle_paths
            .into_iter()
            .map(|merkle_path| {
                let key_path = merkle_path
                    .key_path
                    .into_iter()
                    // take only the first key_path from a merkle path
                    // https://github.com/gjermundgaraba/union/blob/10355e6/light-clients/ethereum-light-client/src/client.rs#L87
                    .next()
                    .ok_or_else(|| Chain::raise_error(EmptyMerkleKeyPath))?;

                let key = ibc_commitment_key_v2(key_path.into(), IBC_HANDLER_COMMITMENTS_SLOT)
                    .to_be_bytes()
                    .into();

                Ok(key)
            })
            .collect::<Result<_, Chain::Error>>()?;

        let response = provider
            .get_proof(ibc_contract_address, keys)
            .block_id(execution_height.into())
            .await
            .map_err(Chain::raise_error)?;

        let account_proof = AccountProof {
            storage_root: response.storage_hash.into(),
            proof: response
                .account_proof
                .into_iter()
                .map(|x| x.to_vec())
                .collect(),
        };

        let proofs = <[_; N]>::try_from(response.storage_proof)
            .map_err(|x| Chain::raise_error(format!("length should be {} but got {}", N, x.len())))?;

        let storage_proofs = proofs
            .map(|proof| StorageProof {
                key: U256::from_be_bytes(proof.key.as_b256().0),
                value: U256::from_limbs(proof.value.into_limbs()),
                proof: proof
                    .proof
                    .into_iter()
                    .map(|bytes| bytes.to_vec())
                    .collect(),
            });

        Ok((account_proof, storage_proofs))
    }
}
