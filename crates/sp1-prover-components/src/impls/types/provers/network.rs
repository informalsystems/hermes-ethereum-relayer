use cgp::core::Async;
use hermes_prover_components::traits::types::prover::ProvideProverType;
use sp1_sdk::NetworkProverV2;

pub struct UseNetworkProver;

impl<Context> ProvideProverType<Context> for UseNetworkProver
where
    Context: Async,
{
    type Prover = NetworkProverV2;
}
