use cgp::core::Async;
use hermes_prover_components::traits::types::prover::ProvideProverType;
use sp1_sdk::MockProver;

pub struct UseMockProver;

impl<Context> ProvideProverType<Context> for UseMockProver
where
    Context: Async,
{
    type Prover = MockProver;
}
