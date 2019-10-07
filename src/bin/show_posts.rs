extern crate web_demo;
extern crate diesel;

use self::web_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use web_demo::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    println!("{:?}", results);

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}