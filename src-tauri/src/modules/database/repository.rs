use super::error::StorageError;
use super::super::types::*;

pub trait GameRepository {
    fn get_games(&self, page: i64, page_size: i64, search: Option<&str>) -> Result<GameListResponse, StorageError>;
    fn get_game(&self, id: &str) -> Result<Option<GameInfo>, StorageError>;
    fn search_games(&self, query: &str) -> Result<Vec<GameInfo>, StorageError>;
    fn insert_game(&self, game: &GameInfo) -> Result<(), StorageError>;
    fn insert_games(&self, games: &[GameInfo]) -> Result<(), StorageError>;
    fn get_game_count(&self) -> Result<i64, StorageError>;
}

pub trait GameRequirementsRepository {
    fn get_requirements(&self, game_id: &str) -> Result<Option<GameRequirements>, StorageError>;
    fn insert_requirements(&self, requirements: &GameRequirements) -> Result<(), StorageError>;
    fn insert_requirements_batch(&self, requirements_list: &[GameRequirements]) -> Result<(), StorageError>;
}

pub trait HistoryRepository {
    fn get_history(&self, limit: usize) -> Result<Vec<HistoryRecord>, StorageError>;
    fn save_history(&self, record: &HistoryRecord) -> Result<(), StorageError>;
    fn delete_history(&self, id: &str) -> Result<(), StorageError>;
    fn clear_history(&self) -> Result<(), StorageError>;
}
