/*
 * Author:Jake Goodwin
 * Date: 2021
 * Description: rust basic server.
 */

//##################################################
//IMPORTS

use std::fmt;
use std::sync::{Arc, Mutex};
use slab::Slab;
use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode}; //Improt the different types.
use hyper::service::service_fn;
//use hyper::rt::Future;  //re-exported from the Future crate.
//use hyper::service::service_fn_ok;  //Import the function/method


//#################################################
//GLOBAL


//This is a shared refernce, the r# and # are for multiline rust strings.
const INDEX: &'static str = r#"
<!doctype HTML>
<html>
    <head>
        <title>Rust Microservice</title>
    </head>
    <body>
        <h3>Rust Microservice</h3>
    </body>
</html>
"#;

type UserId = u64;
struct UserData;

//Arc is a atomic reference counter provides multiple references to single data.
//In this case it's the mutex for the data it provides refs for. Mutex it a 
//atomic flag that checks that only one thread has data access.
//Slab is super fancy stuff.Slab is an allocator that can store and 
//remove any value identified by an ordered number.
type UserDb = Arc<Mutex<Slab<UserData>>>;


//#################################################
//Main code

/*
 * Input:
 * Output:
 * Description:
 */
fn main() {
    /*
     * What A server needs minimum:
     * Address to bind to.
     * server instance to handle requests.
     * a default handler for any request.
     * A reactor (runtime) where the server will operate.
     */

    println!("Rust microservice testing...");

    //make address ipv4 that we can bind to.
    //we construct a socket address from the tuple.
    let addr = ([127, 0, 0, 1], 8080).into();

    //This creates a Builder instance
    //we give bind() the reference so we reatain ownership of addr
    let builder = Server::bind(&addr);

    //This is our request handler
    //we use the builder to make a server instance
    /*
    let server = builder.serve( || {
        service_fn_ok(|_| {
            //our response to the request is a html body with contents.
            Response::new(Body::from("Rust Microservice"))
        })//no semicolen, it returns this?
    });
    */

    let server = builder.serve( || service_fn(microservice_handler));



    //Drop any error we run into.
    let server = server.map_err(drop);

    hyper::rt::run(server);


}


//#################################################
//Functions



/*
 * Input: A Request
 * Ouput: A Response
 * Description: Processes requests.
 */
//Usally we would wrap it in a Box for unknown but we use impl instead
fn microservice_handler(req: Request<Body>)
    -> impl Future<Item=Response<Body>, Error=Error>
{
    //we are using the methods of the request object.
    //Using match we can see if it's the index "page" per-say
    match(req.method(), req.uri().path()) {
        //only matches if the method is GET and the URI is index
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        },
        //otherwise everything else is a error besides root.
        _ => {
            //Here we build the response
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        },
    }
}














