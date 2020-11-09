#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate diesel;

struct Foo;

pub mod test_table {
    #![allow(dead_code)]
    pub use self::columns::*;
    use diesel::associations::HasTable;
    use diesel::insertable::Insertable;
    use diesel::query_builder::nodes::Identifier;
    use diesel::query_builder::*;
    use diesel::query_source::joins::{Join, JoinOn};
    use diesel::query_source::{AppearsInFromClause, Never, Once};
    use diesel::sql_types::*;
    use diesel::{JoinTo, QuerySource, Table};
    /// Re-exports all of the columns of this table, as well as the
    /// table struct renamed to the module name. This is meant to be
    /// glob imported for functions which only deal with one table.
    pub mod dsl {
        macro_rules! __static_cond {
            (test_table test_table) => {
                compile_error!(concat!(
                    "Column `",
                    stringify!(id),
                    "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \"",
                    stringify!(id),
                    "\"]` to reference the table's `",
                    stringify!(id),
                    "` column. \n \
                            See the documentation of the `table!` macro for details`\n"
                ));
            };
            (test_table id) => {
                pub use super::columns::id;
            };
        }
        pub use super::columns::id;
        macro_rules! __static_cond {
            (test_table test_table) => {
                compile_error!(concat!(
                    "Column `",
                    stringify!(foobar),
                    "` cannot be named the same as its table.\n \
                            You may use `#[sql_name = \"",
                    stringify!(foobar),
                    "\"]` to reference the table's `",
                    stringify!(foobar),
                    "` column. \n \
                            See the documentation of the `table!` macro for details`\n"
                ));
            };
            (test_table foobar) => {
                pub use super::columns::foobar;
            };
        }
        pub use super::columns::foobar;
        pub use super::table as test_table;
    }
    #[allow(non_upper_case_globals, dead_code)]
    /// A tuple of all of the columns on this table
    pub const all_columns: (id, foobar) = (id, foobar);
    #[allow(non_camel_case_types)]
    /// The actual table struct
    ///
    /// This is the type which provides the base methods of the query
    /// builder, such as `.select` and `.filter`.
    pub struct table;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for table {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                table => {
                    let mut debug_trait_builder = f.debug_tuple("table");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for table {
        #[inline]
        fn clone(&self) -> table {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::marker::Copy for table {}
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_query_id_for_table() {
        extern crate std;
        use diesel;
        use diesel::query_builder::QueryId;
        #[allow(non_camel_case_types)]
        impl QueryId for table {
            type QueryId = table;
            const HAS_STATIC_QUERY_ID: bool = true;
        }
    }
    impl table {
        #[allow(dead_code)]
        /// Represents `table_name.*`, which is sometimes necessary
        /// for efficient count queries. It cannot be used in place of
        /// `all_columns`
        pub fn star(&self) -> star {
            star
        }
    }
    /// The SQL type of all of the columns on this table
    pub type SqlType = (Integer, Foo);
    /// Helper type for representing a boxed query from this table
    pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
    impl QuerySource for table {
        type FromClause = Identifier<'static>;
        type DefaultSelection = <Self as Table>::AllColumns;
        fn from_clause(&self) -> Self::FromClause {
            Identifier("test_table")
        }
        fn default_selection(&self) -> Self::DefaultSelection {
            Self::all_columns()
        }
    }
    impl AsQuery for table {
        type SqlType = SqlType;
        type Query = SelectStatement<Self>;
        fn as_query(self) -> Self::Query {
            SelectStatement::simple(self)
        }
    }
    impl Table for table {
        type PrimaryKey = (id);
        type AllColumns = (id, foobar);
        fn primary_key(&self) -> Self::PrimaryKey {
            (id)
        }
        fn all_columns() -> Self::AllColumns {
            (id, foobar)
        }
    }
    impl HasTable for table {
        type Table = Self;
        fn table() -> Self::Table {
            table
        }
    }
    impl IntoUpdateTarget for table {
        type WhereClause = <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
        fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
            self.as_query().into_update_target()
        }
    }
    impl AppearsInFromClause<table> for table {
        type Count = Once;
    }
    impl AppearsInFromClause<table> for () {
        type Count = Never;
    }
    impl<Left, Right, Kind> JoinTo<Join<Left, Right, Kind>> for table
    where
        Join<Left, Right, Kind>: JoinTo<table>,
    {
        type FromClause = Join<Left, Right, Kind>;
        type OnClause = <Join<Left, Right, Kind> as JoinTo<table>>::OnClause;
        fn join_target(rhs: Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
            let (_, on_clause) = Join::join_target(table);
            (rhs, on_clause)
        }
    }
    impl<Join, On> JoinTo<JoinOn<Join, On>> for table
    where
        JoinOn<Join, On>: JoinTo<table>,
    {
        type FromClause = JoinOn<Join, On>;
        type OnClause = <JoinOn<Join, On> as JoinTo<table>>::OnClause;
        fn join_target(rhs: JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
            let (_, on_clause) = JoinOn::join_target(table);
            (rhs, on_clause)
        }
    }
    impl<F, S, D, W, O, L, Of, G> JoinTo<SelectStatement<F, S, D, W, O, L, Of, G>> for table
    where
        SelectStatement<F, S, D, W, O, L, Of, G>: JoinTo<table>,
    {
        type FromClause = SelectStatement<F, S, D, W, O, L, Of, G>;
        type OnClause = <SelectStatement<F, S, D, W, O, L, Of, G> as JoinTo<table>>::OnClause;
        fn join_target(
            rhs: SelectStatement<F, S, D, W, O, L, Of, G>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, on_clause) = SelectStatement::join_target(table);
            (rhs, on_clause)
        }
    }
    impl<'a, QS, ST, DB> JoinTo<BoxedSelectStatement<'a, QS, ST, DB>> for table
    where
        BoxedSelectStatement<'a, QS, ST, DB>: JoinTo<table>,
    {
        type FromClause = BoxedSelectStatement<'a, QS, ST, DB>;
        type OnClause = <BoxedSelectStatement<'a, QS, ST, DB> as JoinTo<table>>::OnClause;
        fn join_target(
            rhs: BoxedSelectStatement<'a, QS, ST, DB>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, on_clause) = BoxedSelectStatement::join_target(table);
            (rhs, on_clause)
        }
    }
    impl<T> Insertable<T> for table
    where
        <table as AsQuery>::Query: Insertable<T>,
    {
        type Values = <<table as AsQuery>::Query as Insertable<T>>::Values;
        fn values(self) -> Self::Values {
            self.as_query().values()
        }
    }
    impl<'a, T> Insertable<T> for &'a table
    where
        table: Insertable<T>,
    {
        type Values = <table as Insertable<T>>::Values;
        fn values(self) -> Self::Values {
            (*self).values()
        }
    }
    /// Contains all of the columns of this table
    pub mod columns {
        use super::table;
        use diesel::backend::Backend;
        use diesel::query_builder::{AstPass, QueryFragment, SelectStatement};
        use diesel::query_source::joins::{Inner, Join, JoinOn, LeftOuter};
        use diesel::query_source::{AppearsInFromClause, Never, Once};
        use diesel::result::QueryResult;
        use diesel::sql_types::*;
        use diesel::{AppearsOnTable, Expression, QuerySource, SelectableExpression};
        #[allow(non_camel_case_types, dead_code)]
        /// Represents `table_name.*`, which is sometimes needed for
        /// efficient count queries. It cannot be used in place of
        /// `all_columns`, and has a `SqlType` of `()` to prevent it
        /// being used that way
        pub struct star;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for star {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    star => {
                        let mut debug_trait_builder = f.debug_tuple("star");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for star {
            #[inline]
            fn clone(&self) -> star {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for star {}
        impl Expression for star {
            type SqlType = ();
        }
        impl<DB: Backend> QueryFragment<DB> for star
        where
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".*");
                Ok(())
            }
        }
        impl SelectableExpression<table> for star {}
        impl AppearsOnTable<table> for star {}
        #[allow(non_camel_case_types, dead_code)]
        pub struct id;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for id {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    id => {
                        let mut debug_trait_builder = f.debug_tuple("id");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for id {
            #[inline]
            fn clone(&self) -> id {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for id {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_id() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for id {
                type QueryId = id;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for id {
            #[inline]
            fn default() -> id {
                id {}
            }
        }
        impl ::diesel::expression::Expression for id {
            type SqlType = Integer;
        }
        impl<DB> ::diesel::query_builder::QueryFragment<DB> for id
        where
            DB: ::diesel::backend::Backend,
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(
                &self,
                mut out: ::diesel::query_builder::AstPass<DB>,
            ) -> ::diesel::result::QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".");
                out.push_identifier("id")
            }
        }
        impl SelectableExpression<table> for id {}
        impl<QS> AppearsOnTable<QS> for id where QS: AppearsInFromClause<table, Count = Once> {}
        impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
        where
            id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
            Left: AppearsInFromClause<table, Count = Once>,
            Right: AppearsInFromClause<table, Count = Never>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
        where
            id: AppearsOnTable<Join<Left, Right, Inner>>,
            Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id where
            id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
        {
        }
        impl<From> SelectableExpression<SelectStatement<From>> for id where
            id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
        {
        }
        impl ::diesel::expression::NonAggregate for id {}
        impl ::diesel::query_source::Column for id {
            type Table = table;
            const NAME: &'static str = "id";
        }
        impl<T> ::diesel::EqAll<T> for id
        where
            T: ::diesel::expression::AsExpression<Integer>,
            ::diesel::dsl::Eq<id, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
        {
            type Output = ::diesel::dsl::Eq<Self, T>;
            fn eq_all(self, rhs: T) -> Self::Output {
                ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for id
        where
            Rhs: ::diesel::expression::AsExpression<
                <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Add>::Rhs,
            >,
        {
            type Output = ::diesel::expression::ops::Add<Self, Rhs::Expression>;
            fn add(self, rhs: Rhs) -> Self::Output {
                ::diesel::expression::ops::Add::new(self, rhs.as_expression())
            }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for id
        where
            Rhs: ::diesel::expression::AsExpression<
                <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Sub>::Rhs,
            >,
        {
            type Output = ::diesel::expression::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, rhs: Rhs) -> Self::Output {
                ::diesel::expression::ops::Sub::new(self, rhs.as_expression())
            }
        }
        impl<Rhs> ::std::ops::Div<Rhs> for id
        where
            Rhs: ::diesel::expression::AsExpression<
                <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Div>::Rhs,
            >,
        {
            type Output = ::diesel::expression::ops::Div<Self, Rhs::Expression>;
            fn div(self, rhs: Rhs) -> Self::Output {
                ::diesel::expression::ops::Div::new(self, rhs.as_expression())
            }
        }
        impl<Rhs> ::std::ops::Mul<Rhs> for id
        where
            Rhs: ::diesel::expression::AsExpression<
                <<id as ::diesel::Expression>::SqlType as ::diesel::sql_types::ops::Mul>::Rhs,
            >,
        {
            type Output = ::diesel::expression::ops::Mul<Self, Rhs::Expression>;
            fn mul(self, rhs: Rhs) -> Self::Output {
                ::diesel::expression::ops::Mul::new(self, rhs.as_expression())
            }
        }
        #[allow(non_camel_case_types, dead_code)]
        pub struct foobar;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::fmt::Debug for foobar {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    foobar => {
                        let mut debug_trait_builder = f.debug_tuple("foobar");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::clone::Clone for foobar {
            #[inline]
            fn clone(&self) -> foobar {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::marker::Copy for foobar {}
        #[allow(non_snake_case, unused_extern_crates, unused_imports)]
        fn _impl_query_id_for_foobar() {
            extern crate std;
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for foobar {
                type QueryId = foobar;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::core::default::Default for foobar {
            #[inline]
            fn default() -> foobar {
                foobar {}
            }
        }
        impl ::diesel::expression::Expression for foobar {
            type SqlType = Foo;
        }
        impl<DB> ::diesel::query_builder::QueryFragment<DB> for foobar
        where
            DB: ::diesel::backend::Backend,
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(
                &self,
                mut out: ::diesel::query_builder::AstPass<DB>,
            ) -> ::diesel::result::QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".");
                out.push_identifier("foobar")
            }
        }
        impl SelectableExpression<table> for foobar {}
        impl<QS> AppearsOnTable<QS> for foobar where QS: AppearsInFromClause<table, Count = Once> {}
        impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for foobar
        where
            foobar: AppearsOnTable<Join<Left, Right, LeftOuter>>,
            Left: AppearsInFromClause<table, Count = Once>,
            Right: AppearsInFromClause<table, Count = Never>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for foobar
        where
            foobar: AppearsOnTable<Join<Left, Right, Inner>>,
            Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Join, On> SelectableExpression<JoinOn<Join, On>> for foobar where
            foobar: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>
        {
        }
        impl<From> SelectableExpression<SelectStatement<From>> for foobar where
            foobar: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>
        {
        }
        impl ::diesel::expression::NonAggregate for foobar {}
        impl ::diesel::query_source::Column for foobar {
            type Table = table;
            const NAME: &'static str = "foobar";
        }
        impl<T> ::diesel::EqAll<T> for foobar
        where
            T: ::diesel::expression::AsExpression<Foo>,
            ::diesel::dsl::Eq<foobar, T>: ::diesel::Expression<SqlType = ::diesel::sql_types::Bool>,
        {
            type Output = ::diesel::dsl::Eq<Self, T>;
            fn eq_all(self, rhs: T) -> Self::Output {
                ::diesel::expression::operators::Eq::new(self, rhs.as_expression())
            }
        }
    }
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &match () {
                () => [],
            },
        ));
    };
}
