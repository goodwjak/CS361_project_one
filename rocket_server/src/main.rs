/*
 * Author: Jake Goodwin
 * Date: 2021
 * Description: My microservice to get click data and provide a heatmap.
 */

//So what does lifecycle with rocket look like?
//1: Routing -> parse and operate.
//2: Validation -> Try is and if it fails then off to next branch
//3: Processing -> request handler invoked returns response.
//4: Response -> rocket sends out the response.


//This is a old way of using the crate.
//#[macro_use] extern crate rocket;

//new way, use all the stuff from rocket crate.
use rocket::*;

/*
 * Input: a get request handler
 * Ouput: a static string.
 * Description: Basic does nothing cool.
 */
#[get("/")]
fn index() -> &'static str {
    "Hello world"
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
    rocket::build().mount("/", routes![index])
}










