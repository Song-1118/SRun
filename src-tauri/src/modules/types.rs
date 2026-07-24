use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub cores: i32,
    pub threads: i32,
    pub base_frequency: f32,
    pub max_frequency: f32,
    pub cache_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub memory: i64,
    pub driver_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryInfo {
    pub total: i64,
    pub available: i64,
    pub used: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DiskType {
    SSD,
    HDD,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageInfo {
    pub name: String,
    pub device: String,
    pub total: i64,
    pub available: i64,
    pub disk_type: DiskType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub build: String,
    pub architecture: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub gpu: Vec<GpuInfo>,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub os: OsInfo,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameInfo {
    pub id: String,
    pub name: String,
    pub cover_url: String,
    pub developer: String,
    pub publisher: String,
    pub release_date: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Requirement {
    pub cpu: String,
    pub cpu_core_count: Option<i32>,
    pub cpu_frequency: Option<f32>,
    pub gpu: String,
    pub gpu_memory: Option<i64>,
    pub memory: i64,
    pub storage: i64,
    pub os: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameRequirements {
    pub game_id: String,
    pub minimum: Requirement,
    pub recommended: Requirement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MatchLevel {
    Perfect,
    Smooth,
    Basic,
    Impossible,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentResult {
    pub name: String,
    pub user_value: String,
    pub required_value: String,
    pub meets_minimum: bool,
    pub meets_recommended: bool,
    pub benchmark_score: Option<i32>,
    pub required_score: Option<i32>,
    pub percentage_above_minimum: f32,
    pub percentage_above_recommended: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkType {
    CPU,
    GPU,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkData {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub r#type: BenchmarkType,
    pub score: i32,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    GameSetting,
    HardwareUpgrade,
    SystemOptimization,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suggestion {
    pub id: String,
    pub r#type: SuggestionType,
    pub title: String,
    pub content: String,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComparisonResult {
    pub game_id: String,
    pub game_name: String,
    pub score: i8,
    pub level: MatchLevel,
    pub cpu_result: ComponentResult,
    pub gpu_result: ComponentResult,
    pub memory_result: ComponentResult,
    pub storage_result: ComponentResult,
    pub suggestions: Vec<Suggestion>,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryRecord {
    pub id: String,
    pub game_id: String,
    pub game_name: String,
    pub score: i8,
    pub level: MatchLevel,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameListResponse {
    pub games: Vec<GameInfo>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub theme: String,
    pub language: String,
    pub auto_update: bool,
    pub data_update_frequency: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectionProgress {
    pub step: String,
    pub progress: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataUpdateInfo {
    pub version: String,
}
