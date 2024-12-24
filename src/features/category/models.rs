use crate::common::ne_parse::NEParse;
use crate::schema::category;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    ToSchema,
    Identifiable,
    AsChangeset,
    Insertable,
    Clone,
    IntoParams,
    PartialEq,
    Eq,
)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, IntoParams, PartialEq, Eq)]
pub struct CategoryResponse {
    pub id: Option<i32>,
    pub name: String,
}

impl CategoryResponse {
    pub fn from_category(category: Category) -> Self {
        CategoryResponse {
            id: category.id,
            name: category.name
        }
    }
    pub fn from_categories(categories: Vec<Category>) -> Vec<Self> {
        categories
            .into_iter()
            .map(|e| Self::from_category(e))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddCategoryRequest {
    /// Category name
    pub name: String,
}

impl AddCategoryRequest {
    pub async fn from_payload_data<'a>(
        payload_data: HashMap<String, Value>,
    ) -> Result<Self, &'a str> {
        let name = if payload_data.contains_key("name") {
            NEParse::opt_immut_str_to_option_string(payload_data["name"].as_str())
        }else{
            None
        };
        if name.is_none() {
            Err("Name cannot be empty")
        } else {
            if name.unwrap().trim().is_empty() {
                Err("Name cannot be empty")
            }else{
                Ok(AddCategoryRequest {
                    name: NEParse::opt_immut_str_to_option_string(payload_data["name"].as_str())
                        .expect("Invalid name"),
                })
            }

        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCategoryRequest {
    /// Category id
    pub id: i32,
    pub name: String,
}

impl UpdateCategoryRequest {
    pub async fn from_payload_data<'a>(
        payload_data: HashMap<String, Value>,
    ) -> Result<Self, &'a str> {
        let id = if payload_data.contains_key("id") {
            NEParse::opt_immut_str_to_opt_i32(payload_data["id"].as_str())
        } else {
           None
        };
        let name = if payload_data.contains_key("name") {
            NEParse::opt_immut_str_to_option_string(payload_data["name"].as_str())
        } else {
            None
        };

        if id.is_none() || name.is_none() {
            Err("Id or name cannot be empty")
        }else{

            Ok(UpdateCategoryRequest {
                id: id.unwrap(),
                name: name.unwrap(),
            })
        }

    }
}
