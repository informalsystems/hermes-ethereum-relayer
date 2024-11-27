use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

#[derive_component(IbcContractAddressGetterComponent, IbcContractAddressGetter<Chain>)]
pub trait HasIbcContractAddress: HasAddressType {
    fn ibc_contract_address(&self) -> &Self::Address;
}

impl<Chain> IbcContractAddressGetter<Chain> for UseContext
where
    Chain: HasAddressType + HasField<symbol!("ibc_contract_address"), Field = Chain::Address>,
{
    fn ibc_contract_address(chain: &Chain) -> &Chain::Address {
        chain.get_field(PhantomData)
    }
}
