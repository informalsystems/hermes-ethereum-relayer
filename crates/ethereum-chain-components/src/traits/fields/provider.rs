use alloy::providers::Provider;
use cgp::prelude::*;

#[derive_component(AlloyProviderGetterComponent, AlloyProviderGetter<Chain>)]
pub trait HasAlloyProvider {
    fn provider(&self) -> &dyn Provider;
}
