//! Additional SQL definitions and utilities.

use diesel::define_sql_function;
use diesel::sql_types::Timestamptz;

define_sql_function!(
    /// Represents the `AGE` SQL function.
    #[sql_name = "AGE"]
    fn age(end: Timestamptz, start: Timestamptz) -> Interval
);
