use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::heroes;

#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

#[derive(Queryable)]
struct HeroRow {
    id: i32,
    name: String,
    identity: String,
    hometown: String,
    age: i32,
}

#[derive(Insertable)]
#[diesel(table_name = heroes)]
struct NewHero {
    name: String,
    identity: String,
    hometown: String,
    age: i32,
}

impl From<HeroRow> for Hero {
    fn from(row: HeroRow) -> Self {
        Hero {
            id: Some(row.id),
            name: row.name,
            identity: row.identity,
            hometown: row.hometown,
            age: row.age,
        }
    }
}

impl Hero {
    pub fn create(hero: Hero, connection: &mut MysqlConnection) -> Hero {
        let new_hero = NewHero {
            name: hero.name,
            identity: hero.identity,
            hometown: hero.hometown,
            age: hero.age,
        };

        diesel::insert_into(heroes::table)
            .values(&new_hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table
            .order(heroes::id.desc())
            .first::<HeroRow>(connection)
            .unwrap()
            .into()
    }

    pub fn read(connection: &mut MysqlConnection) -> Vec<Hero> {
        heroes::table
            .order(heroes::id)
            .load::<HeroRow>(connection)
            .unwrap()
            .into_iter()
            .map(Hero::from)
            .collect()
    }

    pub fn update(id: i32, hero: Hero, connection: &mut MysqlConnection) -> bool {
        diesel::update(heroes::table.find(id))
            .set((
                heroes::name.eq(hero.name),
                heroes::identity.eq(hero.identity),
                heroes::hometown.eq(hero.hometown),
                heroes::age.eq(hero.age),
            ))
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &mut MysqlConnection) -> bool {
        diesel::delete(heroes::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
