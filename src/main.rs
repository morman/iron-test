// bring in iron and its URL router 
extern crate iron;
extern crate router;

// use specific parts 
use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {	
	
    let mut router = Router::new();  // Alternative syntax:
    router.get("/", handler);        // let router = router!(get "/" => handler,
    router.get("/:query", handler);  //                      get "/:query" => handler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        if *query == "test" {
			Ok(Response::with((status::Ok, "This is the test route."))) 
        } else {
        	Ok(Response::with((status::Ok, "Your route is: ".to_string() + *query)))             	
        }
    }
}

