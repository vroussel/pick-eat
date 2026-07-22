use std::path::PathBuf;

use image::{DynamicImage, ImageFormat};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

use crate::{AppError, conf::ImagesConf};

#[derive(Clone, Debug)]
pub struct ImageBank {
    storage_root: PathBuf,
    url_prefix: String,
}

#[derive(Debug)]
pub struct RawImage {
    pub data: DynamicImage,
    #[allow(dead_code)]
    pub ext: ImageFormat,
}

#[derive(Clone)]
pub struct StoredImage {
    pub stem: Uuid,
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
            url_prefix: conf.url_prefix.trim_end_matches('/').to_owned(),
        }
    }

    pub fn add_image(&self, image: RawImage) -> Result<StoredImage, AppError> {
        let uuid = Uuid::new_v4();
        let (ext, ext_str) = (ImageFormat::WebP, "webp");
        let stored_image = StoredImage {
            stem: uuid,
            extension: ext_str.to_owned(),
        };
        for size in ImageSize::iter() {
            let px = size as u32;
            let path = self.stored_image_file_path(&stored_image, size);

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            image
                .data
                .resize(px, px, image::imageops::FilterType::Lanczos3)
                .save_with_format(path, ext)?;
        }

        Ok(stored_image)
    }

    fn stored_image_filename(&self, image: &StoredImage, size: ImageSize) -> String {
        let px = size as u32;
        format!("{}-{}.{}", image.stem, px, image.extension)
    }

    fn stored_image_subdir(&self, image: &StoredImage) -> String {
        // get last 2 hex digits of stem
        format!("{:02x}", image.stem.as_bytes()[15])
    }

    fn stored_image_file_path(&self, image: &StoredImage, size: ImageSize) -> PathBuf {
        let filename = self.stored_image_filename(image, size);
        let subdir = self.stored_image_subdir(image);
        self.storage_root.join(subdir).join(filename)
    }

    pub fn stored_image_url(&self, image: &StoredImage, size: ImageSize) -> String {
        let filename = self.stored_image_filename(image, size);
        let subdir = self.stored_image_subdir(image);
        format!("{}/{}/{}", self.url_prefix, subdir, filename)
    }
}
