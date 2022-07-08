use crate::{
    config::ServerConfig,
    services::{email::EmailService, telegram::TelegramService},
};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use axum_macros::debug_handler;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct SendNotificationBody {
    user_id: String,
    subject: String,
    message: String,
}
#[debug_handler]
async fn handle_send_notification(
    Json(payload): Json<SendNotificationBody>,
    Extension(state): Extension<ServerConfig>,
) -> impl IntoResponse {
    let channels = sqlx::query!("SELECT * from channels WHERE user_id= $1;", payload.user_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();
    let mut ok = true;
    for channel in channels {
        println!("{:?}", channel);
        if let Some(service) = state.get_service(channel.service_id.clone()) {
            service
                .send(
                    channel.service_username.clone(),
                    payload.subject.clone(),
                    payload.message.clone(),
                )
                .await;
        } else {
            println!("could not send message to service {}", channel.service_id);
            ok = false
        }
    }
    (StatusCode::OK, Json(ok))
}

async fn add_channel(
    config: ServerConfig,
    user_id: String,
    service_id: String,
    service_username: String,
) -> Result<(), String> {
    sqlx::query!(
        "INSERT INTO channels (user_id,service_id, service_username) VALUES ($1,$2,$3)",
        user_id,
        service_id,
        service_username
    )
    .execute(&config.pool)
    .await
    .unwrap();
    Ok(())
}

#[derive(Debug, Deserialize)]
struct RemoveChannelBody {
    user_id: String,
    service_id: String,
}

#[debug_handler]
async fn remove_channel(
    Extension(config): Extension<ServerConfig>,
    Json(payload): Json<RemoveChannelBody>,
) -> Result<(), String> {
    sqlx::query!(
        "DELETE FROM channels WHERE user_id= $1 AND service_id= $2",
        payload.user_id,
        payload.service_id
    )
    .execute(&config.pool)
    .await
    .unwrap();
    Ok(())
}
#[derive(Deserialize)]
struct AddEmailChannelBody {
    user_id: String,
    email: String,
}

#[debug_handler]
async fn handle_add_email_channel(
    Json(payload): Json<AddEmailChannelBody>,
    Extension(config): Extension<ServerConfig>,
) -> impl IntoResponse {
    match add_channel(
        config,
        payload.user_id,
        EmailService::ID.to_string(),
        payload.email,
    )
    .await
    {
        Ok(_) => (StatusCode::OK, Json(true)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(false)),
    }
}

#[derive(Deserialize)]
struct AddTelegramChannelBody {
    user_id: String,
    telegram_username: String,
}

async fn handle_add_telegram_channel(
    Json(payload): Json<AddTelegramChannelBody>,
    Extension(config): Extension<ServerConfig>,
) -> impl IntoResponse {
    if let Some(mut telegram_svc) = config.telegram.clone() {
        match telegram_svc.get_chat_id(payload.telegram_username).await {
            Some(chat_id) => {
                add_channel(
                    config,
                    payload.user_id,
                    TelegramService::ID.to_string(),
                    chat_id.to_string(),
                )
                .await
                .unwrap();
                return (StatusCode::OK, Json(true));
            }
            None => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(false));
            }
        }
    }
    (StatusCode::INTERNAL_SERVER_ERROR, Json(false))
}

pub fn create_router(config: ServerConfig) -> Router {
    let app = Router::new()
        .route("/notify", post(handle_send_notification))
        .route("/add_email_channel", post(handle_add_email_channel))
        .route("/add_telegram_channel", post(handle_add_telegram_channel))
        .route("/remove_channel", post(remove_channel))
        .layer(Extension(config));
    app
}
