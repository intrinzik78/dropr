use clap::ValueEnum;

// list of file types Dropr can operate on
#[derive(Clone,Debug,PartialEq,ValueEnum)]
pub enum ImageType {
    Ai,
    Eps,
    Gif,
    Heif,
    Jiff,
    Jpg,
    Jpeg,
    Png,
    Psd,
    Raw,
    Tif,
    Tiff,
    Webp
}