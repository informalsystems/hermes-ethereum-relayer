use alloy::primitives::Address;
use alloy::providers::Provider;
use beacon_api::client::BeaconApiClient;
use cgp::core::component::{UseContext, UseDelegate};
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::AddressTypeComponent;
use hermes_error::impls::ProvideHermesError;
use hermes_ethereum_chain_components::impls::account_proof::CanBuildAccountProof;
use hermes_ethereum_chain_components::traits::fields::beacon::{
    BeaconApiClientGetterComponent, HasBeaconApiClient,
};
use hermes_ethereum_chain_components::traits::fields::ibc_contract_address::IbcContractAddressGetterComponent;
use hermes_ethereum_chain_components::traits::fields::provider::AlloyProviderGetter;
use hermes_ethereum_chain_components::traits::types::beacon_preset::BeaconPresetComponent;
use unionlabs::ethereum::config::Minimal;

use crate::contexts::error::HandleEthereumChainError;

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
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleEthereumChainError>,
        AddressTypeComponent: WithType<Address>,
        BeaconPresetComponent: WithType<Minimal>,
        [
            BeaconApiClientGetterComponent,
            IbcContractAddressGetterComponent,
        ]:
            UseContext,
    }
}

impl AlloyProviderGetter<EthereumChain> for EthereumChainComponents {
    fn provider(chain: &EthereumChain) -> &dyn Provider {
        chain.provider.as_ref()
    }
}

pub trait CanUseEthereumChain: Async + HasBeaconApiClient + CanBuildAccountProof {}

impl CanUseEthereumChain for EthereumChain {}
