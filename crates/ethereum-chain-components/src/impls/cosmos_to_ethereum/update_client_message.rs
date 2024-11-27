use cgp::prelude::*;
use hermes_chain_components::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_chain_components::traits::types::ibc::HasClientIdType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;

pub struct BuildCosmosToEthereumUpdateClientMessage;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosToEthereumUpdateClientMessage
where
    Chain: HasClientIdType<Counterparty> + HasMessageType + CanRaiseError<bincode::Error>,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let header = payload.headers.into_iter().next().unwrap();
        let encoded_header = bincode::serialize(&header).map_err(Chain::raise_error)?;

        todo!()
    }
}
