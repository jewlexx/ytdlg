use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YtdlManifest {
    pub id: String,
    pub title: String,
    pub formats: Vec<Format>,
    pub thumbnails: Vec<Thumbnail>,
    pub description: String,
    pub upload_date: String,
    pub uploader: String,
    pub uploader_id: String,
    pub uploader_url: String,
    pub channel_id: String,
    pub channel_url: String,
    pub duration: i64,
    pub view_count: i64,
    pub average_rating: Option<serde_json::Value>,
    pub age_limit: i64,
    pub webpage_url: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub is_live: Option<serde_json::Value>,
    pub automatic_captions: HashMap<String, Vec<AutomaticCaption>>,
    pub subtitles: Subtitles,
    pub chapters: Vec<Chapter>,
    pub like_count: i64,
    pub channel: String,
    pub extractor: String,
    pub webpage_url_basename: String,
    pub extractor_key: String,
    pub playlist: Option<serde_json::Value>,
    pub playlist_index: Option<serde_json::Value>,
    pub thumbnail: String,
    pub display_id: String,
    pub requested_subtitles: Option<serde_json::Value>,
    pub requested_formats: Vec<Format>,
    pub format: String,
    pub format_id: String,
    pub width: i64,
    pub height: i64,
    pub resolution: Option<serde_json::Value>,
    pub fps: i64,
    pub vcodec: String,
    pub vbr: f64,
    pub stretched_ratio: Option<serde_json::Value>,
    pub acodec: Acodec,
    pub abr: f64,
    pub ext: YtdlManifestExt,
    pub fulltitle: String,
    #[serde(rename = "_filename")]
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticCaption {
    pub ext: AutomaticCaptionExt,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub start_time: i64,
    pub end_time: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub asr: Option<i64>,
    pub filesize: Option<i64>,
    pub format_id: String,
    pub format_note: String,
    pub fps: Option<i64>,
    pub height: Option<i64>,
    pub quality: i64,
    pub tbr: f64,
    pub url: String,
    pub width: Option<i64>,
    pub ext: YtdlManifestExt,
    pub vcodec: String,
    pub acodec: Acodec,
    pub abr: Option<f64>,
    pub downloader_options: Option<DownloaderOptions>,
    pub container: Option<Container>,
    pub format: String,
    pub protocol: Protocol,
    pub http_headers: HttpHeaders,
    pub vbr: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloaderOptions {
    pub http_chunk_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept-Charset")]
    pub accept_charset: AcceptCharset,
    #[serde(rename = "Accept")]
    pub accept: Accept,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: AcceptEncoding,
    #[serde(rename = "Accept-Language")]
    pub accept_language: AcceptLanguage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitles {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub height: i64,
    pub url: String,
    pub width: i64,
    pub resolution: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Acodec {
    #[serde(rename = "mp4a.40.2")]
    Mp4A402,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "opus")]
    Opus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AutomaticCaptionExt {
    #[serde(rename = "srv1")]
    Srv1,
    #[serde(rename = "srv2")]
    Srv2,
    #[serde(rename = "srv3")]
    Srv3,
    #[serde(rename = "ttml")]
    Ttml,
    #[serde(rename = "vtt")]
    Vtt,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum YtdlManifestExt {
    #[serde(rename = "m4a")]
    M4A,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "webm")]
    Webm,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Container {
    #[serde(rename = "m4a_dash")]
    M4ADash,
    #[serde(rename = "mp4_dash")]
    Mp4Dash,
    #[serde(rename = "webm_dash")]
    WebmDash,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Accept {
    #[serde(rename = "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")]
    TextHtmlApplicationXhtmlXmlApplicationXmlQ09Q08,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptCharset {
    #[serde(rename = "ISO-8859-1,utf-8;q=0.7,*;q=0.7")]
    Iso88591Utf8Q07Q07,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptEncoding {
    #[serde(rename = "gzip, deflate")]
    GzipDeflate,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptLanguage {
    #[serde(rename = "en-us,en;q=0.5")]
    EnUsEnQ05,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "https")]
    Https,
}
