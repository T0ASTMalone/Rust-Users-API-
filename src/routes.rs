
use rocket_contrib::json::Json;
use serde_json::Value;
use super::db::Conn as DbConn;
use super::models::{User, NewUser};

#[get("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/new-user", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[get("/get-user?<username>")]
pub fn find_user(conn: DbConn, username: String) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(username, &conn),
    }))
}
