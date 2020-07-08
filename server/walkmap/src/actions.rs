use diesel_migrations::embed_migrations;

embed_migrations!("migrations/");

use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

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

// Keep the databse info in mind to drop them later
struct TestContext {
    base_url: String,
    db_name: String,
}

impl TestContext {
    fn new(base_url: &str, db_name: &str) -> Self {
        // First, connect to postgres db to be able to create our test
        // database.
        let postgres_url = format!("{}/postgres", base_url);
        let conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

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
        let conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

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
    let _ctx = TestContext::new("postgresql://localhost:5432", "walkmaptest");

    let conn = PgConnection::establish(&"postgresql://localhost:5432/walkmaptest").unwrap();

    // Now do your test.

    let user = insert_new_user("bill", &conn).unwrap();
    assert_eq!(user.name, "bill");

    let found_user = find_user_by_uid(Uuid::parse_str(&user.id).unwrap(), &conn)
        .unwrap()
        .unwrap();
    assert_eq!(found_user.name, "bill");
}
