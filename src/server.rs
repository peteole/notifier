use crate::{
    config::ServerConfig,
    services::{email::EmailService, telegram::TelegramService},
};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use axum_macros::debug_handler;
use serde::Deserialize;
use utoipa::{Component, OpenApi};
#[derive(Debug, Deserialize, Component)]
struct SendNotificationBody {
    user_id: String,
    subject: String,
    message: String,
}

/// Send notification
///
/// send notification to user with given id on all channels registered for that user
#[utoipa::path(
        post,
        path = "/notify",
        responses(
            (status = 200, description = "Notification sent successfully")
        ),
        request_body = SendNotificationBody
    )]
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

#[derive(Debug, Deserialize, Component)]
struct RemoveChannelBody {
    user_id: String,
    service_id: String,
}

/// Remove channel
///
/// Remove notification channel for user
#[utoipa::path(
    post,
    path = "/remove_channel",
    responses(
        (status = 200, description = "Channel removed successfully")
    ),
    request_body = RemoveChannelBody
)]
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
#[derive(Deserialize, Component)]
struct AddEmailChannelBody {
    user_id: String,
    email: String,
}

/// Add email channel
///
/// Add email notification channel for user
#[utoipa::path(
    post,
    path = "/add_channel/email",
    responses(
        (status = 200, description = "Channel added successfully")
    ),
    request_body = AddEmailChannelBody
)]
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

#[derive(Deserialize, Component)]
struct AddTelegramChannelBody {
    user_id: String,
    telegram_username: String,
}

/// Add telegram channel
///
/// Add telegram notification channel for user
#[utoipa::path(
    post,
    path = "/add_channel/telegram",
    responses(
        (status = 200, description = "Channel added successfully")
    ),
    request_body = AddTelegramChannelBody
)]
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
        .route("/add_channel/email", post(handle_add_email_channel))
        .route("/add_channel/telegram", post(handle_add_telegram_channel))
        .route("/remove_channel", post(remove_channel))
        .layer(Extension(config));
    app
}

#[derive(OpenApi)]
#[openapi(
    handlers(
        handle_send_notification,
        handle_add_email_channel,
        handle_add_telegram_channel,
    ),
    components(AddTelegramChannelBody, AddEmailChannelBody, RemoveChannelBody, SendNotificationBody),
)]
pub struct ApiDoc;
