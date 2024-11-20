use cgp::prelude::*;
use sp1_sdk::SP1ProofKind;

#[derive_component(ProofKindGetterComponent, ProofKindGetter<Context>)]
pub trait HasProofKind {
    fn proof_kind(&self) -> &SP1ProofKind;
}
