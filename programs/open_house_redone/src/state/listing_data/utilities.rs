use rust_decimal::Decimal;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Utilities {
    pub power: PowerUtility,
    pub water: WaterUtility,
    pub internet: InternetConnectivity,
    pub generator: Generator,
    pub cooking: CookingUtility,
    pub waste: WasteManagement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerUtility {
    pub light_situation: String,
    pub meter_type: String,
    pub meter_number: String,
    pub average_monthly_bill: Decimal,
    pub last_bill_amount: Decimal,
    pub last_bill_date: String,
    pub electricity_band: String,
    pub power_company: String,
    pub load_limit: String,
    pub power_rating: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterUtility {
    pub situation: String,
    pub source: Vec<String>,
    pub storage: WaterStorage,
    pub water_pressure: String,
    pub pumping_schedule: String,
    pub treatment_system: String,
    pub backup: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaterStorage {
    pub ground_tank: String,
    pub overhead_tank: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternetConnectivity {
    pub fiber_optic: InternetFiberOptic,
    pub telcos: TelcoNetworks,
    pub average_speed: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternetFiberOptic {
    pub available: bool,
    pub providers: Vec<String>,
    pub max_speed: String,
    pub installation_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelcoNetworks {
    pub mtn: u8,
    pub airtel: u8,
    pub glo: u8,
    pub nine_mobile: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    pub generator_type: String,
    pub details: GeneratorDetails,
    pub fuel_policy: FuelPolicy,
    pub shared_generator: SharedGenerator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorDetails {
    pub capacity: String,
    pub brand: String,
    pub max_allowed_size: String,
    pub housing_location: String,
    pub operating_hours: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelPolicy {
    pub policy_type: String,
    pub storage_allowed: bool,
    pub max_storage_limit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedGenerator {
    pub available: bool,
    pub capacity: Option<String>,
    pub schedule: Option<String>,
    pub monthly_fee: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookingUtility {
    pub gas_allowed: bool,
    pub gas_type: Vec<String>,
    pub electric_cooking_allowed: bool,
    pub kerosene_allowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasteManagement {
    pub management_system: String,
    pub collection_days: Vec<String>,
    pub monthly_fee: Decimal,
    pub segregation_required: bool,
    pub bin_location: String,
}