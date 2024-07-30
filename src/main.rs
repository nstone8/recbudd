use free_willy::C11440_22CUSource;
use recbudd::RecBudd;
use papillae::{AnalysisInterface,InterfaceSettings};
use papillae::iced::{Settings,Application};
use clap::Parser;
use std::path::PathBuf;
use chrono::offset::Local;
use chrono::{Datelike,Timelike};
//ask where to save our images
#[derive(Parser)]
struct RecArgs{
    path:PathBuf
}
///connect to the DCAM api
fn get_framesource() -> C11440_22CUSource {
    C11440_22CUSource::new(0, 500)
}

fn main() {
    //make our settings
    let mut args = RecArgs::parse();
    let timestamp = Local::now();
    args.path.push(&format!("{}_{}_{}_{}_{}.recbudd",timestamp.year(),timestamp.month(),timestamp.day(),timestamp.hour(),timestamp.minute()));
    let settings = InterfaceSettings::<C11440_22CUSource, RecBudd> {
        analysis: RecBudd::new_from_path(&args.path.into_os_string().into_string().unwrap()),
        source_fn: get_framesource,
        exposure: 0.01,
        resolution: [1024, 1024],
    };
    AnalysisInterface::<C11440_22CUSource, RecBudd>::run(Settings::with_flags(settings))
        .unwrap();
}

