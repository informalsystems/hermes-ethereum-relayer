use beacon_api::client::BeaconApiClient;

pub trait HasBeaconApiClient {
    fn beacon_api_client(&self) -> &BeaconApiClient;
}
