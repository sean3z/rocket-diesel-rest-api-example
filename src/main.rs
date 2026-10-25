mod db;
mod hero;
mod schema;

use hero::Hero;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, routes};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, mut connection: db::Connection) -> Json<Hero> {
    let insert = Hero {
        id: None,
        ..hero.into_inner()
    };
    Json(Hero::create(insert, &mut connection))
}

#[get("/")]
fn read(mut connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(&mut connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, mut connection: db::Connection) -> Json<Value> {
    let update = Hero {
        id: Some(id),
        ..hero.into_inner()
    };
    Json(json!({
        "success": Hero::update(id, update, &mut connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, mut connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Hero::delete(id, &mut connection)
    }))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch()
        .await?;

    Ok(())
}
