use serde::{Deserialize, Serialize};
use actix_json_response::JsonResponse;
use actix_web::{web, get, Responder, Result};
use actix_web::HttpResponse;

#[derive(Deserialize)]
struct RequestBody {
    start_date: f32,
    end_date: f32,
}

#[derive(Serialize)]
struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Serialize)]
struct Metadata {
    name: String,
}

#[derive(Serialize)]
struct HistoryCoordinate {
    metadata: Metadata,
    location: Location
}

#[derive(Serialize)]
struct ResponseBody {
    data: Vec<HistoryCoordinate>,
}

fn get_data(start_date: f64, end_date: f64) -> ResponseBody {
    let loc1 = Location {
        longitude: -2138123493280.43243 as f64,
        latitude: 2930291302.34243 as f64,
    };

    let metadata1 = Metadata {
        name: "Mcdonalds".to_string(),
    };

    let loc2 = Location {
        longitude: -2138123493280.231433 as f64,
        latitude: 2930291302.43243 as f64,
    };

    let metadata2 = Metadata {
        name: "chipotle".to_string(),
    };

    let history1 = HistoryCoordinate {
        location: loc1,
        metadata: metadata1,
    };

    let history2 = HistoryCoordinate {
        location: loc2,
        metadata: metadata2,
    };

    let response = ResponseBody {
        data: vec![history1, history2],
    };
    response
}

#[get("/history")]
pub async fn history(json: web::Json<RequestBody>) -> Result<impl Responder> {
    let data = get_data(json.start_date.into(), json.end_date.into());
    Ok(web::Json(data))
}

