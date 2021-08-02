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


//#######################################
//GLOBAL
//#######################################

//A xy coordinate struct just to package them together.
struct coordinate {
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
 * Input: a get request with a pagename.
 * Output: a string response.
 * Description: A placeholder function that does a little.
 */
#[get("/heatmap/<pagename>")]
fn heatmap(pagename: &str) -> String {
    format!("{}, heatmap is being generated...", pagename)
}


/*
 * Input: a bunch of mouse data.
 * Output: a heatmap file.
 * Description: handles heatmap requests.
 */
#[post("/heatmap/<pagename>",format="application/json", data="<input>")]
fn get_heatmap_data(pagename: &str) -> String {
    let xy_vals = parse_mouse_clicks(input);
    let heatmap_visual = make_heatmap(xy_vals);
    format!("{}, heatmap is being generated...", pagename)
    println!("Heatmap input: {}", input);
}




//#######################################
//Functions
//#######################################


/*
 * Input: json of mouse xy values when clicked.
 * Output: those xy values in a array or list.
 * Description: converts the post data into usable data structs.
 */
fn parse_mouse_clicks(json_data: str) {
    //create vector of coordinates.
    let v Vec::<coordinate> = Vec::new();

    //TODO: add parsing to this function.

    //A struct we will reuse in a loop to "map" the data.
    let mut click_point;

    //TODO: Add loop to this.
    click_point.x = 0;
    click_point.y = 0;

    v.push((0, 0)<coordinate>)

    //return the vector and give ownership to the calling scope.
    v
}


/*
 * Input: A vector of coordinats
 * Output: a visual of those coordinates.
 * Description: Makes a visual heatmap of the coodinates.
 */
fn make_heatmap(click_points: Vec::<coordinate>) {
    //TODO: Find a crate for making visual data.

    let heatmap = 1
}


//#######################################
//MAIN CODE
//#######################################



//You have to launch the rocket to get anything done.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/heatmap", routes![get_heatmap_data])
}










