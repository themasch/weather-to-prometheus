use axum::extract::State;
use axum::{routing::get, Router};

use owm_api::LatLon;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

use log::*;

mod metrics;
mod owm_api;

use metrics::*;

struct AppState {
    metric_registry: Registry,
}

async fn get_metrics(State(state): State<Arc<AppState>>) -> String {
    let mut buffer = String::new();
    encode(&mut buffer, &state.metric_registry).unwrap();

    buffer
}

async fn update_weather_for(client: &owm_api::ApiClient, metrics: &MetricFamilies, loc: &LatLon) {
    let w = match client.get_current_weather(loc).await {
        Ok(w) => w,
        Err(err) => {
            error!("Could not fetch current weather form API: {}", err);
            return;
        }
    };

    let labels = &Labels {
        location_id: w.id,
        location: w.name,
    };
    metrics.temp.get_or_create(labels).set(w.main.temp);
    metrics
        .temp_feel
        .get_or_create(labels)
        .set(w.main.feels_like);
    metrics.humidity.get_or_create(labels).set(w.main.humidity);
    metrics.pressure.get_or_create(labels).set(w.main.pressure);
    metrics.req_count.get_or_create(labels).inc();
}

async fn update_metrics(metrics: MetricFamilies, client: owm_api::ApiClient, loc: LatLon) {
    let mut interval = time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;
        update_weather_for(&client, &metrics, &loc).await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = std::env::var("OWM_API_KEY").unwrap();
    let lat_lon: LatLon = std::env::var("OWM_LAT_LON")
        .unwrap()
        .parse()
        .expect("could not parse location data");

    let (registry, families) = init_metrics();

    let state = Arc::new(AppState {
        metric_registry: registry,
    });

    // create our periodic update task, that pulls the weather api in fixed intervals
    tokio::spawn(async move {
        let api_client = owm_api::ApiClient::create(api_key);
        update_metrics(families, api_client, lat_lon).await
    });

    // build our application with a single route
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
