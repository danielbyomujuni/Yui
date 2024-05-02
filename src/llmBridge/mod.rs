use reqwest::Client;
use serde_json::json;
use serde::Deserialize;




#[derive(Deserialize, Debug)]
struct Response {
    response: String,
}


pub async fn set_prompt(promt: &str) -> String {

    let chat_body = json!({
        "model": "tinyllama",
        "prompt": promt,
        "stream": false
    });


    let http_client = Client::new();
    let result = http_client
        .post("http://192.168.1.76:11434/api/generate")
        .json(&chat_body)
        .send().await;

    if (result.is_err()) {
        return "Critical Server Error".to_string();
    }

    let chat: Response = result.ok().unwrap().json().await.expect("REASON");//.json().await;

    return chat.response;
}