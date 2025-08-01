use clap::ValueEnum;

// list of file types Dropr can operate on
#[derive(Clone,Debug,PartialEq,ValueEnum)]
pub enum ImageType {
    Ai,
    Bmp,
    Eps,
    Gif,
    Heif,
    Jiff,
    Jpg,
    Jpeg,
    Pdf,
    Png,
    Psd,
    Raw,
    Svg,
    Tif,
    Tiff,
    Webp
}