use chrono::{ DateTime, Utc };
use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use super::base_types::{ PropertyTimestamp, VerificationStatus };
use super::media::PropertyMedia;
use super::basic_info::BasicInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyListing {
    pub listing_id: String,
    pub version: String,
    pub timestamp: PropertyTimestamp,
    pub blockchain: BlockchainInfo,
    pub media: PropertyMedia,
    pub basic_info: BasicInfo,
    pub detailed_info: DetailedInfo,
    pub metadata: PropertyMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub network: String,
    pub lister_wallet: String,
    pub contract_address: String,
    pub tokens_minted: String,
    pub verification_status: VerificationStatus,
    pub listing_status: ListingStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyMetadata {
    pub views: u32,
    pub saves: u32,
    pub inquiries: u32,
    pub last_viewed: DateTime<Utc>,
    pub search_tags: Vec<String>,
}

impl PropertyListing {
    pub fn new(data: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(data)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.timestamp.expires_at
    }

    pub fn calculate_total_cost(&self) -> Decimal {
        self.detailed_info.financials.rent.amount +
            self.detailed_info.financials.service_charge.amount +
            self.detailed_info.financials.additional_fees.legal_fee +
            self.detailed_info.financials.additional_fees.agreement_fee +
            self.detailed_info.financials.additional_fees.agency_fee.amount
    }
}