use isahc::{http::StatusCode, prelude::*, HttpClient};
use tokio::time::{sleep, Duration};

use crate::model::CombinationResult;
use url::form_urlencoded;

pub async fn build_http_client() -> Result<HttpClient, isahc::Error> {
    HttpClient::builder()
        .title_case_headers(true)
        .default_header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
        )
        .default_header("Accept", "*/*")
        .default_header("Accept-Language", "en-US,en;q=0.5")
        .default_header("Referer", "https://neal.fun/infinite-craft/")
        .default_header("Connection", "keep-alive")
        .default_header("Referrer-Policy", "strict-origin-when-cross-origin")
        .default_header("Sec-Fetch-Dest", "empty")
        .default_header("Sec-Fetch-Mode", "cors")
        .default_header("Sec-Fetch-Site", "same-origin")
        .default_header("Sec-GPC", "1")
        .build()
}
pub async fn request_combination<'a>(
    ingredients: &'a [String],
    client: &'a HttpClient,
) -> Result<CombinationResult, &'a str> {
    sleep(Duration::from_millis(900)).await;

    let (first, second) = {
        if ingredients.first().is_none() {
            return Err("missing First ingredient");
        };

        if ingredients.get(1).is_none() {
            return Err("missing Second ingredient");
        };

        (
            form_urlencoded::byte_serialize(ingredients[0].as_bytes()).collect::<String>(),
            form_urlencoded::byte_serialize(ingredients[1].as_bytes()).collect::<String>(),
        )
    };

    let url = format!(
        "https://neal.fun/api/infinite-craft/pair?first={first}&second={second}",
        first = first,
        second = second
    );

    let mut response = match client.get(url) {
        Ok(response) => response,
        Err(why) => {
            eprintln!("{why:?}");
            return Err("Error APIIIII {}");
        }
    };
    match response.status() {
        StatusCode::OK => match response.json::<CombinationResult>() {
            Ok(result) => Ok(result),
            Err(_) => Err("Error parsing combination result"),
        },
        _ => Err("API DIED WITH CODE {status}!!!! EXITING IMMEDIATELY!!!!"),
    }
}
