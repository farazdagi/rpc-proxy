use crate::metrics::Metrics;
use crate::project::Registry;
use crate::BuildInfo;
use crate::Config;
use opentelemetry_prometheus::PrometheusExporter;

pub struct State {
    pub config: Config,
    pub exporter: PrometheusExporter,
    pub metrics: Metrics,
    pub registry: Registry,
    pub build_info: BuildInfo,
}

build_info::build_info!(fn build_info);

pub fn new_state(
    config: Config,
    exporter: PrometheusExporter,
    metrics: Metrics,
    registry: Registry,
) -> State {
    let build_info: &BuildInfo = build_info();

    State {
        config,
        exporter,
        metrics,
        registry,
        build_info: build_info.clone(),
    }
}
