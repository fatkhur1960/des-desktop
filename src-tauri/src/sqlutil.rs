use diesel::{sql_query, sql_types, sql_types::Timestamp};

sql_function!(
    /// To lowerize sql column value typely
    fn lower(x: sql_types::Text) -> sql_types::Text);

sql_function! (
    fn strftime(x: sql_types::Text, y: sql_types::Timestamp) -> sql_types::Text
);
