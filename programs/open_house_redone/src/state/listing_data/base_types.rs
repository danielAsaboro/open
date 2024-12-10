use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };

// Base coordinate and dimension types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    latitude: String,
    longitude: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

// Status enums
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Verified,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListingStatus {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyStatus {
    Vacant,
    Occupied,
}

// Core metadata types
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    dimensions: Dimensions,
    file_size: u64,
    file_type: String,
    device_info: Option<String>,
    coordinates: Option<Coordinates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTimestamp {
    created: DateTime<Utc>,
    last_modified: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

// Document verification
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentStatus {
    available: bool,
    number: Option<String>,
    verified: bool,
}
