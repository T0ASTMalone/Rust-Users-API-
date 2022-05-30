
use rocket_contrib::json::Json;
use serde_json::Value;
use crate::db::Conn as DbConn;
use crate::models::user::{User, NewUser};

#[get("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/users/create", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[get("/users/<id>")]
pub fn find_user_by_id(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_id(id, &conn),
    }))
}

#[put("/users/<id>", format = "application/json", data = "<updated_user>")]
pub fn update_user(conn: DbConn, id: i32, updated_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::update_user(id, updated_user.into_inner(), &conn),
    }))
}

#[delete("/users/<id>")]
pub fn delete_user(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::delete_user(id, &conn),
    }))
}

#[get("/users/filter?<username>", rank = 2)]
pub fn find_user_by_name(conn: DbConn, username: String) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(username, &conn),
    }))
}
