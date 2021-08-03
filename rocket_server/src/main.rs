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
//Actually they explain why they used the other way on the page.
//URL:  https://rocket.rs/v0.5-rc/guide/overview/
use rocket::*;
use std::io;
//use serde::{Serialize, Deserialize};
//use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
//#[macro_use] extern crate serde;
use serde_json::{Result, Value};
use plotters::prelude::*;

//#######################################
//GLOBAL
//#######################################

//A xy coordinate struct just to package them together.
struct Coordinate {
    x: u64,
    y: u64
}



//#######################################
//Request Handlers
//#######################################



/*
 * Input: a get request handler
 * Ouput: a static string.
 * Description: Basic does nothing cool.
 */
#[get("/")] // <- Route attribute
fn index() -> &'static str {    // <- request handler
    "Hello world"
}


/*
 * Input: a bunch of mouse data.
 * Output: a heatmap file.
 * Description: handles heatmap requests.
 */
#[post("/heatmap/<pagename>",format="application/json", data="<input>")]
fn get_heatmap_data(pagename: &str, input: &str) -> &'static str {
    //let xy_vals = parse_mouse_clicks(input);
    let mut v: Vec::<Coordinate> = Vec::new();

    let xy_parsed = parse_json_post(input, &v);
    //let heatmap_visual = make_heatmap(xy_vals);
    format!("{}, heatmap is being generated...", pagename);
    println!("Heatmap input: {}", input);
    println!("xy_parsed: {}", xy_parsed["x_vals"]);
    "TEST"
}




//#######################################
//Functions
//#######################################


fn parse_json_post(json_data: &str, v: &Vec::<Coordinate>) -> Result<()> {
     //make sede from json string.
    let val: Value = serde_json::from_str(json_data)?;
    println!("val: {}", val);

    println!("x0: {}", val[0]);



    Ok(())
}


/*
 * Input: json of mouse xy values when clicked.
 * Output: those xy values in a array or list.
 * Description: converts the post data into usable data structs.
 */
fn parse_mouse_clicks(json_data: &str) -> Vec::<Coordinate> {
    //create vector of coordinates.
    let mut v: Vec::<Coordinate> = Vec::new();

    //let example_json: &str = "[776,776,788,788,812,812,798,798],[236,236,267,267,267,267,235,235]";
    
    let data = parse_json_post(json_data, &v);

    //A struct we will reuse in a loop to "map" the data.
    let mut click_point = Coordinate {
        x: 0,
        y: 0,
    };

    //

    //set to zero.
    click_point.x = 0;
    click_point.y = 0;

    v.push(click_point);

    //return the vector and give ownership to the calling scope.
    v
}


/*
 * Input: A vector of coordinats
 * Output: a visual of those coordinates.
 * Description: Makes a visual heatmap of the coodinates.
 */
//TODO: change return type to a file or something.
fn make_heatmap(click_points: Vec::<Coordinate>) ->u64 {
    //TODO: Find a crate for making visual data.

    let heatmap = 1;
    heatmap
}


//#######################################
//MAIN CODE
//#######################################



//You have to launch the rocket to get anything done.
#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index])
        .mount("/", routes![get_heatmap_data])
}










