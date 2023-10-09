#[cfg(feature = "rocket")]
#[test]
fn test_rocket_responder() {
    use hypersynthetic::prelude::*;

    use rocket::{
        http::{ContentType, Status},
        local::blocking::Client,
        response::Responder,
    };

    let rocket = rocket::build();
    let client = Client::tracked(rocket).unwrap();
    let req = client.get("/");

    let response = html!(<body><h1>"Hello, world!"</h1></body>).respond_to(&req);

    assert!(response.is_ok());

    let response = response.unwrap();

    assert_eq!(response.status(), Status::Ok);

    assert_eq!(response.content_type(), Some(ContentType::HTML));

    // not checking the body because it's an async function and it's annoying
}
