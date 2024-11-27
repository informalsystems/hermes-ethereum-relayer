use alloy::primitives::Address;
use alloy::providers::Provider;
use beacon_api::client::BeaconApiClient;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_ethereum_chain_components::traits::fields::provider::AlloyProviderGetter;

#[derive(HasField)]
pub struct EthereumChain {
    pub ibc_contract_address: Address,
    pub beacon_api_client: BeaconApiClient,
    pub provider: Box<dyn Provider>,
}

pub struct EthereumChainComponents;

impl HasComponents for EthereumChain {
    type Components = EthereumChainComponents;
}

delegate_components! {
    EthereumChainComponents {
        AddressTypeComponent: WithType<Address>,
    }
}

impl AlloyProviderGetter<EthereumChain> for EthereumChainComponents {
    fn provider(chain: &EthereumChain) ->  &dyn Provider {
        chain.provider.as_ref()
    }
}
