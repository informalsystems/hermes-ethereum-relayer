use cgp::core::Async;
use hermes_prover_components::traits::types::prover::ProvideProverType;
use sp1_sdk::CpuProver;

pub struct UseCpuProver;

impl<Context> ProvideProverType<Context> for UseCpuProver
where
    Context: Async,
{
    type Prover = CpuProver;
}
