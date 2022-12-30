use super::Action;
use super::Category;

use std::{cmp, fmt};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub category: Category,
    #[serde(rename = "actionType")]
    pub action: Action,
    pub segment: [f64; 2],
    #[serde(rename = "UUID")]
    pub uuid: String,
    //pub locked: i64,
    //pub votes: i64,
    //pub video_duration: f64,
    //#[serde(rename = "userID", with = "hex")]
    //pub user_id: [u8; 32],
    //pub description: String,
}

pub type Segments = Vec<Segment>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    //#[serde(rename = "videoID")]
    //pub video_id: String,
    #[serde(with = "hex")]
    pub hash: [u8; 32],
    pub segments: Segments,
}

pub type Videos = Vec<Video>;

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.segment[0].partial_cmp(&other.segment[0])
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - {}",
            self.category, self.segment[0], self.segment[1]
        )
    }
}
