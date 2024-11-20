use cgp::core::Async;
use hermes_prover_components::traits::types::proof::ProvideProofType;
use sp1_sdk::SP1ProofWithPublicValues;

pub struct UseSp1Proof;

impl<Context: Async, App> ProvideProofType<Context, App> for UseSp1Proof {
    type Proof = SP1ProofWithPublicValues;
}
