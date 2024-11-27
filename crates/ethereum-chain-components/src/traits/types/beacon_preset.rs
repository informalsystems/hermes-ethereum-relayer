use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(BeaconPresetComponent, ProvideBeaconPreset<Chain>)]
pub trait HasBeaconPreset {
    type BeaconPreset;
}

impl<Chain, Provider, Preset> ProvideBeaconPreset<Chain> for WithProvider<Provider>
where
    Provider: ProvideType<Chain, BeaconPresetComponent, Type = Preset>,
{
    type BeaconPreset = Preset;
}
