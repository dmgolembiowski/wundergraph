use crate::diesel_ext::BoxableFilter;
use crate::query_builder::selection::filter::build_filter::BuildFilter;
use crate::scalar::WundergraphScalarValue;
use diesel::backend::Backend;
use diesel::expression::{operators, AsExpression, NonAggregate};
use diesel::query_builder::QueryFragment;
use diesel::serialize::ToSql;
use diesel::sql_types::{Bool, HasSqlType, Text};
use diesel::{AppearsOnTable, Column, TextExpressionMethods};
use juniper::{InputValue, ToInputValue};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Like<C>(Option<String>, ::std::marker::PhantomData<C>);

impl<C> Like<C> {
    pub(super) fn new(v: Option<String>) -> Self {
        Self(v, PhantomData)
    }
}

impl<C> Clone for Like<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<C, DB> BuildFilter<DB> for Like<C>
where
    C: TextExpressionMethods + NonAggregate + Column + QueryFragment<DB> + Default + 'static,
    String: AsExpression<C::SqlType>,
    <String as AsExpression<C::SqlType>>::Expression:
        NonAggregate + AppearsOnTable<C::Table> + QueryFragment<DB> + 'static,
    DB: Backend + HasSqlType<Text> + 'static,
    String: ToSql<Text, DB>,
    C::Table: 'static,
    operators::Like<C, <String as AsExpression<C::SqlType>>::Expression>:
        AppearsOnTable<C::Table, SqlType = Bool>,
{
    type Ret = Box<dyn BoxableFilter<C::Table, DB, SqlType = Bool>>;

    fn into_filter(self) -> Option<Self::Ret> {
        let Self(filter, _) = self;
        filter.map(|v| Box::new(C::default().like(v)) as Box<_>)
    }
}

impl<C> ToInputValue<WundergraphScalarValue> for Like<C> {
    fn to_input_value(&self) -> InputValue<WundergraphScalarValue> {
        self.0.to_input_value()
    }
}
