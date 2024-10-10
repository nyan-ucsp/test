use chrono::{DateTime, NaiveDateTime, SecondsFormat, TimeZone, Utc};
use serde_json::Value;

pub struct NEParse;

impl NEParse {
    pub fn opt_immut_str_to_opt_i32(s: Option<&str>) -> Option<i32> {
        if s != None {
            match s?.to_string().parse::<i32>() {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn opt_immut_str_to_option_string(s: Option<&str>) -> Option<String> {
        if s != None {
            Some(s?.to_string())
        } else {
            None
        }
    }

    pub fn opt_immut_vec_serde_json_value_to_opt_vec_string(
        value: Option<&Vec<Value>>,
    ) -> Option<Vec<String>> {
        if value != None {
            let v: Vec<String> = value
                .into_iter()
                .map(|data| {
                    data.into_iter()
                        .map(|s| s.as_str().unwrap().to_string())
                        .collect()
                })
                .collect();
            Option::from(v)
        } else {
            None
        }
    }

    pub fn opt_immut_str_to_opt_naive_datetime(value: Option<&str>) -> Option<NaiveDateTime> {
        // Check if the Option is Some, and if so, attempt to parse the string
        value.and_then(|v| {
            DateTime::parse_from_rfc3339(v)
                .ok()
                .map(|dt| dt.naive_utc())
        })
    }

    pub fn opt_naive_datetime_to_utc_opt_string(naive: Option<NaiveDateTime>) -> Option<String> {
        if naive != None {
            let datetime_utc = Utc.from_utc_datetime(&naive?);
            Some(datetime_utc.to_rfc3339_opts(SecondsFormat::Secs, true))
        } else {
            None
        }
    }

    pub fn opt_immut_vec_serde_json_value_to_vec_string(value: Option<&Vec<Value>>) -> Vec<String> {
        if value.is_none() {
            vec![]
        } else {
            value
                .unwrap_or(&Vec::new())
                .into_iter()
                .map(|s| s.as_str().unwrap_or("").to_string())
                .collect()
        }
    }
}
