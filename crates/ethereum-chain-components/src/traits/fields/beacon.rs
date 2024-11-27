use core::marker::PhantomData;

use beacon_api::client::BeaconApiClient;
use cgp::core::component::UseContext;
use cgp::prelude::*;

#[derive_component(BeaconApiClientGetterComponent, BeaconApiClientGetter<Chain>)]
pub trait HasBeaconApiClient {
    fn beacon_api_client(&self) -> &BeaconApiClient;
}

impl<Chain> BeaconApiClientGetter<Chain> for UseContext
where
    Chain: HasField<symbol!("beacon_api_client"), Field = BeaconApiClient>,
{
    fn beacon_api_client(chain: &Chain) -> &BeaconApiClient {
        chain.get_field(PhantomData)
    }
}
