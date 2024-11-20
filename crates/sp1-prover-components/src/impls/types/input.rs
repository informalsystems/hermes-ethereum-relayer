use cgp::core::Async;
use hermes_prover_components::traits::types::input::ProvideProgramInputType;

pub struct UseSp1Input;

impl<Context: Async, App> ProvideProgramInputType<Context, App> for UseSp1Input {
    type ProgramInput = Vec<Vec<u8>>;
}
