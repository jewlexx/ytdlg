use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YtdlManifest {
    pub id: Option<String>,
    pub title: Option<String>,
    pub formats: Vec<Format>,
    pub thumbnails: Option<Vec<Thumbnail>>,
    pub description: Option<String>,
    pub upload_date: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub duration: Option<i64>,
    pub view_count: Option<i64>,
    pub average_rating: Option<serde_json::Value>,
    pub age_limit: Option<i64>,
    pub webpage_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub is_live: Option<serde_json::Value>,
    pub automatic_captions: Option<HashMap<String, Vec<AutomaticCaption>>>,
    pub subtitles: Option<Subtitles>,
    pub chapters: Option<Vec<Chapter>>,
    pub like_count: Option<i64>,
    pub channel: Option<String>,
    pub extractor: Option<String>,
    pub webpage_url_basename: Option<String>,
    pub extractor_key: Option<String>,
    pub playlist: Option<serde_json::Value>,
    pub playlist_index: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    pub display_id: Option<String>,
    pub requested_subtitles: Option<serde_json::Value>,
    pub requested_formats: Option<Vec<Format>>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub resolution: Option<serde_json::Value>,
    pub fps: Option<i64>,
    pub vcodec: Option<String>,
    pub vbr: Option<f64>,
    pub stretched_ratio: Option<serde_json::Value>,
    pub acodec: Option<Acodec>,
    pub abr: Option<f64>,
    pub ext: Option<YtdlManifestExt>,
    pub fulltitle: Option<String>,
    #[serde(rename = "_filename")]
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticCaption {
    pub ext: Option<AutomaticCaptionExt>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub asr: Option<i64>,
    pub filesize: Option<i64>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub fps: Option<i64>,
    pub height: Option<i64>,
    pub quality: Option<i64>,
    pub tbr: Option<f64>,
    pub url: Option<String>,
    pub width: Option<i64>,
    pub ext: Option<YtdlManifestExt>,
    pub vcodec: Option<String>,
    pub acodec: Option<Acodec>,
    pub abr: Option<f64>,
    pub downloader_options: Option<DownloaderOptions>,
    pub container: Option<Container>,
    pub format: Option<String>,
    pub protocol: Option<Protocol>,
    pub http_headers: Option<HttpHeaders>,
    pub vbr: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloaderOptions {
    pub http_chunk_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: Option<String>,
    #[serde(rename = "Accept-Charset")]
    pub accept_charset: Option<AcceptCharset>,
    #[serde(rename = "Accept")]
    pub accept: Option<Accept>,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: Option<AcceptEncoding>,
    #[serde(rename = "Accept-Language")]
    pub accept_language: Option<AcceptLanguage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitles {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub height: Option<i64>,
    pub url: Option<String>,
    pub width: Option<i64>,
    pub resolution: Option<String>,
    pub id: Option<String>,
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
