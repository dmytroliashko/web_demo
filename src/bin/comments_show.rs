use std::env;
use web_demo::establish_connection;
use web_demo::models::*;
use diesel::prelude::*;

fn main() {
    use web_demo::schema::comments::dsl::*;

    let args: Vec<String> = env::args().collect();

    let passed_post_id = &args[1].parse::<i32>().unwrap();

    let connection = establish_connection();
    let results = comments.filter(post_id.eq(*passed_post_id))
        .load::<Comment>(&connection)
        .expect("Failed to load comments");

    for comment in results {
        println!("{}", comment.id);
        
    }
}