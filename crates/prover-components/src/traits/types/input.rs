use cgp::prelude::*;

#[derive_component(ProgramInputTypeComponent, ProvideProgramInputType<Context>)]
pub trait HasProgramInputType<App>: Async {
    type ProgramInput: Async;
}
