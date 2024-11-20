use cgp::prelude::*;

#[derive_component(ProvableProgramTypeComponent, ProvideProvableProgramType<Context>)]
pub trait HasProvableProgramType<App>: Async {
    type ProvableProgram: Async;
}
