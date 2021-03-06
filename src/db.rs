use crate::models::{ReadUser, User};
use crate::schema;
use diesel::pg::upsert::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// TODO: rewrite as DAO instead of separate functions?

pub fn establish_connection(database_url: &str) -> Result<PgConnection, diesel::ConnectionError> {
    PgConnection::establish(&database_url)
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
pub fn upsert_user_karma(
    pgconn: &PgConnection,
    user_id: u64,
    karma: i32,
) -> Result<(), diesel::result::Error> {
    use schema::users;

    // this is technically unsafe transform but due to knowledge about the id system of discord
    // we can ignore this for now (until 2084)
    let user_id = user_id as i64;
    let user = User {
        user_id: &user_id,
        karma: Some(&karma),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .on_conflict(on_constraint("users_pkey"))
        .do_update()
        .set(users::karma.eq(users::karma + karma))
        .execute(pgconn)
        .map(|_| ())
}

pub fn leaderboards(pgconn: &PgConnection) -> Result<Vec<ReadUser>, diesel::result::Error> {
    use schema::users::dsl::*;
    users.order(karma.desc()).limit(10).load::<ReadUser>(pgconn)
}

pub fn get_karma_for_id(
    pgconn: &PgConnection,
    discord_user_id: u64,
) -> Result<i32, diesel::result::Error> {
    use schema::users::dsl::*;
    // this is technically unsafe transform but due to knowledge about the id system of discord
    // we can ignore this for now (until 2084)
    let discord_user_id = discord_user_id as i64;
    users
        .filter(user_id.eq(discord_user_id))
        .limit(1)
        .load::<ReadUser>(pgconn)
        .map(|user_result| {
            if user_result.len() == 1 {
                user_result[0].karma.unwrap_or(0)
            } else {
                0
            }
        })
}
