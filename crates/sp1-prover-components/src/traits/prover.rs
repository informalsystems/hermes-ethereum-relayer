use sp1_prover::components::DefaultProverComponents;
use sp1_sdk::Prover;

pub trait Sp1Prover: Prover<DefaultProverComponents> {}

impl<P> Sp1Prover for P where P: Prover<DefaultProverComponents> {}
