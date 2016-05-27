
extern crate rustc_serialize;

use std::cmp::Ord;
use std::cmp::Ordering;
use rustc_serialize::json::Json;

pub struct DataItem {
    pub number: u64,
    pub created: String,
    pub project: String,
    pub subject: String,
    pub status: String,
}

impl DataItem {
    pub fn new(json: &Json) -> DataItem {
        let mut created = json["created"].as_string().unwrap().to_owned();
        created.truncate(10);
        DataItem {
            number: json["_number"].as_u64().unwrap(),
            created: created,
            project: json["project"].as_string().unwrap().to_owned(),
            subject: json["subject"].as_string().unwrap().to_owned(),
            status: json["status"].as_string().unwrap().to_owned(),
        }
    }
}

impl Ord for DataItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.created < other.created {
            Ordering::Less
        } else if self.created > other.created {
            Ordering::Greater
        } else {
            // create time is equal, cmp with project
            if self.project < other.project {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl PartialOrd for DataItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DataItem {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for DataItem {}
