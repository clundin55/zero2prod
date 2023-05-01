use axum::extract::Form;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(user_data: Form<FormData>) {
    println!("Received user_data: {:?}", user_data);
}
