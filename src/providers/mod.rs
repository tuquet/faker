use crate::models::{IdentityData, LocationData, NameData};

pub mod vietnam;
pub mod usa;
pub mod japan;

pub trait NationalityProvider: Send + Sync {
    /// Two-letter ISO / Alpha-2 nationality code (e.g. "VN", "US", "JP")
    fn nat_code(&self) -> &'static str;

    /// Official country name in English
    fn country_name(&self) -> &'static str;

    /// Generate authentic full name and parts
    fn generate_name(&self, is_male: bool, rng: &mut dyn rand::RngCore) -> NameData;

    /// Generate authentic hierarchical address
    fn generate_location(&self, rng: &mut dyn rand::RngCore) -> LocationData;

    /// Generate government-compliant National ID (CCCD, SSN, My Number, etc.)
    fn generate_id(&self, birth_year: u32, is_male: bool, rng: &mut dyn rand::RngCore) -> IdentityData;

    /// Generate realistic telecom phone number
    fn generate_phone(&self, rng: &mut dyn rand::RngCore) -> String;

    /// Generate typical professional job title
    fn generate_job(&self, rng: &mut dyn rand::RngCore) -> &'static str;

    /// Return timezone offset and description (e.g. ("+07:00", "Bangkok, Hanoi, Jakarta"))
    fn timezone(&self) -> (&'static str, &'static str);
}

pub fn get_provider(nat_code: &str) -> Box<dyn NationalityProvider> {
    let clean = nat_code.trim().to_uppercase();
    match clean.as_str() {
        "JP" | "JAPAN" => Box::new(japan::JapanProvider),
        "US" | "USA" | "EN" => Box::new(usa::UsaProvider),
        _ => Box::new(vietnam::VietnamProvider),
    }
}

pub fn supported_nationalities() -> &'static [&'static str] {
    &["VN", "US", "JP"]
}
