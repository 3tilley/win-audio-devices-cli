use log::Level;
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::{Config, LoggerProvider};
use opentelemetry_sdk::Resource;
use crate::models::{DeviceRep, Direction};
use crate::switcher::{output_devices};

use clap::Parser;
use clap::Subcommand;
use opentelemetry::trace::FutureExt;
use specs::DisplayInstructions;

mod models;
mod switcher;
mod view_models;
mod specs;

#[derive(Parser)]
#[command(arg_required_else_help = true, author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List audio devices
    List {
        #[clap(long, short, action)]
        input: bool,
        #[clap(long, short, action)]
        output: bool,
        #[clap(long, short, action)]
        json: bool,
        // TODO: Work out short codes
        #[clap(long, action)]
        device_id: Option<Vec<String>>,
        #[clap(long, action)]
        device_name: Option<Vec<String>>,
        #[clap(long, action)]
        device_partial: Option<Vec<String>>,
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
    let args = Cli::parse();

    match &args.command {
        Command::List { input, output, json, device_id, device_name, device_partial } => {
            println!("Listing devices {:?}", device_id);
            let ids = if device_id.is_some() {
                Some(device_id.clone().unwrap().into_iter().map(|s| DeviceRep::DeviceId(s.to_string())).collect::<Vec<_>>())
            } else {
                None
            };
            let display_defaults = DisplayInstructions {
                device_list: ids,
                direction: None,
                states: None,
                json: *json,
            };
            output_devices(display_defaults).unwrap();
        }
        Command::Switch {} => {
            println!("Switching")
        }
    }

}
