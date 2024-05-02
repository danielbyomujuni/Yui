use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};




#[derive(Deserialize, Debug)]
struct Response {
    message: Message,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct ChatBody {
    model: String,
    messages: Vec<Message>,
    stream: bool
}
static mut MSG_HISTORY: Vec<Message> = Vec::new();

pub async unsafe fn set_prompt(promt: &str) -> String {

    MSG_HISTORY.push(Message {
        role: "user".to_string(),
        content: promt.to_string()
    });


    let chat_body = json!(ChatBody {
        model: "llama3".to_string(),
        messages: MSG_HISTORY.clone(),
        stream: false
    });


    let http_client = Client::new();
    let result = http_client
        .post("http://192.168.1.76:11434/api/chat")
        .json(&chat_body)
        .send().await;

    if (result.is_err()) {
        return "Critical Server Error".to_string();
    }

    let chat: Response = result.ok().unwrap().json().await.expect("REASON");//.json().await;

    let content = chat.message.content.as_str();

    MSG_HISTORY.push(Message {
        role: chat.message.role,
        content: chat.message.content.clone()
    });

    return content.to_string();
}