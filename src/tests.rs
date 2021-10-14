use rocket::local::blocking::Client;
use rocket::http::{Status, ContentType, Accept};
use rocket::serde::{Serialize, Deserialize, uuid::Uuid};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    id: Option<usize>,
    message: String
}

impl Message {
    fn new(message: impl Into<String>) -> Self {
        Message { message: message.into(), id: None }
    }

    fn with_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }
}

#[test]
fn json_bad_get_put() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Try to get a message with an ID that doesn't exist.
    let res = client.get("/json/99").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.into_string().unwrap();
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found."));

    // Try to get a message with an invalid ID.
    let res = client.get("/json/hi").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::NotFound);
    assert!(res.into_string().unwrap().contains("error"));

    // Try to put a message without a proper body.
    let res = client.put("/json/80").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::BadRequest);

    // Try to put a message with a semantically invalid body.
    let res = client.put("/json/0")
        .header(ContentType::JSON)
        .body(r#"{ "dogs?": "love'em!" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::UnprocessableEntity);

    // Try to put a message for an ID that doesn't exist.
    let res = client.put("/json/80")
        .json(&Message::new("hi"))
        .dispatch();

    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn json_post_get_put_get() {
    let client = Client::tracked(super::rocket()).unwrap();

    // Create/read/update/read a few messages.
    for id in 0..10 {
        let uri = format!("/json/{}", id);

        // Check that a message with doesn't exist.
        let res = client.get(&uri).header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::NotFound);

        // Add a new message. This should be ID 0.
        let message = Message::new(format!("Hello, JSON {}!", id));
        let res = client.post("/json").json(&message).dispatch();
        assert_eq!(res.status(), Status::Ok);

        // Check that the message exists with the correct contents.
        let res = client.get(&uri).header(Accept::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.into_json::<Message>().unwrap(), message.with_id(id));

        // Change the message contents.
        let message = Message::new("Bye bye, world!");
        let res = client.put(&uri).json(&message).dispatch();
        assert_eq!(res.status(), Status::Ok);

        // Check that the message exists with the updated contents.
        let res = client.get(&uri).header(Accept::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.into_json::<Message>().unwrap(), message.with_id(id));
    }
}
