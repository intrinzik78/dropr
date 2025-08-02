use crate::enums::{ FsEntryType, ImageType };

pub trait ToImageType {
    fn to_image_type(self) -> Option<ImageType>;
}

impl ToImageType for FsEntryType {
    fn to_image_type(self) -> Option<ImageType> {
        let filename = match self {
            FsEntryType::File(filename) => filename,
            _ => return None
        };

        let (_,search_string) = filename.split_once('.')?;

        let image_type = match search_string {
            "ai" => ImageType::Ai,
            "bmp" => ImageType::Bmp,
            "eps" => ImageType::Eps,
            "gif" => ImageType::Gif,
            "heif" => ImageType::Heif,
            "jiff" => ImageType::Jiff,
            "jpg" => ImageType::Jpg,
            "jpeg" => ImageType::Jpeg,
            "png" => ImageType::Png,
            "psd" => ImageType::Psd,
            "raw" => ImageType::Raw,
            "svg" => ImageType::Svg,
            "tif" => ImageType::Tif,
            "tiff" => ImageType::Tiff,
            "webp" => ImageType::Webp,
            "pdf" => ImageType::Pdf,
            _ => return None
        };

        Some(image_type)

    }
}