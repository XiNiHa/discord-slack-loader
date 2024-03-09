use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde_with::chrono_0_4::datetime_utc_ts_seconds_from_any;

use super::user::UserId;

#[derive(Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    pub is_private: bool,
    pub name: String,
    pub creator: UserId,
    pub is_archived: bool,
    pub members: Vec<UserId>,
    pub topic: ChannelTopic,
    pub purpose: ChannelPurpose,
}

#[derive(Deserialize)]
pub struct ChannelTopic {
    pub value: String,
    pub creator: UserId,
    #[serde(with = "ts_seconds")]
    pub last_set: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ChannelPurpose {
    pub value: String,
    pub creator: UserId,
    #[serde(with = "ts_seconds")]
    pub last_set: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct Message {
    pub user: UserId,
    pub text: String,
    #[serde(with = "datetime_utc_ts_seconds_from_any")]
    pub ts: DateTime<Utc>,
    pub subtype: Option<MessageSubType>,
}

#[derive(Deserialize)]
pub enum MessageSubType {
    #[serde(rename = "channel_join")]
    ChannelJoin,
    #[serde(rename = "channel_topic")]
    ChannelTopic,
    #[serde(rename = "channel_purpose")]
    ChannelPurpose,
    #[serde(rename = "tombstone")]
    Tombstone,
    #[serde(rename = "thread_broadcast")]
    ThreadBroadcast,
}
