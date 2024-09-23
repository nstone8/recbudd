use chrono::offset::Local;
use chrono::{Datelike, Timelike};
use ciborium;
use core::time::Duration;
use iced::widget::{text, Container};
use iced::Theme;
use papillae::Analysis;
use papillae::UiMessage;
use papillae::{futures, iced, ralston};
use ralston::image;
use serde;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
//we need to wrap frame in a newtype so we can derive Serialize on it
#[derive(Serialize, Deserialize)]
pub struct RecFrame {
    timestamp: Duration,
    width: u32,
    height: u32,
    data: Vec<u16>,
}

impl RecFrame {
    pub fn new(im: image::DynamicImage, ts: Duration) -> Self {
        //convert to luma16 (this is what the camera captures)
        let luma = im.into_luma16();
        RecFrame {
            timestamp: ts,
            width: luma.width(),
            height: luma.height(),
            data: luma.into_vec(),
        }
    }
    pub fn to_image(self) -> image::DynamicImage {
        let luma = image::ImageBuffer::<image::Luma<u16>, Vec<u16>>::from_vec(
            self.width,
            self.height,
            self.data,
        )
        .expect("couldn't build image from saved data");
        image::DynamicImage::ImageLuma16(luma)
    }
    pub fn get_timestamp(&self) -> f32 {
        self.timestamp.as_secs_f32()
    }
}
pub struct RecBudd {
    file_writer: BufWriter<File>,
}

impl RecBudd {
    ///build a new RecBudd which will store images at `filepath`
    pub fn new_from_path(filepath: &str) -> Self {
        //create a new file
        let file = File::create(filepath).expect("couldn't open file");
        RecBudd {
            file_writer: BufWriter::new(file),
        }
    }
    pub fn new() -> Self {
        let timestamp = Local::now();
        RecBudd::new_from_path(&format!(
            "{}_{}_{}_{}_{}.recbudd",
            timestamp.year(),
            timestamp.month(),
            timestamp.day(),
            timestamp.hour(),
            timestamp.minute()
        ))
    }
}

impl Analysis for RecBudd {
    type DisplayData = Duration;
    fn get_title() -> String {
        String::from("recbudd")
    }
    fn process_frame(
        &mut self,
        frame: ralston::Frame,
        mut sender: futures::channel::mpsc::Sender<(image::DynamicImage, Self::DisplayData)>,
    ) {
        //serialize the frame and send it to the display
        let ts = frame.get_timestamp();
        let im = frame.to_image();
        sender
            .try_send((im.clone(), ts.clone()))
            .expect("couldn't display frame");
        ciborium::into_writer(&RecFrame::new(im, ts), &mut self.file_writer)
            .expect("serialization error");
    }
    fn display_results(
        d: &Self::DisplayData,
    ) -> Container<'_, UiMessage<Self::DisplayData>, Theme, iced::Renderer> {
        let disp_text = text(format!("time elapsed: {}", d.as_secs_f32()));
        Container::new(disp_text)
    }
}
