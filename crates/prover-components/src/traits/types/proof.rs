use cgp::prelude::*;

#[derive_component(ProofTypeComponent, ProvideProofType<Context>)]
pub trait HasProofType<App>: Async {
    type Proof: Async;
}
