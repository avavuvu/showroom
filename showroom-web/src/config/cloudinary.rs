use sha1::{Digest, Sha1};
use url::Url;

#[derive(Clone)]
pub struct CloudinaryConfig {
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
}

impl CloudinaryConfig {
    pub fn from_env() -> Self {
        let raw = std::env::var("CLOUDINARY_URL").expect("CLOUDINARY_URL must be set");
        let url = Url::parse(&raw).expect("invalid CLOUDINARY_URL");

        let api_key = url.username().to_string();
        let api_secret = url.password().expect("CLOUDINARY_URL missing api_secret").to_string();
        let cloud_name = url.host_str().expect("CLOUDINARY_URL missing cloud_name").to_string();

        Self { cloud_name, api_key, api_secret }
    }

    pub fn sign(&self, timestamp: i64) -> String {
        let to_sign = format!("timestamp={timestamp}{}", self.api_secret);
        let mut hasher = Sha1::new();
        hasher.update(to_sign.as_bytes());
        hex::encode(hasher.finalize())
    }
}
