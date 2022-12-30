use serde::Deserialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub coord: LatLon,
    pub main: MainWeatherData,
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct MainWeatherData {
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Debug, Deserialize)]
pub struct LatLon {
    lat: f32,
    lon: f32,
}

impl FromStr for LatLon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(',') {
            return Err(());
        }

        let (slat, slon) = s.split_once(',').ok_or(())?;

        let lat = slat.parse().map_err(|_| ())?;
        let lon = slon.parse().map_err(|_| ())?;

        Ok(Self { lat, lon })
    }
}

pub struct ApiClient {
    api_key: String,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {source}")]
    HttpError {
        #[from]
        source: reqwest::Error,
    },
}

impl ApiClient {
    pub fn create<S: Into<String>>(api_key: S) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    pub async fn get_current_weather(&self, w: &LatLon) -> Result<CurrentWeather, ApiError> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            w.lat, w.lon, self.api_key
        );

        Ok(reqwest::get(&url).await?.json().await?)
    }
}
