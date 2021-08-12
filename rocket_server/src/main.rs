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

//new way, use all the stuff from rocket crate.
//Actually they explain why they used the other way on the page.
//URL:  https://rocket.rs/v0.5-rc/guide/overview/
extern crate serde_json;

use rocket::*;
use std::io;
use serde_json::{Result, Value};
use svg::Document;
use svg::node::element::Path as svg_path;
use svg::node::element::path::Data;
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, relative};



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
 *
#[post("/heatmap/<pagename>",format="application/json", data="<input>")]
fn get_heatmap_data(pagename: &str, input: &str) -> &'static str {
    let xy_vals = parse_mouse_clicks(input);
    let heatmap_doc = paint_heatmap(xy_vals);
    format!("{}, heatmap is being generated...", pagename);
    println!("Heatmap input: {}", input);
    "FILE"
}
*/

#[post("/heatmap/file/<pagename>",format="application/json", data="<input>")]
pub async fn get_heatmap_data(pagename: &str, input: &str) -> Option<NamedFile> {
    println!("input: {:?}", input);
    let xy_vals = parse_mouse_clicks(input);
    let screen_size = parse_screen_size(input);
    paint_heatmap(xy_vals, screen_size.x, screen_size.y);
    NamedFile::open("heatmap.svg").await.ok() 
}




//#######################################
//Functions
//#######################################


/*
 * Input: a resolusion, color, size
 * Output: a node for a svg image.
 * Description: returns a NODE for adding to svg.
 */
fn paint_dot(x: u64, y: u64, dot_size: u64) -> svg::node::element::Path{
    let data = Data::new()
        .move_to((x, y))
        .line_by((0, dot_size))
        .close();

    let path = svg_path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("opacity", 0.25)
        .set("stroke-width", dot_size)
        .set("d", data);

    path
}

/*
 * Input: x_vals, y_vals, dot size, svg::document
 * Output: svg::document
 * Description: adds dots to the document without knowing number ahead of time.
 */
fn add_dots(mut xy_vals: Vec::<Coordinate>, size: u64, doc: svg::Document) -> svg::Document {
    let a = xy_vals.pop().unwrap();
    let x: u64 = a.x;
    let y: u64 = a.y;
    let temp_doc = doc
        .clone()
        .add(paint_dot(x, y, size));
    if xy_vals.len() > 0 {
        let rec_doc = add_dots(xy_vals, size, temp_doc);
        return rec_doc;
    } 
    else {
        return temp_doc;
    }
}

/*
 * Input:
 * Output:
 * Description:
 */
fn paint_heatmap(xy_vals: Vec::<Coordinate>, width: u64, height: u64) {
    let document = Document::new()
        .set("viewBox", (0, 0,width, height));

    let heatmap_doc = add_dots(xy_vals, 25, document);
    svg::save("heatmap.svg", &heatmap_doc).unwrap();
}


/*
 * Input: json of mouse xy values when clicked.
 * Output: those xy values in a array or list.
 * Description: converts the post data into usable data structs.
 */
fn parse_mouse_clicks(json_data: &str) -> Vec::<Coordinate> {
    //Try to parse the json.
    let res = serde_json::from_str(json_data);
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
 * Input: json string
 * Ouput: Coordinate
 * Description: gets the screen size from json
 */
fn parse_screen_size(json_data: &str) -> Coordinate {
    let res = serde_json::from_str(json_data);
    let p: Value = res.unwrap();
    let screen_size = Coordinate {
    x: p["x_res"].as_u64().unwrap(),
    y: p["y_res"].as_u64().unwrap()
    };
    screen_size
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










