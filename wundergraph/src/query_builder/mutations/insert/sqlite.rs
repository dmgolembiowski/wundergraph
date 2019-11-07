use super::{HandleBatchInsert, HandleInsert};
use crate::context::WundergraphContext;
use crate::query_builder::selection::fields::WundergraphFieldList;
use crate::query_builder::selection::filter::build_filter::BuildFilter;
use crate::query_builder::selection::order::BuildOrder;
use crate::query_builder::selection::query_modifier::QueryModifier;
use crate::query_builder::selection::select::BuildSelect;
use crate::query_builder::selection::{LoadingHandler, SqlTypeOfPlaceholder};
use crate::scalar::WundergraphScalarValue;
use diesel::associations::HasTable;
use diesel::dsl::SqlTypeOf;
use diesel::expression::dsl::sql;
use diesel::query_builder::{BoxedSelectStatement, InsertStatement, QueryFragment};
use diesel::query_dsl::methods::{BoxedDsl, ExecuteDsl, LimitDsl, OrderDsl};
use diesel::sql_types::{Bool, HasSqlType};
use diesel::sqlite::Sqlite;
use diesel::{AppearsOnTable, Connection, Insertable, RunQueryDsl, Table};
use juniper::{ExecutionResult, Executor, Selection, Value};

impl<I, Ctx, L, T> HandleInsert<L, I, Sqlite, Ctx> for T
where
    T: Table + HasTable<Table = T> + 'static,
    T::FromClause: QueryFragment<Sqlite>,
    L: LoadingHandler<Sqlite, Ctx, Table = T>,
    L::Columns: BuildOrder<T, Sqlite>
        + BuildSelect<
            T,
            Sqlite,
            SqlTypeOfPlaceholder<L::FieldList, Sqlite, L::PrimaryKeyIndex, T, Ctx>,
        >,
    Ctx: WundergraphContext + QueryModifier<L, Sqlite>,
    Ctx::Connection: Connection<Backend = Sqlite>,
    L::FieldList: WundergraphFieldList<Sqlite, L::PrimaryKeyIndex, T, Ctx>,
    I: Insertable<T>,
    I::Values: QueryFragment<Sqlite>,
    InsertStatement<T, I::Values>: ExecuteDsl<Ctx::Connection>,
    T: BoxedDsl<
        'static,
        Sqlite,
        Output = BoxedSelectStatement<'static, SqlTypeOf<<T as Table>::AllColumns>, T, Sqlite>,
    >,
    <L::Filter as BuildFilter<Sqlite>>::Ret: AppearsOnTable<T>,
    Sqlite: HasSqlType<SqlTypeOfPlaceholder<L::FieldList, Sqlite, L::PrimaryKeyIndex, T, Ctx>>,
{
    fn handle_insert(
        selection: Option<&'_ [Selection<'_, WundergraphScalarValue>]>,
        executor: &Executor<'_, Ctx, WundergraphScalarValue>,
        insertable: I,
    ) -> ExecutionResult<WundergraphScalarValue> {
        let ctx = executor.context();
        let conn = ctx.get_connection();
        conn.transaction(|| -> ExecutionResult<WundergraphScalarValue> {
            let look_ahead = executor.look_ahead();
            insertable.insert_into(T::table()).execute(conn)?;
            let q = OrderDsl::order(L::build_query(&[], &look_ahead)?, sql::<Bool>("rowid DESC"));
            let q = LimitDsl::limit(q, 1);
            let items = L::load(&look_ahead, selection, executor, q)?;

            Ok(items.into_iter().next().unwrap_or(Value::Null))
        })
    }
}

impl<I, Ctx, L, T> HandleBatchInsert<L, I, Sqlite, Ctx> for T
where
    T: Table + HasTable<Table = T> + 'static,
    T::FromClause: QueryFragment<Sqlite>,
    L: LoadingHandler<Sqlite, Ctx, Table = T>,
    L::Columns: BuildOrder<T, Sqlite>
        + BuildSelect<
            T,
            Sqlite,
            SqlTypeOfPlaceholder<L::FieldList, Sqlite, L::PrimaryKeyIndex, T, Ctx>,
        >,
    Ctx: WundergraphContext + QueryModifier<L, Sqlite>,
    Ctx::Connection: Connection<Backend = Sqlite>,
    L::FieldList: WundergraphFieldList<Sqlite, L::PrimaryKeyIndex, T, Ctx>,
    I: Insertable<T>,
    I::Values: QueryFragment<Sqlite>,
    InsertStatement<T, I::Values>: ExecuteDsl<Ctx::Connection>,
    T: BoxedDsl<
        'static,
        Sqlite,
        Output = BoxedSelectStatement<'static, SqlTypeOf<<T as Table>::AllColumns>, T, Sqlite>,
    >,
    <L::Filter as BuildFilter<Sqlite>>::Ret: AppearsOnTable<T>,
    Sqlite: HasSqlType<SqlTypeOfPlaceholder<L::FieldList, Sqlite, L::PrimaryKeyIndex, T, Ctx>>,
{
    fn handle_batch_insert(
        selection: Option<&'_ [Selection<'_, WundergraphScalarValue>]>,
        executor: &Executor<'_, Ctx, WundergraphScalarValue>,
        batch: Vec<I>,
    ) -> ExecutionResult<WundergraphScalarValue> {
        let ctx = executor.context();
        let conn = ctx.get_connection();
        conn.transaction(|| -> ExecutionResult<WundergraphScalarValue> {
            let look_ahead = executor.look_ahead();
            let n: usize = batch
                .into_iter()
                .map(|i| i.insert_into(T::table()).execute(conn))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum();
            let q = OrderDsl::order(L::build_query(&[], &look_ahead)?, sql::<Bool>("rowid DESC"));
            let q = LimitDsl::limit(q, n as i64);
            let items = L::load(&look_ahead, selection, executor, q)?;
            Ok(Value::list(items.into_iter().rev().collect()))
        })
    }
}
