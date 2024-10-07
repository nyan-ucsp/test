use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use image::GenericImageView;
use mime_guess::from_path;
use serde_json::Value;
use uuid::Uuid;

use crate::common::enums::FileDataMap;
use crate::common::models::file_metadata::{FileMetadata, ImageMetadata, VideoMetadata};

pub fn save_file_to_directory(filepath: &str, bytes: &Vec<u8>) {
    create_directory_if_not_exists(filepath);
    let mut f = fs::File::create(filepath).unwrap();
    f.write_all(bytes).unwrap();
}
pub fn create_directory_if_not_exists(path: &str) {
    if let Some(parent) = Path::new(path).parent() {
        // Check if the directory exists
        if !parent.exists() {
            // Create the directory and all necessary parent directories
            fs::create_dir_all(parent).unwrap();
        }
    }
}

pub fn move_file_and_replace(src: &str, dest: &str) {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);
    // Ensure destination directory exists
    create_directory_if_not_exists(dest);

    // Check if the destination file already exists
    if dest_path.exists() {
        // Remove the existing destination file
        if let Err(e) = fs::remove_file(dest_path) {
            eprintln!("Error removing existing file {}: {}", dest_path.display(), e);
            return;
        }
    }

    // Try to copy the file
    if let Err(e) = fs::copy(src_path, dest_path) {
        eprintln!("Error copying file from {} to {}: {}", src_path.display(), dest_path.display(), e);
        return;
    }

    // Remove the original file
    if let Err(e) = fs::remove_file(src_path) {
        eprintln!("Error removing original file {}: {}", src_path.display(), e);
    }
}

pub fn delete_directory_if_exists(path: &str) {
    let path = Path::new(path);
    // Check if the directory exists
    if path.exists() && path.is_dir() {
        // remove the directory
        fs::remove_dir_all(path).unwrap();
    }
}

pub fn delete_file_if_exists(path: &str) {
    let path = Path::new(path);
    // Check if the directory exists
    if path.exists() && path.is_file() {
        // remove the directory
        fs::remove_file(path).unwrap();
    }
}

pub fn get_data_directory() -> String {
    // format!("{}/data", env::current_dir().expect("REASON").display(), )
    "data".to_string()
}
pub fn get_project_directory() -> String {
    format!("{}", env::current_dir().expect("REASON").display(), )
}

pub async fn parse_payload_data(mut payload: Multipart) -> Result<(HashMap<String, Value>, String), String> {
    let tmp_uuid = Uuid::new_v4().to_string();
    let tmp_path = format!("{}/tmp/{}", get_project_directory(), tmp_uuid);
    // !dynamic mapping
    let mut form_data_map: HashMap<String, Value> = HashMap::new();
    let mut file_data_map: HashMap<String, Vec<String>> = HashMap::new();

    create_directory_if_not_exists(&tmp_path.clone());
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition().unwrap().clone();
        // let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_else(|| "application/octet-stream".to_string());
        let name = content_disposition.get_name().unwrap().to_string();
        if content_disposition.is_form_data() {
            if let Some(filename) = content_disposition.get_filename() {
                let uuid_path = Uuid::new_v4().to_string();
                let mut file_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.unwrap();
                    file_data.extend_from_slice(&chunk);
                }
                // Save file to disk or handle it as needed
                let filepath = format!("{}/{}/{}", tmp_path.clone(), uuid_path, filename);
                save_file_to_directory(&filepath, &file_data);
                // Store the file path in the form data map
                file_data_map.add_file_path(name, filepath);
            } else {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.unwrap();
                    value.extend_from_slice(&chunk);
                }
                let json_value = Value::String(String::from_utf8(value).unwrap());
                form_data_map.insert(name.to_string(), json_value);
            }
        }
    }
    // Convert File Data HashMap to JSON value and then insert to FormData struct
    for (key, paths) in file_data_map {
        form_data_map.insert(key, Value::Array(paths.into_iter().map(Value::String).collect()));
    }
    Ok((form_data_map, tmp_path))
}

pub fn get_file_metadata(file_path: &str) -> FileMetadata {
    let metadata = fs::metadata(file_path).unwrap();
    let size = metadata.len();
    let original_name = get_file_name(file_path).unwrap_or(String::from("No file found"));
    let mime_type = from_path(file_path).first_or_octet_stream();
    let content_type = mime_type.to_string();
    let mut image_data: Option<ImageMetadata> = None;
    let video_data: Option<VideoMetadata> = None;
    if content_type.starts_with("image/") {
        image_data = get_image_metadata(file_path);
    } else if content_type.starts_with("video/") {
        //TODO Add video with height and generate thumbnail here
    } else {}
    FileMetadata {
        content_type,
        original_name,
        size,
        image_data,
        video_data,
    }
}

fn get_file_name(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);

    // Get the file name as an Option<&OsStr>
    path.file_name()
        // Convert the OsStr to a &str
        .and_then(|name| name.to_str())
        // Convert &str to String
        .map(|name| name.to_string())
}

fn get_image_metadata(file_path: &str) -> Option<ImageMetadata> {
    // Load the image from the file path
    let img = match image::open(&Path::new(file_path)) {
        Ok(img) => img,
        Err(_) => return None,
    };
    // Get the dimensions
    let (width, height) = img.dimensions();
    Option::from(ImageMetadata { width, height })
}

fn is_multi_keywords(word: &str) -> bool {
    word.ends_with("s") || word.ends_with("es")
}

pub fn remove_values_from_vec_string<'a>(filter_vec: &Vec<String>, original_vec: &'a mut Vec<String>) -> &'a mut Vec<String> {
    original_vec.retain(|value| !filter_vec.contains(value));
    original_vec
}

