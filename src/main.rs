#![feature(plugin, const_mut_refs, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use dotenv::dotenv;
// use std::process::Command;

mod db;
mod models;
mod routes;
mod schema;

fn rocket () -> rocket::Rocket {
    dotenv().ok();

    let database_url  = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![
                routes::users::get_all, 
                routes::users::new_user, 
                routes::users::find_user_by_id,
                routes::users::find_user_by_name,
                routes::users::update_user,
                routes::users::delete_user,
            ],
        )
}

fn main() {
    rocket().launch();
}
