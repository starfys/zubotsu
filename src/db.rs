#![feature(custom_attribute, proc_macro)]
// #[macro_use]
// extern crate diesel;


// extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

// mod schema {
//     infer_schema!("dotenv:DATABASE_URL");
// }

use schema::*;
use std::env;

pub fn establish_connection() -> Option<PgConnection> {
    dotenv().ok();

    match env::var("DATABASE_URL") {
        Ok(database_url) => match PgConnection::establish(&database_url) {
        Ok(conn) => Some(conn),
        None => None,
        },
        None => None,

    }
        
    
        // .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Insertable, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="users"]
struct User<'a> {
    pub name: &'a str,
    pub karma: Option<&'a i64>,
}
