use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/subscribe", address))
        .form(&[
            ("email", "ursula.k.leguin@gmail.com"),
            ("name", "Ursula K. Leguin"),
        ])
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = [
        (
            vec![("email", "ursula.leguin@gmail.com")],
            "missing the name",
        ),
        (vec![("name", "Ursula K. Leguin")], "missing the email"),
        (vec![], "missing both email and name"),
    ];

    for (form_data, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscribe", address))
            .form(&form_data)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST,
            "Did not fail with a 400 when the request was {}",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to run server");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
