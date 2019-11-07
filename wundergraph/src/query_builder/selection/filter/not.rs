use super::BuildFilter;
use crate::juniper_ext::{NameBuilder, Nameable, FromLookAheadValue};
use crate::scalar::WundergraphScalarValue;
use diesel::backend::Backend;
use diesel::dsl;
use diesel::helper_types;
use juniper::meta::MetaType;
use juniper::{FromInputValue, GraphQLType, InputValue, LookAheadValue, Registry, ToInputValue};

/// A filter node representing a negation operation
#[derive(Debug)]
pub struct Not<I>(I);

impl<DB, I> BuildFilter<DB> for Not<I>
where
    DB: Backend,
    I: BuildFilter<DB>,
{
    type Ret = helper_types::not<I::Ret>;

    fn into_filter(self) -> Option<Self::Ret> {
        self.0.into_filter().map(dsl::not)
    }
}

impl<I> Nameable for Not<I>
where
    I: Nameable,
{
    fn name() -> String {
        format!("Not_{}", I::name())
    }
}

impl<I> FromInputValue<WundergraphScalarValue> for Not<I>
where
    I: FromInputValue<WundergraphScalarValue>,
{
    fn from_input_value(v: &InputValue<WundergraphScalarValue>) -> Option<Self> {
        I::from_input_value(v).map(Self)
    }
}

impl<I> FromLookAheadValue for Not<I>
where
    I: FromLookAheadValue,
{
    fn from_look_ahead(v: &LookAheadValue<'_, WundergraphScalarValue>) -> Option<Self> {
        I::from_look_ahead(v).map(Self)
    }
}

impl<I> ToInputValue<WundergraphScalarValue> for Not<I>
where
    I: ToInputValue<WundergraphScalarValue>,
{
    fn to_input_value(&self) -> InputValue<WundergraphScalarValue> {
        I::to_input_value(&self.0)
    }
}

impl<F> GraphQLType<WundergraphScalarValue> for Not<F>
where
    F: GraphQLType<WundergraphScalarValue>,
    F::TypeInfo: Default,
{
    type Context = F::Context;
    type TypeInfo = NameBuilder<Self>;

    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(info.name())
    }

    fn meta<'r>(
        _info: &Self::TypeInfo,
        registry: &mut Registry<'r, WundergraphScalarValue>,
    ) -> MetaType<'r, WundergraphScalarValue>
    where
        WundergraphScalarValue: 'r,
    {
        F::meta(&Default::default(), registry)
    }
}
