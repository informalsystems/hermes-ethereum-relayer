use cgp::prelude::*;

use crate::traits::types::input::HasProgramInputType;
use crate::traits::types::program::HasProvableProgramType;
use crate::traits::types::proof::HasProofType;
use crate::traits::types::prover::HasProverType;

#[derive_component(ProofGeneratorComponent, ProofGenerator<Context>)]
#[async_trait]
pub trait CanGenerateProof<App>:
    HasProverType
    + HasProvableProgramType<App>
    + HasProgramInputType<App>
    + HasProofType<App>
    + HasErrorType
{
    async fn generate_proof(
        &self,
        prover: &Self::Prover,
        program: &Self::ProvableProgram,
        input: Self::ProgramInput,
    ) -> Result<Self::Proof, Self::Error>;
}
