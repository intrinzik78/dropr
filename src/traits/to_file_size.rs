use crate::enums::{Error,FileSize};

type Result<T> = std::result::Result<T,Error>;

const KB:u64 = 1024;
const MB:u64 = 1_048_576;
const GB:u64 = 1_073_741_824;
const TB:u64 = 1_099_511_627_776;
const PB:u64 = 1_000_000_000_000_000;


pub trait ToFileSize {
    fn to_file_size(self) -> Result<FileSize>;
}

impl ToFileSize for u64 {
    fn to_file_size(self) -> Result<FileSize> {
        let f32_file_size: f64 = match self {
            ..KB   => self as f64,
            KB..MB => self as f64 / 1_000_f64,
            MB..GB => self as f64 / 1_000_000_f64,
            GB..TB => self as f64 / 1_000_000_000_f64,
            TB..PB => self as f64 / 1_000_000_000_000_f64,
            PB..   => self as f64 / PB as f64
        };

        let fractional_size = ((f32_file_size * 10.0).trunc() / 10.0) as f32;

        let file_size = match self {
            ..KB   => FileSize::Byte(self as u32),
            KB..MB => FileSize::Kilobte(fractional_size),
            MB..GB => FileSize::Megabyte(fractional_size),
            GB..TB => FileSize::GigaByte(fractional_size),
            TB..PB => FileSize::TeraByte(fractional_size),
            PB..   => FileSize::PetaByte(fractional_size)
        };

        Ok(file_size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u64_to_file_size() {
        // test lower break point
        assert_eq!(0_u64.to_file_size().unwrap(), FileSize::Byte(0));
        assert_eq!(KB.to_file_size().unwrap(), FileSize::Kilobte(1.0));
        assert_eq!(MB.to_file_size().unwrap(), FileSize::Megabyte(1.0));
        assert_eq!(GB.to_file_size().unwrap(), FileSize::GigaByte(1.0));
        assert_eq!(TB.to_file_size().unwrap(), FileSize::TeraByte(1.0));
        assert_eq!(PB.to_file_size().unwrap(), FileSize::PetaByte(1.0));

        // test upper break point
        assert_eq!((KB-1).to_file_size().unwrap(), FileSize::Byte(1023));
        assert_eq!((MB-1).to_file_size().unwrap(), FileSize::Kilobte(1_048.5_f32));
        assert_eq!((GB-1).to_file_size().unwrap(), FileSize::Megabyte(1_073.7_f32));
        assert_eq!((TB-1).to_file_size().unwrap(), FileSize::GigaByte(1_099.5_f32));
        assert_eq!((PB-1).to_file_size().unwrap(), FileSize::TeraByte(999.9_f32));
    }
}