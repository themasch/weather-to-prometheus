use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::sync::atomic::AtomicU64;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    pub location_id: u64,
    pub location: String,
}

pub type TempFamily = Family<Labels, Gauge<f64, AtomicU64>>;
pub type GaugeFamily = Family<Labels, Gauge>;
pub type CountFamily = Family<Labels, Counter>;

pub struct MetricFamilies {
    pub(crate) temp: TempFamily,
    pub(crate) temp_feel: TempFamily,
    pub(crate) req_count: CountFamily,
    pub(crate) humidity: GaugeFamily,
    pub(crate) pressure: GaugeFamily,
}

pub fn init_metrics() -> (Registry, MetricFamilies) {
    let mut registry = Registry::default();

    let temp = TempFamily::default();
    let temp_feel = TempFamily::default();
    let req_count = CountFamily::default();
    let pressure = GaugeFamily::default();
    let humidity = GaugeFamily::default();

    registry.register_with_unit(
        "temperature",
        "current temp in °C",
        prometheus_client::registry::Unit::Celsius,
        temp.clone(),
    );
    registry.register_with_unit(
        "temperature_feel",
        "felt current temp in °C",
        prometheus_client::registry::Unit::Celsius,
        temp_feel.clone(),
    );
    registry.register(
        "api_requests",
        "number of OWM API requests since sstart",
        req_count.clone(),
    );

    use prometheus_client::registry::Unit;
    registry.register_with_unit(
        "pressure",
        "preassure",
        Unit::Other("hPa".into()),
        pressure.clone(),
    );
    registry.register("humidity", "humidity", humidity.clone());

    (
        registry,
        MetricFamilies {
            temp,
            temp_feel,
            req_count,
            pressure,
            humidity,
        },
    )
}
