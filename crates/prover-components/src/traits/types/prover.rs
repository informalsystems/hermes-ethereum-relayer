use cgp::prelude::*;

#[derive_component(ProverTypeComponent, ProvideProverType<Context>)]
pub trait HasProverType: Async {
    type Prover: Async;
}
