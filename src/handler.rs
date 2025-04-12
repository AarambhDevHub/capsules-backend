use std::sync::Arc;

use axum::{extract::Path, response::IntoResponse, Extension, Json};
use nanoid::nanoid;
use validator::Validate;

use crate::{db::TableExt, dtos::{CapsuleDto, CreateCapsuleRequest, CreateCapsuleResponse}, error::HttpError, AppState};

pub async fn create_capsule(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreateCapsuleRequest>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let public_id = nanoid!(10);

    let capsule = app_state.db_client.create_capsule(
        &body.name, 
        &body.email, 
        &body.title, 
        &body.message, 
        body.unlock_at, 
        &public_id
    )
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = CreateCapsuleResponse {
        public_id: capsule.public_id,
        unlock_at: capsule.unlock_at.unwrap(),
    };

    Ok(Json(response))
}

pub async fn get_all_capsules(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let capsules = app_state.db_client.get_all_capsules()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let capsule_dto: Vec<CapsuleDto> = capsules.into_iter().map(Into::into).collect();

    Ok(Json(capsule_dto))
}

pub async fn get_capsule_by_public_id(
    Path(public_id): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let capsule = app_state.db_client.get_capsule_by_public_id(&public_id)
    .await
    .map_err(|e| HttpError::server_error(e.to_string()))?;

    match capsule {
        Some(capsule ) => {
            let capsule_dto = CapsuleDto::from(capsule);
            Ok(Json(capsule_dto))
        },
        None => Err(HttpError::bad_request("Capsule not found".to_string()))
    }
}