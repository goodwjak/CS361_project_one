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
extern crate serde_json;

use rocket::*;
use std::io;
use serde_json::{Result, Value};

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
    let xy_vals = parse_mouse_clicks(input);
    //let xy_parsed = parse_json_post(input, &v);
    //let heatmap_visual = make_heatmap(xy_vals);
    format!("{}, heatmap is being generated...", pagename);
    println!("Heatmap input: {}", input);
    println!("xy_vals: {}", xy_vals.len());
    //println!("xy_parsed: {}", xy_parsed["x_vals"]);
    "TESTING"
}




//#######################################
//Functions
//#######################################


/*
 * Input: json of mouse xy values when clicked.
 * Output: those xy values in a array or list.
 * Description: converts the post data into usable data structs.
 */
fn parse_mouse_clicks(json_data: &str) -> Vec::<Coordinate> {
   
    println!("input: {}", json_data);
    //Try to parse the json.
    let res = serde_json::from_str(json_data);
    
    println!("ret: {:?}", res);

    let mut v: Vec::<Coordinate> = Vec::new();
    let p: Value = res.unwrap();
    let x_vals = p["x_vals"].as_array().unwrap();
    let y_vals = p["y_vals"].as_array().unwrap();

    let arr_len = x_vals.len(); 

    for i in 0..arr_len {
        let mut click_point = Coordinate {
        x: x_vals.get(i).unwrap().as_u64().unwrap(),
        y: y_vals.get(i).unwrap().as_u64().unwrap(),
        };
        v.push(click_point);
    }
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










