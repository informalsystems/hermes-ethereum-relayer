use cgp::prelude::CanRaiseError;
use hermes_prover_components::traits::generate_proof::ProofGenerator;
use hermes_prover_components::traits::types::input::HasProgramInputType;
use hermes_prover_components::traits::types::program::HasProvableProgramType;
use hermes_prover_components::traits::types::proof::HasProofType;
use hermes_prover_components::traits::types::prover::HasProverType;
use sp1_sdk::provers::ProofOpts;
use sp1_sdk::{CpuProver, Prover, SP1ContextBuilder, SP1ProofWithPublicValues, SP1Stdin};

use crate::traits::fields::proof_kind::HasProofKind;

pub struct GenerateSp1ProofWithCpu;

impl<Context, App> ProofGenerator<Context, App> for GenerateSp1ProofWithCpu
where
    Context: HasProverType<Prover = CpuProver>
        + HasProvableProgramType<App, ProvableProgram = Vec<u8>>
        + HasProgramInputType<App, ProgramInput = Vec<Vec<u8>>>
        + HasProofType<App, Proof = SP1ProofWithPublicValues>
        + HasProofKind
        + CanRaiseError<anyhow::Error>,
{
    async fn generate_proof(
        context: &Context,
        prover: &CpuProver,
        program: &Vec<u8>,
        inputs: Vec<Vec<u8>>,
    ) -> Result<Context::Proof, Context::Error> {
        let proof_kind = context.proof_kind();

        let (proving_key, _) = prover.setup(program);

        let mut stdin = SP1Stdin::new();

        for input in inputs {
            stdin.write_vec(input);
        }

        let options = ProofOpts::default();

        let mut context_builder = SP1ContextBuilder::default();

        let prove_context = context_builder.build();

        let proof = prover
            .prove(&proving_key, stdin, options, prove_context, *proof_kind)
            .map_err(Context::raise_error)?;

        Ok(proof)
    }
}
