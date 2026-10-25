use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::heroes;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = heroes)]
pub struct Hero {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn create(hero: Hero, connection: &mut MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table
            .order(heroes::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read(connection: &mut MysqlConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id).load(connection).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &mut MysqlConnection) -> bool {
        diesel::update(heroes::table.filter(heroes::id.eq(id)))
            .set(&hero)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &mut MysqlConnection) -> bool {
        diesel::delete(heroes::table.filter(heroes::id.eq(id)))
            .execute(connection)
            .is_ok()
    }
}
