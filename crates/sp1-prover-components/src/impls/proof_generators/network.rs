use cgp::prelude::CanRaiseError;
use hermes_prover_components::traits::generate_proof::ProofGenerator;
use hermes_prover_components::traits::types::input::HasProgramInputType;
use hermes_prover_components::traits::types::program::HasProvableProgramType;
use hermes_prover_components::traits::types::proof::HasProofType;
use hermes_prover_components::traits::types::prover::HasProverType;
use sp1_sdk::{NetworkProverV2, SP1ProofWithPublicValues, SP1Stdin};

use crate::traits::fields::proof_mode::HasProofMode;

pub struct GenerateSp1NetworkProof;

impl<Context, App> ProofGenerator<Context, App> for GenerateSp1NetworkProof
where
    Context: HasProverType<Prover = NetworkProverV2>
        + HasProvableProgramType<App, ProvableProgram = Vec<u8>>
        + HasProgramInputType<App, ProgramInput = Vec<Vec<u8>>>
        + HasProofType<App, Proof = SP1ProofWithPublicValues>
        + HasProofMode
        + CanRaiseError<anyhow::Error>,
{
    async fn generate_proof(
        context: &Context,
        prover: &NetworkProverV2,
        program: &Vec<u8>,
        inputs: Vec<Vec<u8>>,
    ) -> Result<Context::Proof, Context::Error> {
        let proof_mode = context.proof_mode();

        let mut stdin = SP1Stdin::new();

        for input in inputs {
            stdin.write_vec(input);
        }

        prover
            .prove(program, stdin, proof_mode.clone(), None)
            .await
            .map_err(Context::raise_error)?;

        todo!()
    }
}
