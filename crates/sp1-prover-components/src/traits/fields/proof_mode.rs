use cgp::prelude::*;
use sp1_sdk::network_v2::proto::network::ProofMode;

#[derive_component(ProofModeGetterComponent, ProofModeGetter<Context>)]
pub trait HasProofMode {
    fn proof_mode(&self) -> &ProofMode;
}
