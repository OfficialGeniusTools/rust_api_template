use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::posts;
use super::schema::posts::dsl::posts as all_posts;

use std::option::Option;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}


impl Post {
    pub fn all(conn: &PgConnection) -> Vec<Post> {
        all_posts.order(posts::id.desc()).load::<Post>(conn).unwrap()
    }

    pub fn get(id: i64, conn: &PgConnection) -> Option<Post> {
        all_posts.find(id).get_result::<Post>(conn).optional().unwrap()
    }

    pub fn insert(conn: &PgConnection, new_post: &NewPost) -> bool {
        diesel::insert_into(posts::table)
            .values(new_post)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(conn: &PgConnection, id: i64) -> bool {
        diesel::delete(posts::table.find(id)).execute(conn).is_ok()
    }

    pub fn update(conn: &PgConnection, id: i64, new_post: &NewPost) -> bool {
        diesel::update(posts::table.find(id))
            .set(new_post)
            .execute(conn).unwrap_or(0) > 0
    }
}