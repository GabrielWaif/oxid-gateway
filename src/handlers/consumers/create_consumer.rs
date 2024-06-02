use axum::{extract::State, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    domain::models::{
        error::ResultErrors,
        result_body_container::{ResultBodyContainer, ResultBodyContainerConsumer},
    },
    infra::repositories::consumers_repository::{self, NewConsumer},
};

#[utoipa::path(
    post,
    path = "/consumers",
    operation_id = "create_consumer",
    tag = "Consumers",
    responses (
        (status = 201, body = ResultBodyContainerConsumer)
    )
)]
pub async fn create_consumer(
    State(app_state): State<AppState>,
    Json(body): Json<NewConsumer>,
) -> Result<(StatusCode, Json<ResultBodyContainerConsumer>), ResultErrors> {
    let response = consumers_repository::create(&app_state.pool, body)
        .await
        .unwrap();

    return Ok((
        StatusCode::CREATED,
        Json(ResultBodyContainer::success(response)),
    ));
}
