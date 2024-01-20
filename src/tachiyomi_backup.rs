use ureq::serde::{Deserialize, Serialize};

/// serial name 'eu.kanade.tachiyomi.data.backup.models.Backup'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Backup {
    #[prost(message, repeated, tag = "1")]
    pub backup_manga: ::prost::alloc::vec::Vec<BackupManga>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "2")]
    pub backup_categories: ::prost::alloc::vec::Vec<BackupCategory>,
    /// WARNING: a default value decoded when value is missing
    ///   repeated BrokenBackupSource backupBrokenSources = 100;
    ///   // WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "101")]
    pub backup_sources: ::prost::alloc::vec::Vec<BackupSource>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "104")]
    pub backup_preferences: ::prost::alloc::vec::Vec<BackupPreference>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "105")]
    pub backup_source_preferences: ::prost::alloc::vec::Vec<BackupSourcePreferences>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupManga'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupManga {
    #[prost(int64, required, tag = "1")]
    pub source: i64,
    #[prost(string, required, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "4")]
    pub artist: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "5")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, repeated, tag = "7")]
    pub genre: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "8")]
    pub status: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "9")]
    pub thumbnail_url: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "13")]
    pub date_added: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "14")]
    pub viewer: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "16")]
    pub chapters: ::prost::alloc::vec::Vec<BackupChapter>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, repeated, packed = "false", tag = "17")]
    pub categories: ::prost::alloc::vec::Vec<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "18")]
    pub tracking: ::prost::alloc::vec::Vec<BackupTracking>,
    /// WARNING: a default value decoded when value is missing
    #[prost(bool, optional, tag = "100")]
    pub favorite: ::core::option::Option<bool>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "101")]
    pub chapter_flags: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    ///   repeated BrokenBackupHistory brokenHistory = 102;
    ///   // WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "103")]
    pub viewer_flags: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(message, repeated, tag = "104")]
    pub history: ::prost::alloc::vec::Vec<BackupHistory>,
    /// WARNING: a default value decoded when value is missing
    #[prost(enumeration = "UpdateStrategy", optional, tag = "105")]
    pub update_strategy: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "106")]
    pub last_modified_at: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "107")]
    pub favorite_modified_at: ::core::option::Option<i64>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupCategory'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupCategory {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "2")]
    pub order: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "100")]
    pub flags: ::core::option::Option<i64>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupSource'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupSource {
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, required, tag = "2")]
    pub source_id: i64,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupPreference'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupPreference {
    #[prost(string, required, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, required, tag = "2")]
    pub value: PreferenceValue,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupSourcePreferences'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupSourcePreferences {
    #[prost(string, required, tag = "1")]
    pub source_key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub prefs: ::prost::alloc::vec::Vec<BackupPreference>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupChapter'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupChapter {
    #[prost(string, required, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "3")]
    pub scanlator: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(bool, optional, tag = "4")]
    pub read: ::core::option::Option<bool>,
    /// WARNING: a default value decoded when value is missing
    #[prost(bool, optional, tag = "5")]
    pub bookmark: ::core::option::Option<bool>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "6")]
    pub last_page_read: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "7")]
    pub date_fetch: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "8")]
    pub date_upload: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(float, optional, tag = "9")]
    pub chapter_number: ::core::option::Option<f32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "10")]
    pub source_order: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "11")]
    pub last_modified_at: ::core::option::Option<i64>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupTracking'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupTracking {
    #[prost(int32, required, tag = "1")]
    pub sync_id: i32,
    #[prost(int64, required, tag = "2")]
    pub library_id: i64,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "3")]
    pub media_id_int: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "4")]
    pub tracking_url: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(string, optional, tag = "5")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// WARNING: a default value decoded when value is missing
    #[prost(float, optional, tag = "6")]
    pub last_chapter_read: ::core::option::Option<f32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "7")]
    pub total_chapters: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(float, optional, tag = "8")]
    pub score: ::core::option::Option<f32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int32, optional, tag = "9")]
    pub status: ::core::option::Option<i32>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "10")]
    pub started_reading_date: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "11")]
    pub finished_reading_date: ::core::option::Option<i64>,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "100")]
    pub media_id: ::core::option::Option<i64>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BackupHistory'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BackupHistory {
    #[prost(string, required, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, required, tag = "2")]
    pub last_read: i64,
    /// WARNING: a default value decoded when value is missing
    #[prost(int64, optional, tag = "3")]
    pub read_duration: ::core::option::Option<i64>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.PreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PreferenceValue {
    #[prost(string, required, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// decoded as message with one of these types:
    ///    message BooleanPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.BooleanPreferenceValue'
    ///    message FloatPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.FloatPreferenceValue'
    ///    message IntPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.IntPreferenceValue'
    ///    message LongPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.LongPreferenceValue'
    ///    message StringPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.StringPreferenceValue'
    ///    message StringSetPreferenceValue, serial name 'eu.kanade.tachiyomi.data.backup.models.StringSetPreferenceValue'
    #[prost(bytes = "vec", required, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.BooleanPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct BooleanPreferenceValue {
    #[prost(bool, required, tag = "1")]
    pub value: bool,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.FloatPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct FloatPreferenceValue {
    #[prost(float, required, tag = "1")]
    pub value: f32,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.IntPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct IntPreferenceValue {
    #[prost(int32, required, tag = "1")]
    pub value: i32,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.LongPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct LongPreferenceValue {
    #[prost(int64, required, tag = "1")]
    pub value: i64,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.StringPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct StringPreferenceValue {
    #[prost(string, required, tag = "1")]
    pub value: ::prost::alloc::string::String,
}

/// serial name 'eu.kanade.tachiyomi.data.backup.models.StringSetPreferenceValue'
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct StringSetPreferenceValue {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

/// serial name 'eu.kanade.tachiyomi.source.model.UpdateStrategy'
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateStrategy {
    AlwaysUpdate = 0,
    OnlyFetchOnce = 1,
}

impl UpdateStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateStrategy::AlwaysUpdate => "ALWAYS_UPDATE",
            UpdateStrategy::OnlyFetchOnce => "ONLY_FETCH_ONCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALWAYS_UPDATE" => Some(Self::AlwaysUpdate),
            "ONLY_FETCH_ONCE" => Some(Self::OnlyFetchOnce),
            _ => None,
        }
    }
}
