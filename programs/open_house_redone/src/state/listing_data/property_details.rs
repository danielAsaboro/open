use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDetails {
    pub age: PropertyAge,
    pub size: PropertySize,
    pub condition: PropertyCondition,
    pub rooms: PropertyRooms,
    pub finishes: PropertyFinishes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyAge {
    pub category: String,
    pub year_built: String,
    pub last_major_renovation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertySize {
    pub category: String,
    pub dimensions: RoomDimensions,
    pub ceiling_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomDimensions {
    pub total_area: String,
    pub living_room: String,
    pub bedroom: String,
    pub kitchen: String,
    pub bathroom: String,
    pub balcony: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyCondition {
    pub needs_repair: String,
    pub maintenance_history: MaintenanceHistory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceHistory {
    pub last_inspection: String,
    pub regular_maintenance: bool,
    pub last_painted: String,
    pub structural_issues: bool,
    pub recent_repairs: Vec<String>,
    pub ongoing_issues: Vec<String>,
    pub planned_maintenance: Vec<PlannedMaintenance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedMaintenance {
    pub maintenance_type: String,
    pub scheduled_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyRooms {
    pub living_room: Room,
    pub bedroom: Room,
    pub kitchen: Room,
    pub bathroom: Room,
    pub toilet: Toilet,
    pub balcony: Balcony,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub size: String,
    pub features: Vec<String>,
    pub windows: u32,
    pub flooring: String,
    pub orientation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Toilet {
    pub separate: bool,
    pub size: String,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balcony {
    pub balcony_type: String,
    pub size: String,
    pub features: Vec<String>,
    pub orientation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyFinishes {
    pub walls: WallFinishes,
    pub floors: FloorFinishes,
    pub windows: WindowFinishes,
    pub doors: DoorFinishes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallFinishes {
    pub exterior: String,
    pub interior: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorFinishes {
    pub material: String,
    pub finish_type: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowFinishes {
    pub window_type: String,
    pub glazing: String,
    pub nets: String,
    pub burglar_proof: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoorFinishes {
    pub main: String,
    pub internal: String,
    pub condition: String,
}