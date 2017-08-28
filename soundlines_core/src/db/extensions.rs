use postgres::rows::Row;
use postgres::rows::Rows;
use postgres::types::ToSql;
use postgres::types::FromSql;
use postgres::types::Type;
use postgres::types::INTERVAL;
use postgres::types::IsNull;

use std::error::Error;
use std::result::Result as StdResult;
use std::ops::Deref;

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

use std::time::Duration;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlDuration(pub Duration);

impl Deref for SqlDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const USEC_PER_SEC: i64 = 1_000_000;

impl FromSql for SqlDuration {
    fn from_sql(_: &Type, raw: &[u8]) -> StdResult<SqlDuration, Box<Error + Sync + Send>> {
        use std::io::Cursor;

        let mut cursor = Cursor::new(raw);
        let t = cursor.read_i64::<BigEndian>().expect("Failed to parse interval integer");

        Ok(SqlDuration(Duration::from_secs(t as u64 / USEC_PER_SEC as u64)))
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            INTERVAL => true,
            _ => false,
        }
    }
}

impl ToSql for SqlDuration {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> StdResult<IsNull, Box<Error + 'static + Sync + Send>> {
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            INTERVAL => true,
            _ => false,
        }
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> StdResult<IsNull, Box<Error + 'static + Sync + Send>> {
        Ok(IsNull::No)
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
    fn delete<T: SqlType>(&self, id: i32) -> Result<()>;
    fn update<T: SqlType>(&self, id: i32, &T) -> Result<()>;
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

    fn delete<T: SqlType>(&self, id: i32) -> Result<()> {
        let query = format!("delete from {} where id=$1", T::table_name());
        self.execute(&query, &[&id]).map(|_| ())
    }

    fn update<T: SqlType>(&self, id: i32, value: &T) -> Result<()> {
        let mut values = value.to_sql_array();
        let values_len = values.len();
        let mut values_str = "".to_string();
        for (i, field) in T::insert_fields().into_iter().enumerate() {
            values_str += &format!("set {}=${}{}", field, i + 1, if i == values_len -1 { "" } else { ", " });
        }

        let query = format!("update {} {} where id={}", T::table_name(), values_str, values_len + 1);
        values.push(&id);
        self.execute(&query, &values)?;
        Ok(())
    }
}
