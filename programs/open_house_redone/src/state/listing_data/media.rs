use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use super::base_types::{ MediaMetadata, VerificationStatus };

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyMedia {
    pub images: Vec<MediaItem>,
    pub videos: Vec<MediaItem>,
    pub virtual_tour: Option<VirtualTour>,
    pub floor_plan: Option<FloorPlan>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: String,
    pub category: String,
    pub item_type: String,
    pub ipfs_hash: String,
    pub thumbnail_hash: String,
    pub description: String,
    pub metadata: MediaMetadata,
    pub order: u32,
    pub verification_status: VerificationStatus,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualTour {
    pub enabled: bool,
    pub provider: String,
    pub ipfs_hash: String,
    pub thumbnail_hash: String,
    pub description: String,
    pub metadata: VirtualTourMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualTourMetadata {
    pub tour_id: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub total_views: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorPlan {
    pub enabled: bool,
    pub ipfs_hash: String,
    pub thumbnail_hash: String,
    pub description: String,
    pub metadata: MediaMetadata,
}

impl PropertyMedia {
    pub fn total_media_count(&self) -> usize {
        self.images.len() + self.videos.len()
    }
}