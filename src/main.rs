mod api;

use api::*;
use chrono::Local;

const MXP_CHANNEL_ID: &str = "2f0d4646-3d8b-4c5e-ab78-4d3249d27daf";

#[tokio::main]
async fn main() {
    let customer_name = "Endro";
    let whatsapp_number = "82244604125";

    let namevalue = customer_name.to_uppercase();
    let message = namevalue.clone();
    let finalnumber = format!("62{}", whatsapp_number);
    let template_msg = "ca41ea37-1005-41a2-8fe8-46bc3991e016";

    let language = Language {
        code: "id".to_string(),
    };

    let body_message = vec![BodyMessage {
        key: "1".to_string(),
        value: "message".to_string(),
        value_text: message,
    }];

    let parameters = Parameters { body: body_message };

    let data_insert = DataInsert {
        to_number: finalnumber,
        to_name: customer_name.to_string(),
        message_template_id: template_msg.to_string(),
        channel_integration_id: MXP_CHANNEL_ID.to_string(),
        language,
        parameters,
        execute_type: "immediately".to_string(),
        send_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    match broadcast_message(&data_insert).await {
        Ok(response) => println!("Status: {}, Message: {}", response.status, response.msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
