use super::schema::posts;
use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost {
    #[field(validate = len(5..20))]
    pub title: String,
    #[field(validate = len(5..20))]
    pub body: String,
}

#[derive(Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: String,
    pub body: String,
}
