#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

mod hero;
use hero::{Hero};

mod db;
mod schema;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let hero = hero.into_inner();

    let insert = Hero {
        id: None,
        name: hero.name,
        identity: hero.identity,
        hometown: hero.hometown,
        age: hero.age
    };

    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
    let hero = hero.into_inner();

    let update = Hero {
        id: Some(id),
        name: hero.name,
        identity: hero.identity,
        hometown: hero.hometown,
        age: hero.age
    };
    
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
