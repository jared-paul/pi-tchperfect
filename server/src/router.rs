
use std::sync::{Arc, Mutex};

use axum::routing::post;
use axum::{
    routing::get,
    Router,
};

use crate::actors::request::RequestActorHandle;
use crate::actors::videodl::VideoDlActorHandle;
use crate::routes::karaoke::here_video;
use crate::ytdlp::YtdlpError;
use crate::{routes::healthcheck::healthcheck, state::AppState, ytdlp::Ytdlp};

pub async fn create_router_with_state() -> Result<Router, YtdlpError> {
    let ytdlp = Ytdlp::new().await?;
    let ytdlp_clone = ytdlp.clone();

    let videodl_actor = VideoDlActorHandle::new(ytdlp_clone);
    let request_actor_handle = Arc::new(RequestActorHandle::new(videodl_actor));

    let app_state = AppState::new(request_actor_handle);

    Ok(Router::new()
            .route("/api/healthcheck", get(healthcheck))
            // .route("/queue_song", post(queue_song))
            .route("/here_video", get(here_video))
            // .route("/end_song", get(end_song))
            // .route("/play_song", get(play_song))
            .with_state(app_state))
}