use ciborium;
use clap::Parser;
use recbudd;
use std::fs::{DirBuilder, File};
use std::io::BufReader;
use std::path::PathBuf;
//ask where to save our images
#[derive(Parser)]
struct ExplodeArgs {
    file_path: PathBuf,
}

fn main() {
    let path = ExplodeArgs::parse().file_path;
    let f = File::open(&path).expect("couldn't open file");
    let mut reader = BufReader::new(f);
    let mut dir = PathBuf::new();
    dir.push(path.parent().unwrap());
    dir.push(format!(
        "{}_images",
        path.file_name().unwrap().to_str().unwrap()
    ));
    let b = DirBuilder::new();
    b.create(&dir).expect("couldn't create image directory");
    //pull off frames until there are none left, save them in a new folder next to the data file
    let mut framenum = 1;
    loop {
        match ciborium::from_reader::<recbudd::RecFrame, &mut BufReader<File>>(&mut reader) {
            Ok(rec_frame) => {
                let ts = rec_frame.get_timestamp();
                let im = rec_frame.to_image();
                let mut im_path = dir.clone();
                im_path.push(format!("{}_{}.png", framenum, ts));
                framenum += 1;
                im.save(im_path).expect("couldn't save image");
            }
            Err(_) => {
                break;
            }
        }
    }
}
