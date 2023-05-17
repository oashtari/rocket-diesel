use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(
        BlogPost{
            id: 1,
            title: "My first post".to_string(),
            body: "This is my first post".to_string(),
            published: true,
        }
    )
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(
        BlogPost {
            id,
            title: "Some title".to_string(),
            body: "Some body".to_string(),
            published: true,
        }
    )
}

#[get("/")]
fn get_blog_posts() -> Json<Vec<BlogPost>> {
    Json(vec![
        BlogPost {
            id: 0,
            title: "Harry Potter".to_string(),
            body: "There once lived a boy".to_string(),
            published: true,
        },
        BlogPost {
            id: 1,
            title: "Fantastic Beast".to_string(),
            body: "There once lived a beast".to_string(),
            published: true,
        }
    ])
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .mount("/", routes![index])
        .mount("/blog-posts", routes![get_random_blog_post, get_blog_post, get_blog_posts])
}

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

// #[derive(Serialize, Deserialize)]
// struct BlogPost {
//     id: i32, 
//     title: String,
//     body: String,
//     published: bool,
// }
