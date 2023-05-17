use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::State;
use rocket::fairing::AdHoc;


#[macro_use] extern crate rocket;

// In Rocket, configurations are Managed States, which you can retrieve via &State<T>. 
// Here, we are trying to use the Config values in a GET endpoint by using &State<Config>.

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

// THE GETS

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!(
        "Hello, {}! You are {} years old.", config.name, config.age
    )
}

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

// THE POSTS

#[post("/", data = "<blog_post>")]
fn create_blog_post(blog_post: Json<BlogPost>) -> Json<BlogPost> {
    blog_post
}


#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, get_config])
        .mount("/blog-posts", routes![get_random_blog_post, get_blog_post, get_blog_posts, create_blog_post])
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
