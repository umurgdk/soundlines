use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::types::ToSql;

use super::Result;
use super::Connection;

pub trait SafeRowAccess {
    fn try_get<'a>(&'a self, idx: usize) -> Option<Row<'a>>;
}

impl SafeRowAccess for Rows {
    fn try_get<'a>(&'a self, idx: usize) -> Option<Row<'a>> {
        if idx < self.len() {
            return Some(self.get(idx));
        }

        None
    }
}

pub trait LastByDate { } 

pub trait SqlType {
    fn table_name() -> &'static str;
    fn from_sql_row<'a>(row: Row<'a>) -> Self;
    fn to_sql_array<'a>(&'a self) -> Vec<&'a ToSql>;
    fn insert_fields() -> Vec<&'static str>;
}

pub trait QueryExtensions {
    fn all<T: SqlType>(&self) -> Result<Vec<T>>;
    fn last<T: SqlType>(&self) -> Result<Option<T>>;
    fn insert<T: SqlType>(&self, value: &T) -> Result<T>;
}

impl QueryExtensions for Connection {
    fn all<T: SqlType>(&self) -> Result<Vec<T>> {
        self.query(&format!("select * from {}", T::table_name()), &[])
            .map(|rows| rows.into_iter().map(T::from_sql_row).collect())
    }

    fn last<T: SqlType>(&self) -> Result<Option<T>> {
        self.query(&format!("select * from {} order by created_at desc", T::table_name()), &[])
            .map(|rows| rows.try_get(0).map(T::from_sql_row))
    }

    fn insert<T: SqlType>(&self, value: &T) -> Result<T> {
        let arr = value.to_sql_array();
        let mut values_str = String::with_capacity(arr.len() * 2);
        for i in 0..arr.len() {
            values_str += &format!("${}{}", i + 1, if i == arr.len() -1 { "" } else { "," });
        }

        let field_names = T::insert_fields().join(",");

        let query = format!("insert into {} ({}) values ({}) returning *", T::table_name(), field_names, values_str);
        println!("SQL QUERY:\n{}", query);

        self.query(&query, &arr)
            .map(|rows| T::from_sql_row(rows.get(0)))
    }
}
