use super::models::*;
use diesel::pg::upsert::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::mem;

// TODO: rewrite as DAO instead of separate functions?

pub fn establish_connection() -> Result<PgConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_error| Error::new(ErrorKind::InvalidData, "DATABASE_URL must be set"))?;
    PgConnection::establish(&database_url).map_err(|_error| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Error connecting to {}", database_url),
        )
    })
}

// postgresql integer types are kind of messy
//  Name 	            Storage Size 	Description 	                Range
// smallint             2 bytes 	small-range integer 	            -32768 to +32767
// integer 	            4 bytes 	typical choice for integer 	        -2147483648 to +2147483647
// bigint 	            8 bytes 	large-range integer 	            -9223372036854775808 to 9223372036854775807
// decimal 	            variable 	user-specified precision, exact 	no limit
// numeric 	            variable 	user-specified precision, exact 	no limit
// real 	            4 bytes 	variable-precision, inexact 	    6 decimal digits precision
// double precision 	8 bytes 	variable-precision, inexact 	    15 decimal digits precision
// serial 	            4 bytes 	autoincrementing integer 	        1 to 2147483647
// bigserial 	        8 bytes 	large autoincrementing integer 	    1 to 9223372036854775807
// out of these our options were either
// - bigint  this fits the right size (64 bits), but is signed, so some jankyness has to be handled
// - numeric this might fit the right signing, but not sure how rust handles arbitrary buffers
// - bigserial this might fit both but might have weird odities with updates
// eh at the end of the day this should be okay
pub fn upsert_user_karma(pgconn: &PgConnection, user_id: u64, karma: i32) -> Result<(), Error> {
    use super::schema::users;

    let unsafe_user_id: i64 = unsafe { mem::transmute(user_id) };
    let user = super::models::User {
        user_id: &unsafe_user_id,
        karma: Some(&karma),
    };

    let result = diesel::insert_into(users::table)
        .values(&user)
        .on_conflict(on_constraint("users_pkey"))
        .do_update()
        .set(users::karma.eq(users::karma + karma))
        .execute(pgconn);
    match result.err() {
        Some(e) => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Error reading data {}", e),
        )),
        None => Ok(()),
    }
}

pub fn leaderboards(pgconn: &PgConnection) -> Result<Vec<ReadUser>, Error> {
    use super::schema::users::dsl::*;
    match users.order(karma.desc()).limit(10).load::<ReadUser>(pgconn) {
        Ok(users_result) => Ok(users_result),
        Err(e) => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Error reading data {}", e),
        )),
    }
}

pub fn get_karma_for_id(pgconn: &PgConnection, discord_user_id: u64) -> Result<i32, Error> {
    use super::schema::users::dsl::*;
    let unsafe_user_id: i64 = unsafe { mem::transmute(discord_user_id) };
    match users
        .filter(user_id.eq(unsafe_user_id))
        .limit(1)
        .load::<ReadUser>(pgconn)
    {
        Ok(user_result) if user_result.len() == 1 => Ok(match user_result[0].karma {
            Some(karma_count) => karma_count,
            None => 0,
        }),
        Err(e) => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Error reading data {}", e),
        )),
        _ => Ok(0),
    }
}
