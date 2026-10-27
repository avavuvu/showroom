pub const EMAIL_PROFILE_PX: u32 = 48;

#[derive(Clone, Copy)]
pub enum ImageSize {
    Hero,
    Card,
    Og,
    Banner,
    EmailPicture,
    Picture,
}

impl ImageSize {
    fn transform(self) -> String {
        match self {
            ImageSize::Hero => "c_fill,g_auto,ar_16:9,w_1600".to_string(),
            ImageSize::Card => "c_fill,g_auto,ar_16:9,w_640".to_string(),
            ImageSize::Og => "c_fill,g_auto,w_1200,h_630".to_string(),
            ImageSize::Banner => "c_fill,g_auto,w_1200,h_300".to_string(),
            ImageSize::EmailPicture => {
                let pixels = EMAIL_PROFILE_PX * 2;
                format!("c_fill,g_auto,w_{pixels},h_{pixels}")
            }
            ImageSize::Picture => "c_fill,g_auto,w_256,h_256,f_auto,q_auto".to_string(),
        }
    }
}

pub fn cloudinary_url(cloud_name: &str, public_id: &str, size: ImageSize) -> String {
    format!("https://res.cloudinary.com/{cloud_name}/image/upload/{}/{public_id}", size.transform())
}
