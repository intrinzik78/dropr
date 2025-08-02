#[derive(Clone,Debug,PartialEq)]
pub enum FileSize {
    Byte(u32),
    Kilobte(f32),    // 10^3
    Megabyte(f32),   // 10^6
    GigaByte(f32),   // 10^9
    TeraByte(f32),   // 10^12
    PetaByte(f32),   // 10^15
}