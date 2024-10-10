use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub trait FileDataMap {
    fn add_file_path(&mut self, key: String, path: String);
}

impl FileDataMap for HashMap<String, Vec<String>> {
    fn add_file_path(&mut self, key: String, path: String) {
        if let Some(paths) = self.get_mut(&key) {
            // Check if the path already exists in the Vec<String>
            if !paths.contains(&path) {
                paths.push(path);
            }
        } else {
            // Key doesn't exist, create a new entry
            self.insert(key, vec![path]);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub enum Role {
    Admin,
    User,
    Unknown,
}
