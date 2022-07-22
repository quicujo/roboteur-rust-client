use reqwest::header::AUTHORIZATION;
use rust_socketio::{Payload, SocketBuilder};
use serde_json::json;
use std::time::Duration;

// FIXME: Y so much hardcode
fn main() {
    // get a socket that is connected to the reaction namespace
    let mut socket = SocketBuilder::new("https://uat.roboteur.co.uk")
        .set_namespace("/reaction")
        .expect("illegal namespace")
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .set_opening_header(AUTHORIZATION, "Bearer: xxxxx".parse().unwrap())
        .connect()
        .expect("Connection failed");

    // define a callback, that"s executed when the ack got acked
    let ack_callback = |message: Payload, _| {
        println!("Yehaa! My ack got acked?");
        println!("Ack data: {:#?}", message);
    };

    // payload as defined shape in roboteur for said serviceId + reactor
    let json_payload = json!({
        "serviceId": "a8ada11c-e1dd-4997-9f4e-7e8c51775540",
        "reactor": "Create User account API",
        "payload": {
            "about": "",
            "name": "Test",
            "lastName": "User",
            "dob": "1990-01-01",
            "age": "31",
            "gender": "Male",
            "saCitizen": "Yes",
            "password": "xxxxxx",
            "email": "test@test.com"
             }
    });

    // emit with an ack
    socket
        .emit_with_ack("create", json_payload, Duration::from_secs(2), ack_callback)
        .expect("Server unreachable");

    socket
        .disconnect()
        .expect("And we're stayin' alive, stayin' alive");
}
