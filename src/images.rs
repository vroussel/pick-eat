use std::path::PathBuf;

use image::{DynamicImage, ImageFormat};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

use crate::conf::ImagesConf;

#[derive(Clone, Debug)]
pub struct ImageBank {
    root_dir: PathBuf,
}

#[derive(Debug)]
pub struct RawImage {
    pub data: DynamicImage,
    pub ext: ImageFormat,
}

#[derive(Clone)]
pub struct StoredImage {
    pub stem: String,
    pub extension: String,
}

#[derive(EnumIter, Clone, Copy)]
pub enum ImageSize {
    _128 = 128,
    _256 = 256,
    _512 = 512,
    _1024 = 1024,
}

impl ImageBank {
    pub fn new(conf: &ImagesConf) -> Self {
        Self {
            root_dir: PathBuf::from(conf.path.clone()),
        }
    }

    pub fn add_image(&self, image: RawImage) -> StoredImage {
        let uuid = Uuid::new_v4();
        let ext = *image.ext.extensions_str().first().unwrap();
        for size in ImageSize::iter() {
            let px = size as u32;
            let path = self.root_dir.join(format!("{}-{}.{}", &uuid, px, &ext));
            image
                .data
                .resize(px, px, image::imageops::FilterType::Lanczos3)
                .save(path)
                .unwrap();
        }
        StoredImage {
            stem: uuid.to_string(),
            extension: ext.to_owned(),
        }
    }

    pub fn stored_image_path(&self, image: &StoredImage, size: ImageSize) -> PathBuf {
        let px = size as u32;
        self.root_dir
            .join(format!("{}-{}.{}", &image.stem, px, &image.extension))
    }
}
