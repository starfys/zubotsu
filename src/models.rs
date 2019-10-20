use crate::schema::users;

#[derive(Insertable, Associations, AsChangeset)]
#[table_name = "users"]
pub struct User<'a> {
    pub user_id: &'a i64,
    pub karma: Option<&'a i32>,
}

#[derive(Queryable)]
pub struct ReadUser {
    pub user_id: i64,
    pub karma: Option<i32>,
}
