use chrono::offset::Local;
use chrono::{Datelike, Timelike};
use clap::Parser;
use free_willy::C11440_22CUSource;
use papillae::iced::{Application, Settings};
use papillae::{AnalysisInterface, InterfaceSettings};
use recbudd::RecBudd;
use std::path::PathBuf;
//ask where to save our images
#[derive(Parser)]
struct RecArgs {
    path: PathBuf,
    #[arg(short, long, default_value_t = 100)]
    process_buffer: usize,
}
///connect to the DCAM api
fn get_framesource() -> C11440_22CUSource {
    C11440_22CUSource::new(0, 500)
}

fn main() {
    //make our settings
    let mut args = RecArgs::parse();
    let timestamp = Local::now();
    args.path.push(&format!(
        "{}_{}_{}_{}_{}.recbudd",
        timestamp.year(),
        timestamp.month(),
        timestamp.day(),
        timestamp.hour(),
        timestamp.minute()
    ));
    let settings = InterfaceSettings::<C11440_22CUSource, RecBudd> {
        analysis: RecBudd::new_from_path(&args.path.into_os_string().into_string().unwrap()),
        source_fn: get_framesource,
        exposure: 0.001,
        resolution: [1024, 1024],
        process_buffer: args.process_buffer,
    };
    AnalysisInterface::<C11440_22CUSource, RecBudd>::run(Settings::with_flags(settings)).unwrap();
}
