use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime};
use serde::Deserialize;
use std::{fs::File, path::Path};

/* Raw JSON data */

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RawMetaData {
    title: String,
    photo_taken_time: RawPhotoTakenTime,
}

#[derive(Deserialize, Debug)]
struct RawPhotoTakenTime {
    timestamp: String,
}

/* Parsed */

struct MetaData {
    title: String,
    photo_taken_time: NaiveDateTime,
}
impl MetaData {
    fn parse(raw: RawMetaData) -> Result<Self> {
        let offset: i64 = raw.photo_taken_time.timestamp.parse()?;
        let photo_taken_time = DateTime::from_timestamp_secs(offset)
            .ok_or_else(|| {
                anyhow!(
                    "Failed to parse timestamp {}",
                    raw.photo_taken_time.timestamp
                )
            })?
            .naive_utc();

        Ok(Self {
            title: raw.title,
            photo_taken_time,
        })
    }
}

/* API */

pub fn get_takeout_time<A: AsRef<Path>>(
    path: A,
    search_path: Option<A>,
) -> Result<Option<NaiveDateTime>> {
    let path = path.as_ref();
    let image_dir = path
        .parent()
        .ok_or_else(|| anyhow!("Path {} has no parent", path.display()))?;
    let search_path = search_path
        .as_ref()
        .map(|x| x.as_ref())
        .unwrap_or(image_dir);
    let file_name = path
        .file_name()
        .and_then(|x| x.to_str())
        .ok_or_else(|| anyhow!("Couldn't get file name of {}", path.display()))?;

    let meta_file_path = search_path.join(format!("{}.json", file_name));
    if meta_file_path.exists() {
        let f = File::open(meta_file_path)?;
        let meta = MetaData::parse(serde_json::from_reader(f)?)?;
        assert_eq!(meta.title, file_name);
        Ok(Some(meta.photo_taken_time))
    } else {
        Ok(None)
    }
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE_CONTENT: &str = r#"{
  "title": "20160105_133216.jpg",
  "description": "",
  "imageViews": "7",
  "creationTime": {
    "timestamp": "1697966187",
    "formatted": "Oct 22, 2023, 9:16:27 AM UTC"
  },
  "photoTakenTime": {
    "timestamp": "1451997135",
    "formatted": "Jan 5, 2016, 12:32:15 PM UTC"
  },
  "geoData": {
    "latitude": 0.0,
    "longitude": 0.0,
    "altitude": 0.0,
    "latitudeSpan": 0.0,
    "longitudeSpan": 0.0
  },
  "geoDataExif": {
    "latitude": 0.0,
    "longitude": 0.0,
    "altitude": 0.0,
    "latitudeSpan": 0.0,
    "longitudeSpan": 0.0
  },
  "url": "https://photos.google.com/photo/ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefgh",
  "googlePhotosOrigin": {
    "mobileUpload": {
      "deviceFolder": {
        "localFolderName": ""
      },
      "deviceType": "ANDROID_PHONE"
    }
  }
}"#;

    #[test]
    fn test_parse_takeout() {
        let raw: RawMetaData =
            serde_json::from_str(TEST_FILE_CONTENT).expect("Must parse correctly");
        assert_eq!(raw.title, "20160105_133216.jpg");
        assert_eq!(raw.photo_taken_time.timestamp, "1451997135");

        let parsed = MetaData::parse(raw).expect("Parsing of the raw model must succeed");
        let expected = chrono::NaiveDate::from_ymd_opt(2016, 1, 5)
            .unwrap()
            .and_hms_opt(12, 32, 15)
            .unwrap();

        assert_eq!(expected, parsed.photo_taken_time);
    }
}
