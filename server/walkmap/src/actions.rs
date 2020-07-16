use diesel_migrations::embed_migrations;

embed_migrations!("migrations/");

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
use std::time::SystemTime;
use uuid::Uuid;

use crate::models;

#[derive(Debug)]
pub struct Letters;

impl Distribution<char> for Letters {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const RANGE: u32 = 26;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        // We can pick from 62 characters. This is so close to a power of 2, 64,
        // that we can do better than `Uniform`. Use a simple bitshift and
        // rejection sampling. We do not use a bitmask, because for small RNGs
        // the most significant bits are usually of higher quality.
        loop {
            let var = rng.gen_range(0, 26);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize] as char;
            }
        }
    }
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn insert_new_marker(
    map_id_: &str,
    order_parameter_: f64,
    lat_: f64,
    lon_: f64,
    conn: &PgConnection,
) -> Result<models::Marker, diesel::result::Error> {
    use crate::schema::markers::dsl::*;

    let marker = models::Marker {
        id: Uuid::new_v4().to_string(),
        map_id: map_id_.to_owned(),
        order_parameter: order_parameter_,
        lat: lat_,
        lon: lon_,
        annotation: None,
        image_url: None,
    };

    diesel::insert_into(markers).values(&marker).execute(conn)?;

    Ok(marker)
}

pub fn insert_new_map(conn: &PgConnection) -> Result<models::Map, diesel::result::Error> {
    use crate::schema::maps::dsl::*;
    let rand_string: String = thread_rng()
        .sample_iter(&Letters)
        .take(5)
        .collect::<String>()
        .to_uppercase();
    let new_map = models::Map {
        id: rand_string,
        user_id: None,
        created_at: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not find system time")
            .as_secs() as i64,
    };

    diesel::insert_into(maps).values(&new_map).execute(conn)?;

    Ok(new_map)
}

#[cfg(test)]
mod test {
    // Keep the databse info in mind to drop them later
    use super::*;
    struct TestContext {
        base_url: String,
        db_name: String,
    }

    impl TestContext {
        fn new(base_url: &str, db_name: &str) -> Self {
            // First, connect to postgres db to be able to create our test
            // database.
            let postgres_url = format!("{}/postgres", base_url);
            let conn = PgConnection::establish(&postgres_url)
                .expect("Cannot connect to postgres database.");

            // Create a new database for the test
            let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
            query
                .execute(&conn)
                .expect(format!("Could not create database {}", db_name).as_str());

            let conn = PgConnection::establish(&format!("{}/{}", base_url, db_name))
                .expect(&format!("Cannot connect to {} database", db_name));
            let result = embedded_migrations::run(&conn);

            if let Err(e) = result {
                panic!("Failed to run migrations. Error: {}", e);
            }

            Self {
                base_url: base_url.to_string(),
                db_name: db_name.to_string(),
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            let postgres_url = format!("{}/postgres", self.base_url);
            let conn = PgConnection::establish(&postgres_url)
                .expect("Cannot connect to postgres database.");

            let disconnect_users = format!(
                "SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '{}';",
                self.db_name
            );

            diesel::sql_query(disconnect_users.as_str())
                .execute(&conn)
                .unwrap();

            let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
            query
                .execute(&conn)
                .expect(&format!("Couldn't drop database {}", self.db_name));
        }
    }

    #[test]
    fn insert_user_test() {
        let _ctx = TestContext::new("postgresql://localhost:5432", "walkmaptesttwo");

        let conn = PgConnection::establish(&"postgresql://localhost:5432/walkmaptesttwo").unwrap();

        let user = insert_new_user("bill", &conn).unwrap();
        assert_eq!(user.name, "bill");

        let found_user = find_user_by_uid(Uuid::parse_str(&user.id).unwrap(), &conn)
            .unwrap()
            .unwrap();
        assert_eq!(found_user.name, "bill");
    }

    #[test]
    fn insert_map_marker_test() {
        let _ctx = TestContext::new("postgresql://localhost:5432", "walkmaptest");

        let conn = PgConnection::establish(&"postgresql://localhost:5432/walkmaptest").unwrap();

        let map = insert_new_map(&conn).expect("Could not insert new map");
        println!("{:?}", map);

        let marker = insert_new_marker(&map.id, 100.0, 1.0, 1.0, &conn);
        println!("{:?}", marker);
    }
}
