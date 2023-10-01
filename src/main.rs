use log::{error, Level};
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::{Config, LoggerProvider};
use opentelemetry_sdk::Resource;
use crate::contract::{DefaultAudioDeviceSwitch, Direction, DisplayInstructions};
use crate::switcher::display_devices;

use clap::Parser;
use clap::Subcommand;

mod contract;
mod switcher;

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// List audio devices
    List {
        #[clap(long, short, action)]
        input: bool,
        #[clap(long, short, action)]
        output: bool,
    },
    /// Switch default audio device
    Switch {
    },
}

fn main() {
    let exporter = opentelemetry_stdout::LogExporterBuilder::default()
        // uncomment the below lines to pretty print output.
        // .with_encoder(|writer, data|
        //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();
    let logger_provider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "logs-basic-example",
            )])),
        )
        .with_simple_exporter(exporter)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Error.to_level_filter());

    wasapi::initialize_mta().unwrap();
    let args = SubCommand::parse();

    let display_defaults = DisplayInstructions {
        device_list: None,
        direction: Direction::Output,
        states: None,
    };

    let input_display_defaults = DisplayInstructions {
        device_list: None,
        direction: Direction::Input,
        states: None,
    };
    display_devices(display_defaults).unwrap();
    display_devices(input_display_defaults).unwrap();
}
