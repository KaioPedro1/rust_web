/*mod common;
use common::init_app;
use reqwest::{Client, Response};

#[tokio::test]
async fn root_check_endpoint_get_return_200_or_304(){
    let address:String = init_app().await.address;
    let client:Client= reqwest::Client::new();
    let response:Response= client.get(&format!("{}/", address))
        .send()
        .await
        .expect("Test failed");
    assert!(response.status().is_success());
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html; charset=utf-8");
}
#[tokio::test]
async fn root_check_valid_endpoint_post_return_200(){
    let app = init_app().await;
    let client:Client= reqwest::Client::new();
    let body="name=cleber";

    let response:Response= client.post(&format!("{}/", app.address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Test failed");

    assert_eq!(200, response.status().as_u16());
    assert_eq!("/lobby", response.headers().get("location").unwrap());
}

#[tokio::test]
async fn root_check_invalid_endpoint_post_return_400(){
    let app = init_app().await;
    let client:Client= reqwest::Client::new();
    let test_cases:Vec<(&str, &str)> = vec![
        ("","empty name"),
        ("name=%20"," name only with spaces"),
        ("{\"name\":\"banana\"","Invalid Payload")
    ];
        for (invalid_body, error_message) in test_cases{
            let response = client
                .post(&format!("{}/", app.address))
                .header("Content-Type","application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Test failed");

                assert_eq!(400,
                response.status().as_u16(),
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message);
            }
} */
