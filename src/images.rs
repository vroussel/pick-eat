use std::path::PathBuf;

use image::{DynamicImage, ImageFormat};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

use crate::conf::ImagesConf;

#[derive(Clone, Debug)]
pub struct ImageBank {
    storage_root: PathBuf,
    url_prefix: PathBuf,
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
            storage_root: PathBuf::from(conf.storage_root.clone()),
            url_prefix: PathBuf::from(conf.url_prefix.clone()),
        }
    }

    pub fn add_image(&self, image: RawImage) -> StoredImage {
        let uuid = Uuid::new_v4();
        let ext = *image.ext.extensions_str().first().unwrap();
        let stored_image = StoredImage {
            stem: uuid.to_string(),
            extension: ext.to_owned(),
        };
        for size in ImageSize::iter() {
            let px = size as u32;
            let path = self.stored_image_file_path(&stored_image, size);

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            image
                .data
                .resize(px, px, image::imageops::FilterType::Lanczos3)
                .save(path)
                .unwrap();
        }

        stored_image
    }

    fn stored_image_rel_path(&self, image: &StoredImage, size: ImageSize) -> PathBuf {
        let px = size as u32;
        // get last 2 chars of stem
        let subdir = &image.stem[image.stem.char_indices().nth_back(1).unwrap().0..];
        PathBuf::from(subdir).join(format!("{}-{}.{}", &image.stem, px, &image.extension))
    }

    fn stored_image_file_path(&self, image: &StoredImage, size: ImageSize) -> PathBuf {
        let rel_path = self.stored_image_rel_path(image, size);
        self.storage_root.join(rel_path)
    }

    pub fn stored_image_url(&self, image: &StoredImage, size: ImageSize) -> PathBuf {
        let rel_path = self.stored_image_rel_path(image, size);
        self.url_prefix.join(rel_path)
    }
}
