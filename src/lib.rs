#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate diesel_codegen;

#[cfg(test)]
mod tests;

mod types {
    use diesel::pg::{Pg, PgMetadataLookup, PgTypeMetadata};
    use diesel::types::HasSqlType;

    #[derive(Clone, Copy)]
    pub struct Ltree;

    impl HasSqlType<Ltree> for Pg {
        fn metadata(_: &PgMetadataLookup) -> PgTypeMetadata {
            PgTypeMetadata {
                oid: 24754,
                array_oid: 24757,
            }
        }
    }

    impl_query_id!(Ltree);
}

mod functions {
    use types::*;
    use diesel::types::*;

    sql_function!(subltree, subltree_t, (ltree: Ltree, start: Int4, end: Int4) -> Ltree);
    sql_function!(subpath, subpath_t, (ltree: Ltree, offset: Int4, len: Int4) -> Ltree);
    // there's a subpath without a len argument; not sure sql_function! can do iter
    // i guess i could separate them by module
    sql_function!(nlevel, nlevel_t, (ltree: Ltree) -> Int4);
    sql_function!(index, index_t, (a: Ltree, b: Ltree) -> Int4);
    // TODO: index with offset
    sql_function!(text2ltree, text2ltree_t, (text: Text) -> Ltree);
    sql_function!(ltree2text, ltree2text_t, (ltree: Ltree) -> Text);
}

mod dsl {
    use types::*;
    use diesel::expression::{AsExpression, Expression};

    mod predicates {
        use diesel::pg::Pg;

        diesel_infix_operator!(Contains, " @> ", backend: Pg);
        diesel_infix_operator!(ContainedBy, " <@ ", backend: Pg);
        diesel_infix_operator!(Eq, " = ", backend: Pg);
        diesel_infix_operator!(NotEq, " <> ", backend: Pg);
        diesel_infix_operator!(Gt, " > ", backend: Pg);
        diesel_infix_operator!(GtEq, " >= ", backend: Pg);
        diesel_infix_operator!(Lt, " < ", backend: Pg);
        diesel_infix_operator!(LtEq, " <= ", backend: Pg);
    }

    use self::predicates::*;

    pub trait LtreeExtensions: Expression<SqlType = Ltree> + Sized {
        fn contains<T: AsExpression<Ltree>>(self, other: T) -> Contains<Self, T::Expression> {
            Contains::new(self, other.as_expression())
        }

        fn contained_by<T: AsExpression<Ltree>>(
            self,
            other: T,
        ) -> ContainedBy<Self, T::Expression> {
            ContainedBy::new(self, other.as_expression())
        }

        fn eq<T: AsExpression<Ltree>>(self, other: T) -> Eq<Self, T::Expression> {
            Eq::new(self, other.as_expression())
        }

        fn ne<T: AsExpression<Ltree>>(self, other: T) -> NotEq<Self, T::Expression> {
            NotEq::new(self, other.as_expression())
        }

        fn gt<T: AsExpression<Ltree>>(self, other: T) -> Gt<Self, T::Expression> {
            Gt::new(self, other.as_expression())
        }

        fn ge<T: AsExpression<Ltree>>(self, other: T) -> GtEq<Self, T::Expression> {
            GtEq::new(self, other.as_expression())
        }

        fn lt<T: AsExpression<Ltree>>(self, other: T) -> Lt<Self, T::Expression> {
            Lt::new(self, other.as_expression())
        }

        fn le<T: AsExpression<Ltree>>(self, other: T) -> LtEq<Self, T::Expression> {
            LtEq::new(self, other.as_expression())
        }
    }

    impl<T: Expression<SqlType = Ltree>> LtreeExtensions for T {}
}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;
