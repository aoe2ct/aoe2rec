use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn aoe2record(mut record: Multipart) -> Response {
    while let Some(field) = record.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let data = field.bytes().await.unwrap();
            let rec_info = aoe2rec::Savegame::from_bytes(data);
            return match rec_info {
                Ok(rec) => Json(rec).into_response(),
                Err(error) => (
                    StatusCode::BAD_REQUEST,
                    format!("Record could not be parsed: {error}"),
                )
                    .into_response(),
            };
        }
    }
    return (StatusCode::BAD_REQUEST, "No file has been uploaded").into_response();
}
