/*
 * Author: Jake Goodwin
 * Date: 2021
 * Description: My microservice to get click data and provide a heatmap.
 */


//This is a old way of using the crate.
//#[macro_use] extern crate rocket;

//new way, use all the stuff from rocket crate.
use rocket::*;

//This is just the sample one.
#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}


/*
 * Input: a get request with a pagename.
 * Output: a string response.
 * Description: A placeholder function that does a little.
 */
#[get("/heatmap/<pagename>")]
fn heatmap(pagename: &str) -> String {
    format!("{}, heatmap is being generated...", pagename)
}


//You have to launch the rocket to get anything done.
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![heatmap])
}










