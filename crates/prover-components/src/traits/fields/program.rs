use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;

use crate::traits::types::program::HasProvableProgramType;

#[derive_component(ProvableProgramGetterComponent, ProvableProgramGetter<Context>)]
pub trait HasProvableProgram<App>: HasProvableProgramType<App> {
    fn provable_program(&self, _phantom: PhantomData<App>) -> &Self::ProvableProgram;
}

impl<Context, App, Tag> ProvableProgramGetter<Context, App> for UseField<Tag>
where
    Context: HasProvableProgramType<App>,
    Context: HasField<Tag, Field = Context::ProvableProgram>,
{
    fn provable_program(
        context: &Context,
        _phantom: PhantomData<App>,
    ) -> &Context::ProvableProgram {
        context.get_field(PhantomData)
    }
}

impl<Context, App, Components> ProvableProgramGetter<Context, App> for UseDelegate<Components>
where
    Context: HasProvableProgramType<App>,
    Components: DelegateComponent<App>,
    Components::Delegate: ProvableProgramGetter<Context, App>,
{
    fn provable_program(context: &Context, phantom: PhantomData<App>) -> &Context::ProvableProgram {
        Components::Delegate::provable_program(context, phantom)
    }
}
