use cgp::core::Async;
use hermes_prover_components::traits::types::program::ProvideProvableProgramType;

pub struct UseProvableElfBytes;

impl<Context: Async, App> ProvideProvableProgramType<Context, App> for UseProvableElfBytes {
    type ProvableProgram = Vec<u8>;
}
