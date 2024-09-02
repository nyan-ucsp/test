use std::{env, fs};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use actix_multipart::{Field, Multipart};
use futures_util::StreamExt;
use image::GenericImageView;
use mime_guess::from_path;
use serde::de::DeserializeOwned;
use serde_json::Value;
use uuid::Uuid;

use crate::common::enums::FileDataMap;
use crate::common::models::file_metadata::{FileMetadata, ImageMetadata, VideoMetadata};
use crate::common::models::response_message::ResponseMessage;

pub fn save_file_to_directory(filepath: &str, bytes: &Vec<u8>) {
    create_directory_if_not_exists(filepath);
    let mut f = std::fs::File::create(filepath).unwrap();
    f.write_all(bytes).unwrap();
}
pub fn create_directory_if_not_exists(path: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        // Check if the directory exists
        if !parent.exists() {
            // Create the directory and all necessary parent directories
            std::fs::create_dir_all(parent).unwrap();
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

pub async fn parse_payload_data<T>(mut payload: Multipart) -> Result<(T, String), actix_web::HttpResponse>
where
    T: DeserializeOwned,
{
    let err = Err(actix_web::HttpResponse::BadRequest().json(ResponseMessage { message: String::from("Failed to parsed payload") }));
    let mut form_data_map: HashMap<String, Value> = HashMap::new();
    let tmp_uuid = Uuid::new_v4().to_string();
    let tmp_path = format!("{}/tmp/{}", get_project_directory(), tmp_uuid);
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
                let json_value = parse_json_value(&mut field).await;
                form_data_map.insert(name.to_string(), json_value);
            }
        }
    }
    // Convert File Data HashMap to JSON value and then insert to FormData struct
    for (key, mut paths) in file_data_map {
        if is_multi_keywords(key.as_str()) {
            form_data_map.insert(key, Value::Array(paths.into_iter().map(Value::String).collect()));
        } else {
            if paths.len() == 1 {
                form_data_map.insert(key, Value::String(paths.pop().unwrap()));
            } else if paths.len() > 1 {
                form_data_map.insert(key, Value::Array(paths.into_iter().map(Value::String).collect()));
            }
        }
    }
    // Convert HashMap to JSON value and then to FormData struct
    let form_data_json = match serde_json::to_value(form_data_map) {
        Ok(form_data_json) => form_data_json,
        Err(_) => {
            delete_directory_if_exists(&tmp_path);
            return err;
        }
    };
    let form_data: T = match serde_json::from_value(form_data_json) {
        Ok(form_data) => form_data,
        Err(e) => {
            println!("{}", e);
            delete_directory_if_exists(&tmp_path);
            return err;
        }
    };
    Ok((form_data, tmp_path))
}


async fn parse_json_value(field: &mut Field) -> Value {
    let mut value = Vec::new();
    while let Some(chunk) = field.next().await {
        let chunk = chunk.unwrap();
        value.extend_from_slice(&chunk);
    }
    let value_str = String::from_utf8(value).unwrap();
    // Determine the appropriate type and convert the string value accordingly
    let json_value = if value_str.trim().starts_with('"') && value_str.trim().ends_with('"') {
        Value::String(value_str)
    } else if let Ok(parsed_bool) = value_str.parse::<bool>() {
        Value::Bool(parsed_bool)
    } else if let Ok(parsed_i32) = value_str.parse::<i32>() {
        Value::Number(parsed_i32.into())
    } else if let Ok(parsed_f64) = value_str.parse::<f64>() {
        Value::Number(serde_json::Number::from_f64(parsed_f64).unwrap())
    } else if let Ok(parsed_datetime) = chrono::DateTime::parse_from_rfc3339(&value_str).map(|dt| dt.with_timezone(&chrono::Utc)) {
        Value::String(parsed_datetime.to_rfc3339())
    } else {
        Value::String(value_str)
    };
    json_value
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