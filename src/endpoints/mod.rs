use self::schema::posts::dsl;
use crate::models;
use crate::schema;
use crate::schema::posts::id;
use crate::services;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::Post;
use models::{NewPost, UpdatePost};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[get("/")]
pub async fn get_posts() -> Json<Vec<Post>> {
    match dsl::posts.load::<Post>(&mut services::establish_connection_pg()) {
        Ok(response) => return Json(response),
        Err(err) => panic!("Database error - {}", err),
    }
}

#[post("/", format = "json", data = "<post_input>")]
pub async fn create_post(post_input: Json<NewPost>) {
    match diesel::insert_into(dsl::posts)
        .values(&post_input.into_inner())
        .execute(&mut services::establish_connection_pg())
    {
        Ok(_) => return,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

#[get("/<post_id>")]
pub async fn get_post(post_id: i32) -> Json<Post> {
    match dsl::posts
        .find(post_id)
        .first(&mut services::establish_connection_pg())
    {
        Ok(res) => return Json(res),
        Err(err) => panic!("Database error - {}", err),
    }
}

#[delete("/<post_id>")]
pub async fn delete_post(post_id: i32) {
    match diesel::delete(dsl::posts.filter(id.eq(post_id)))
        .execute(&mut services::establish_connection_pg())
    {
        Ok(_) => return,
        Err(err) => panic!("Database error - {}", err),
    }
}

#[put("/<post_id>", format = "json", data = "<post_input>")]
pub async fn update_post(post_id: i32, post_input: Json<UpdatePost>) {
    match diesel::update(dsl::posts.find(post_id))
        .set(post_input.into_inner())
        .execute(&mut services::establish_connection_pg())
    {
        Ok(_) => return,
        Err(err) => panic!("Database error - {}", err),
    }
}
