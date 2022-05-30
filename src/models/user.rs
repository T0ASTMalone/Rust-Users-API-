use std::time::SystemTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub date_created: SystemTime,
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub date_created: SystemTime,
}


impl User {

    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(username: String, conn: &PgConnection) -> Vec<User> {
        all_users
            .filter(users::username.like(username))
            .load::<User>(conn)
            .expect("error!")
    }

}