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
            .filter(users::username.ilike(format!("%{}%", username)))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn get_user_by_id(id: i32, conn: &PgConnection) -> Vec<User> {
        all_users
            .filter(users::id.eq(id))
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn update_user(id: i32, updated_user: NewUser,  conn: &PgConnection) -> Vec<User> {
        diesel::update(all_users.filter(users::id.eq(id)))
            .set((
                users::email.eq(updated_user.email), 
                users::username.eq(updated_user.username)
            ))
            .get_results::<User>(conn)
            .expect("error!")
    }

    pub fn delete_user(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_users.filter(users::id.eq(id)))
            .execute(conn)
            .is_ok()
    }

}
