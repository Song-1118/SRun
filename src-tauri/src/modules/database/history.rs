use super::error::StorageError;
use super::repository::HistoryRepository;
use super::super::types::*;
use super::connection;
use std::fs;

pub struct FileHistoryRepository;

impl HistoryRepository for FileHistoryRepository {
    fn get_history(&self, limit: usize) -> Result<Vec<HistoryRecord>, StorageError> {
        let mut records = self.load_history()?;
        records.sort_by(|a, b| b.detected_at.cmp(&a.detected_at));
        records.truncate(limit);
        Ok(records)
    }
    
    fn save_history(&self, record: &HistoryRecord) -> Result<(), StorageError> {
        let mut records = self.load_history()?;
        if let Some(index) = records.iter().position(|r| r.id == record.id) {
            records[index] = record.clone();
        } else {
            records.push(record.clone());
        }
        records.sort_by(|a, b| b.detected_at.cmp(&a.detected_at));
        records.truncate(100);
        self.save_history_records(&records)
    }
    
    fn delete_history(&self, id: &str) -> Result<(), StorageError> {
        let mut records = self.load_history()?;
        records.retain(|r| r.id != id);
        self.save_history_records(&records)
    }
    
    fn clear_history(&self) -> Result<(), StorageError> {
        let path = connection::get_history_file();
        fs::write(&path, "[]")?;
        Ok(())
    }
}

impl FileHistoryRepository {
    fn load_history(&self) -> Result<Vec<HistoryRecord>, StorageError> {
        let path = connection::get_history_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&path)?;
        let records: Vec<HistoryRecord> = serde_json::from_str(&content)?;
        Ok(records)
    }
    
    fn save_history_records(&self, records: &[HistoryRecord]) -> Result<(), StorageError> {
        let path = connection::get_history_file();
        let content = serde_json::to_string_pretty(records)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
