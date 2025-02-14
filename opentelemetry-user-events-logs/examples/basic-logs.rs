//! run with `$ cargo run --example basic-logs --all-features

use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_user_events_logs::{ExporterConfig, ReentrantLogProcessor, UserEventsExporter};
use std::collections::HashMap;
use tracing::{error, info, warn};
use tracing_subscriber::prelude::*;

fn init_logger() -> SdkLoggerProvider {
    let exporter_config = ExporterConfig {
        default_keyword: 1,
        keywords_map: HashMap::new(),
    };
    let exporter = UserEventsExporter::new("test", None, exporter_config);
    let reenterant_processor = ReentrantLogProcessor::new(exporter);
    SdkLoggerProvider::builder()
        .with_log_processor(reenterant_processor)
        .build()
}

fn main() {
    // Example with tracing appender.
    let logger_provider = init_logger();
    let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
    tracing_subscriber::registry().with(layer).init();

    // event_id is passed as an attribute now, there is nothing in metadata where a
    // numeric id can be stored.
    loop {
        error!(
            name: "my-event-name-error",
            event_id = 20,
            user_name = "otel user",
            user_email = "otel@opentelemetry.io"
        );
        warn!(
            name: "my-event-name-warn",
            event_id = 20,
            user_name = "otel user",
            user_email = "otel@opentelemetry.io"
        );
        info!(
            name: "my-event-name-info",
            event_id = 20,
            user_name = "otel user",
            user_email = "otel@opentelemetry.io"
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    }   
}
