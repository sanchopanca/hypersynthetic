#![cfg(feature = "rocket")]

use hypersynthetic::prelude::*;
use rocket::{
    get,
    http::{ContentType, Status},
    local::blocking::Client,
    routes,
};

#[get("/")]
fn index() -> HtmlFragment {
    html! { <body><h1>"Hello, world!"</h1></body> }
}

#[test]
fn test_rocket_responder() {
    let client = Client::tracked(rocket::build().mount("/", routes![index])).unwrap();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(
        response.into_string().as_deref(),
        Some("<body><h1>Hello, world!</h1></body>")
    );
}
