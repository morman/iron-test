extern crate hyper;

use hyper::*;

#[test]
fn get_test() {
    let client = Client::new();

    let res = client.get("http://localhost:3000/test").send().unwrap();

    assert_eq!(res.status, hyper::Ok);
    assert_eq!(res.body, "This is the test route");
}

#[test]
fn get_not_test() {
    let client = Client::new();

    let res = client.get("http://localhost:3000/not_test").send().unwrap();

    assert_eq!(res.status, hyper::Ok);
    assert_eq!(res.body, "Your route is: not_test");
}
