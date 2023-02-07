/* mod common;
use common::init_app;
use reqwest::{Client, Response, cookie::Cookie, header::COOKIE};


//TODO: Decifrar como funciona os cookies com reqwest
#[tokio::test]
async fn lobby_check_endpoint_get_return_200_with_cookie(){
    let address:String = init_app().await.address;
    let client:Client= reqwest::Client::new();

    let response:Response= client.get(&format!("{}/lobby", address))
        .header(COOKIE, "uuid=b3872ca8-5428-4579-a412-0f6f7dc0b37e")
        .send()
        .await
        .expect("Test failed");

    let cookie = response.cookies().into_iter().collect::<Vec<Cookie>>();
    println!("{:?}", cookie);

    assert_eq!(200, response.status().as_u16());
   // assert_eq!(response.headers().get("content-type").unwrap(), "text/html; charset=utf-8");
}*/
