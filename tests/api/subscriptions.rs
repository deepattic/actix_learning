use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=random%20noob&email=randomnoob%40tuta.io";
    let response = client
        .post(&format!("{}/subscriptions", &app_address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email,name FROM subscriptions",)
        .fetch_one(&app_address.db_pool)
        .await
        .expect("Failed to fetch saved Subs!");
    assert_eq!(saved.email, "randomnoob@tuta.io");
    assert_eq!(saved.name, "random noob");
}
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=random%20noob", "missing the email"),
        ("email=randomnoob%40tuta.io", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_return_a_400_when_fields_are_present_but_empty() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=randomnoob%40tuta.io", "empty name"),
        ("name=randomnoob&email=", "empty email"),
        ("name=randomnnoob&email=not-a-email", "invalid email"),
    ];
    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to send the request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request payload was {} .",
            description
        );
    }
}
