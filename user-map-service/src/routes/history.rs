use serde::{Deserialize, Serialize};
use actix_json_response::JsonResponse;
use actix_web::{web, get, Responder, Result};
use actix_web::HttpResponse;

use aws_sdk_dynamodb::Client;

#[derive(Deserialize)]
pub struct RequestBody {
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
    let history1 = HistoryCoordinate {
        location: Location {
            longitude: -221393210.329493024 as f64,
            latitude: 30249094320492.234432 as f64,
        },
        metadata: Metadata {
            name: "Mcdonalds".to_string(),
        }
    };

    let history2 = HistoryCoordinate {
        location: Location {
            longitude: -32409324.324234 as f64,
            latitude: 3320943024.341230 as f64,
        },
        metadata: Metadata {
            name: "chipotle".to_string(),
        }
    };

    let response = ResponseBody {
        data: vec![history1, history2],
    };
    response
}

pub async fn history(json: web::Json<RequestBody>, client: web::Data<Client>) -> Result<impl Responder> {
    let data = get_data(json.start_date.into(), json.end_date.into());
    Ok(web::Json(data))
}

