use diesel::mysql::MysqlConnection;
use uuid::Uuid;

use crate::models::models::User;

use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension};

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &MysqlConnection,
) -> Result<Option<User>, diesel::result::Error> {
    // use crate::diesel_custom::users::dsl::*;

    use crate::models::schema::users::{id, table};
    // use crate::diesel_custom::schema::users;

    let user = table
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &MysqlConnection,
) -> Result<User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    // use crate::diesel_custom::users::dsl::*;
    use crate::models::schema::users::table;

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    println!("New user {}", nm);

    diesel::insert_into(table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}