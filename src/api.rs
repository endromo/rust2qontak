use reqwest::Client;
use serde::{Deserialize, Serialize};

const MXP_ACCESS_TOKEN: &str = "ZDK_eQaAmiwWy4FZSf8vsr5Euc26jVpOuPMY_qY_1Mo";

#[derive(Debug, Serialize)]
pub struct Language {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct BodyMessage {
    pub key: String,
    pub value: String,
    pub value_text: String,
}

#[derive(Debug, Serialize)]
pub struct Parameters {
    pub body: Vec<BodyMessage>,
}

#[derive(Debug, Serialize)]
pub struct DataInsert {
    pub to_number: String,
    pub to_name: String,
    pub message_template_id: String,
    pub channel_integration_id: String,
    pub language: Language,
    pub parameters: Parameters,
    pub execute_type: String,
    pub send_at: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    error: Option<ApiError>,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    messages: String,
}

#[derive(Debug)]
pub struct ResponseData {
    pub status: u16,
    pub msg: String,
}

pub async fn broadcast_message(
    data_insert: &DataInsert,
) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("https://service-chat.qontak.com/api/open/v1/broadcasts/whatsapp/direct")
        .bearer_auth(MXP_ACCESS_TOKEN)
        .json(&data_insert)
        .send()
        .await?;

    let body: ApiResponse = res.json().await?;

    let data = if body.status != "error" {
        ResponseData {
            status: 200,
            msg: "success".to_string(),
        }
    } else {
        ResponseData {
            status: 400,
            msg: body
                .error
                .map_or("Unknown error".to_string(), |e| e.messages),
        }
    };

    Ok(data)
}
