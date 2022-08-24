use crate::config::ServerConfig;
use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{Component, OpenApi};
#[derive(Debug, Deserialize, Component)]
struct NotifyBody {
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
        request_body=NotifyBody
    )]
#[debug_handler]
async fn handle_notify(
    Json(payload): Json<NotifyBody>,
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

#[derive(Debug, Serialize, Component)]
struct ChannelResponse {
    service_id: String,
    service_username: String,
}
#[derive(Debug, Serialize, Component)]
struct ChannelsResponse {
    channels: Vec<ChannelResponse>,
    user_id: String,
}
/// Get notification channels for user
///
/// get all channels registered for user with given id
#[utoipa::path(
        get,
        path = "/get_channels/{user_id}",
        responses(
            (status = 200, description = "Notification channels retrieved successfully", body=ChannelsResponse)
        ),
        params(
            ("user_id" = String, path, description = "User id to get notification channels for"),
        )
    )]
#[debug_handler]
async fn handle_get_channels(
    extract::Path(user_id): extract::Path<String>,
    Extension(state): Extension<ServerConfig>,
) -> Json<ChannelsResponse> {
    let channels = sqlx::query!("SELECT * from channels WHERE user_id= $1;", user_id)
        .fetch_all(&state.pool)
        .await
        .unwrap();
    let mapped = ChannelsResponse {
        channels: channels
            .iter()
            .map(|channel| ChannelResponse {
                service_id: channel.service_id.clone(),
                service_username: channel.service_username.clone(),
            })
            .collect(),
        user_id: user_id.clone(),
    };
    Json(mapped)
}

#[derive(Debug, Deserialize, Component)]
struct AddChannelBody {
    // The user's id in this service
    user_id: String,
    // The notification service id, for example "email" or "telegram"
    service_id: String,
    // The username of the user in the notification service (for example an email address or a telegram chat id)
    service_username: String,
}

/// Add channel
///
/// Add notification channel for user
#[utoipa::path(
    post,
    path = "/add_channel",
    responses(
        (status = 200, description = "Channel added successfully")
    ),
    request_body=AddChannelBody
)]
#[debug_handler]
async fn handle_add_channel(
    Json(payload): Json<AddChannelBody>,
    Extension(state): Extension<ServerConfig>,
) -> Result<(), String> {
    sqlx::query!(
        "INSERT INTO channels (user_id,service_id, service_username) VALUES ($1,$2,$3)",
        payload.user_id,
        payload.service_id,
        payload.service_username
    )
    .execute(&state.pool)
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
    request_body=RemoveChannelBody
)]
#[debug_handler]
async fn handle_remove_channel(
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

/// Get the chat ID of a telegram username
///
/// First call this endpoint, then ask the user to send a message to the bot, then the chat id will be returned
#[utoipa::path(
    get,
    path = "/get_telegram_chat_id/{username}",
    responses(
        (status = 200, description = "Returns the chat ID", body=String),
        (status = 500, description = "Could not look up username")
    ),
    params(
        ("username" = String, path, description = "User id to get notification channels for"),
    )
)]
async fn handle_get_telegram_chat_id(
    extract::Path(telegram_username): extract::Path<String>,
    Extension(config): Extension<ServerConfig>,
) -> impl IntoResponse {
    if let Some(mut telegram_svc) = config.telegram.clone() {
        match telegram_svc.get_chat_id(telegram_username).await {
            Some(chat_id) => {
                return (StatusCode::OK, Json(chat_id.to_string()));
            }
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Error picking up status code".to_string()),
                );
            }
        }
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json("Telegram service not configured".to_string()),
    )
}

pub fn create_router(config: ServerConfig) -> Router {
    let app = Router::new()
        .route("/get_channels/:user_id", get(handle_get_channels))
        .route("/notify", post(handle_notify))
        .route("/add_channel", post(handle_add_channel))
        .route(
            "/get_telegram_chat_id/:username",
            get(handle_get_telegram_chat_id),
        )
        .route("/remove_channel", post(handle_remove_channel))
        .layer(Extension(config));
    app
}

#[derive(OpenApi)]
#[openapi(
    handlers(
        handle_notify,
        handle_add_channel,
        handle_remove_channel,
        handle_get_channels,
        handle_get_telegram_chat_id,
    ),
    components(
        AddChannelBody,
        RemoveChannelBody,
        NotifyBody,
        ChannelsResponse,
        ChannelResponse,
    )
)]
pub struct ApiDoc;
