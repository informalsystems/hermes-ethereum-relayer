use hermes_chain_type_components::traits::types::address::HasAddressType;

pub trait HasIbcContractAddress: HasAddressType {
    fn ibc_contract_address(&self) -> Self::Address;
}
