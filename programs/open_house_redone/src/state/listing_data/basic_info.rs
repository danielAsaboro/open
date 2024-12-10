// Pricing
#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    yearly_rent: Decimal,
    currency: String,
    payment_terms: String,
    negotiable: bool,
    price_range: String,
}

// Availability
#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    status: PropertyStatus,
    expiry_month: String,
    expiry_year: String,
    intent_to_leave: String,
    move_in_date: String,
    minimum_stay_period: String,
    viewing_hours: String,
}
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use super::base_types::{ Coordinates, PropertyStatus };

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicInfo {
    pub property_type: PropertyType,
    pub location: Location,
    pub pricing: Pricing,
    pub availability: Availability,
    pub overview_stats: OverviewStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyType {
    pub house_type: String,
    pub building_type: String,
    pub floor_level: u32,
    pub total_floors: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub nearest_bus_stop: String,
    pub main_road_access: String,
    pub estate: String,
    pub street: String,
    pub area: String,
    pub lga: String,
    pub state: String,
    pub coordinates: Coordinates,
    pub landmarks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricing {
    pub yearly_rent: Decimal,
    pub currency: String,
    pub payment_terms: String,
    pub negotiable: bool,
    pub price_range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    pub status: PropertyStatus,
    pub expiry_month: String,
    pub expiry_year: String,
    pub intent_to_leave: String,
    pub move_in_date: String,
    pub minimum_stay_period: String,
    pub viewing_hours: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewStats {
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub toilets: u32,
    pub parking_spaces: u32,
    pub approximate_size: String,
}

impl BasicInfo {
    pub fn is_available_for_viewing(&self) -> bool {
        self.availability.status == PropertyStatus::Vacant
    }
}

impl Location {
    pub fn is_within_radius(&self, target: &Coordinates, radius_km: f64) -> bool {
        // Haversine formula implementation would go here
        todo!()
    }
}
