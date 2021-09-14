use crate::schemas::post::post;
use crate::schemas::post::post::dsl::*;
use crate::utils::date_serializer;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Deserialize, serde::Serialize, Clone, Insertable, AsChangeset, JsonSchema, Debug,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "post"]
pub struct Post {
    pub t_id: i32,
    pub title: String,
    pub message: String,
    pub views: i32,
    #[serde(with = "date_serializer")]
    #[schemars(with = "String")]
    pub crated_on: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Insertable, Deserialize, JsonSchema, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[table_name = "post"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub author_id: i32,
}

#[derive(Insertable, Deserialize, serde::Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
#[table_name = "post"]
pub struct UpdatePost<'a> {
    pub title: &'a str,
    pub message: &'a str,
}

impl Post {
    pub fn create(conn: &PgConnection, new_post: &NewPost) -> Post {
        diesel::insert_into(post)
            .values(new_post)
            .get_result(conn)
            .expect("Error saving new post")
    }

    pub fn read(conn: &PgConnection, id: i32) -> Post {
        post.find(id).first(conn).expect("Error loading post")
    }

    pub fn update(conn: &PgConnection, id: i32, update_post: &UpdatePost) -> Post {
        let mut post_updated: Post = post.find(id).first(conn).expect("Error loading post");
        post_updated.title = update_post.title.to_string();
        post_updated.message = update_post.message.to_string();

        diesel::update(post.find(id))
            .set(&post_updated)
            .get_result(conn)
            .expect("Error updating post")
    }

    pub fn delete(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(post.find(id)).execute(conn).is_ok()
    }

    pub fn read_all(conn: &PgConnection) -> Vec<Post> {
        post.load::<Post>(conn).expect("Error loading posts")
    }

    pub fn read_all_by_author(conn: &PgConnection, id: i32) -> Vec<Post> {
        post.filter(author_id.eq(id))
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
}
