use super::error::StorageError;
use super::repository::{GameRepository, GameRequirementsRepository};
use super::super::types::*;
use super::connection;
use std::fs;

pub struct FileGameRepository;

impl GameRepository for FileGameRepository {
    fn get_games(&self, page: i64, page_size: i64, search: Option<&str>) -> Result<GameListResponse, StorageError> {
        let games = self.load_games()?;
        
        let filtered: Vec<GameInfo> = if let Some(query) = search {
            let query_lower = query.to_lowercase();
            games.into_iter()
                .filter(|g| g.name.to_lowercase().contains(&query_lower))
                .collect()
        } else {
            games
        };
        
        let total = filtered.len() as i64;
        let offset = (page * page_size) as usize;
        let page_games: Vec<GameInfo> = filtered
            .into_iter()
            .skip(offset)
            .take(page_size as usize)
            .collect();
        
        Ok(GameListResponse {
            games: page_games,
            total,
            page,
            page_size,
        })
    }
    
    fn get_game(&self, id: &str) -> Result<Option<GameInfo>, StorageError> {
        let games = self.load_games()?;
        let result = games.into_iter().find(|g| g.id == id);
        Ok(result)
    }
    
    fn search_games(&self, query: &str) -> Result<Vec<GameInfo>, StorageError> {
        let games = self.load_games()?;
        let query_lower = query.to_lowercase();
        let results: Vec<GameInfo> = games.into_iter()
            .filter(|g| g.name.to_lowercase().contains(&query_lower))
            .take(20)
            .collect();
        Ok(results)
    }
    
    fn insert_game(&self, game: &GameInfo) -> Result<(), StorageError> {
        let mut games = self.load_games()?;
        if let Some(index) = games.iter().position(|g| g.id == game.id) {
            games[index] = game.clone();
        } else {
            games.push(game.clone());
        }
        self.save_games(&games)
    }
    
    fn insert_games(&self, games: &[GameInfo]) -> Result<(), StorageError> {
        let mut existing = self.load_games()?;
        
        for game in games {
            if let Some(index) = existing.iter().position(|g| g.id == game.id) {
                existing[index] = game.clone();
            } else {
                existing.push(game.clone());
            }
        }
        
        self.save_games(&existing)
    }
    
    fn get_game_count(&self) -> Result<i64, StorageError> {
        let games = self.load_games()?;
        Ok(games.len() as i64)
    }
}

impl FileGameRepository {
    fn load_games(&self) -> Result<Vec<GameInfo>, StorageError> {
        let path = connection::get_games_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&path)?;
        let games: Vec<GameInfo> = serde_json::from_str(&content)?;
        Ok(games)
    }
    
    fn save_games(&self, games: &[GameInfo]) -> Result<(), StorageError> {
        let path = connection::get_games_file();
        let content = serde_json::to_string_pretty(games)?;
        fs::write(&path, content)?;
        Ok(())
    }
}

pub struct FileRequirementsRepository;

impl GameRequirementsRepository for FileRequirementsRepository {
    fn get_requirements(&self, game_id: &str) -> Result<Option<GameRequirements>, StorageError> {
        let reqs = self.load_requirements()?;
        let result = reqs.into_iter().find(|r| r.game_id == game_id);
        Ok(result)
    }
    
    fn insert_requirements(&self, requirements: &GameRequirements) -> Result<(), StorageError> {
        let mut reqs = self.load_requirements()?;
        if let Some(index) = reqs.iter().position(|r| r.game_id == requirements.game_id) {
            reqs[index] = requirements.clone();
        } else {
            reqs.push(requirements.clone());
        }
        self.save_requirements(&reqs)
    }
    
    fn insert_requirements_batch(&self, requirements_list: &[GameRequirements]) -> Result<(), StorageError> {
        let mut existing = self.load_requirements()?;
        
        for req in requirements_list {
            if let Some(index) = existing.iter().position(|r| r.game_id == req.game_id) {
                existing[index] = req.clone();
            } else {
                existing.push(req.clone());
            }
        }
        
        self.save_requirements(&existing)
    }
}

impl FileRequirementsRepository {
    fn load_requirements(&self) -> Result<Vec<GameRequirements>, StorageError> {
        let path = connection::get_requirements_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&path)?;
        let reqs: Vec<GameRequirements> = serde_json::from_str(&content)?;
        Ok(reqs)
    }
    
    fn save_requirements(&self, reqs: &[GameRequirements]) -> Result<(), StorageError> {
        let path = connection::get_requirements_file();
        let content = serde_json::to_string_pretty(reqs)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
