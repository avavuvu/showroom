use crate::theme::Color;

pub const WHITE: Color = Color::hex(0xffffff);
const FALLBACK: Color = Color::hex(0x262626);
const MIN_CONTRAST: f32 = 7.0;
const MAX_LIGHTNESS: f32 = 0.42;
const MAX_CHROMA: f32 = 0.13;
const LIGHTNESS_STEP: f32 = 0.02;
const MAX_STEPS: usize = 60;
const SAMPLE_PX: u32 = 64;
const DARKEST: u8 = 24;
const LIGHTEST: u8 = 232;
const BUCKET_BITS: u8 = 3;

#[derive(Clone, Copy, Default)]
struct Bucket {
    sum: [u32; 3],
    count: u32,
    score: f32,
}

pub fn small_artwork_url(src: &str) -> String {
    let Ok(mut url) = url::Url::parse(src) else { return src.to_string() };
    let host = url.host_str().unwrap_or_default().to_string();
    let path = url.path().to_string();

    let resized = if host.ends_with(".mzstatic.com") {
        path.rsplit_once('/').filter(|(_, last)| last.contains("bb.")).map(|(base, _)| format!("{base}/{SAMPLE_PX}x{SAMPLE_PX}bb.jpg"))
    } else if host.ends_with(".bcbits.com") {
        path.rsplit_once('_').filter(|(_, last)| last.ends_with(".jpg")).map(|(base, _)| format!("{base}_42.jpg"))
    } else if host == "res.cloudinary.com" {
        path.split_once("/image/upload/").map(|(base, rest)| format!("{base}/image/upload/w_{SAMPLE_PX},h_{SAMPLE_PX},c_fill,f_png/{rest}"))
    } else {
        None
    };

    match resized {
        Some(path) => {
            url.set_path(&path);
            url.to_string()
        }
        None => src.to_string(),
    }
}

pub fn from_image(bytes: &[u8]) -> Option<Color> {
    let image = image::load_from_memory(bytes).ok()?;
    let image = if image.width() > SAMPLE_PX * 2 { image.thumbnail(SAMPLE_PX, SAMPLE_PX) } else { image };
    let pixels: Vec<[u8; 3]> = image.to_rgb8().pixels().map(|pixel| pixel.0).collect();
    let color = dominant(&pixels).or_else(|| average(&pixels))?;
    Some(for_light_text(color))
}

pub fn is_card_color(color: Color) -> bool {
    color.contrast(WHITE) >= MIN_CONTRAST - 0.05
}

fn dominant(pixels: &[[u8; 3]]) -> Option<Color> {
    let shift = 8 - BUCKET_BITS;
    let mut buckets = vec![Bucket::default(); 1 << (BUCKET_BITS * 3)];

    for &[red, green, blue] in pixels {
        let (max, min) = (red.max(green).max(blue), red.min(green).min(blue));
        if max < DARKEST || min > LIGHTEST {
            continue;
        }
        let key = ((red >> shift) as usize) << (BUCKET_BITS * 2) | ((green >> shift) as usize) << BUCKET_BITS | (blue >> shift) as usize;
        let bucket = &mut buckets[key];
        bucket.sum[0] += u32::from(red);
        bucket.sum[1] += u32::from(green);
        bucket.sum[2] += u32::from(blue);
        bucket.count += 1;
        bucket.score += 1.0 + 4.0 * f32::from(max - min) / 255.0;
    }

    let best = buckets.iter().filter(|bucket| bucket.count > 0).max_by(|left, right| left.score.total_cmp(&right.score))?;
    Some(mean(best.sum, best.count))
}

fn average(pixels: &[[u8; 3]]) -> Option<Color> {
    let count = u32::try_from(pixels.len()).ok().filter(|count| *count > 0)?;
    let sum = pixels.iter().fold([0u32; 3], |sum, pixel| [sum[0] + u32::from(pixel[0]), sum[1] + u32::from(pixel[1]), sum[2] + u32::from(pixel[2])]);
    Some(mean(sum, count))
}

fn mean(sum: [u32; 3], count: u32) -> Color {
    let channel = |value: u32| (value / count) as u8;
    Color { red: channel(sum[0]), green: channel(sum[1]), blue: channel(sum[2]) }
}

fn for_light_text(color: Color) -> Color {
    let mut lch = color.oklch();
    lch.lightness = lch.lightness.min(MAX_LIGHTNESS);
    lch.chroma = lch.chroma.min(MAX_CHROMA);

    for _ in 0..MAX_STEPS {
        match lch.to_color() {
            None => lch.chroma *= 0.85,
            Some(candidate) if is_card_color(candidate) => return candidate,
            Some(_) => lch.lightness = (lch.lightness - LIGHTNESS_STEP).max(0.0),
        }
    }
    FALLBACK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_a_dark_version_of_the_vivid_colour_white_text_can_sit_on() {
        let mut pixels = vec![[245, 245, 245]; 600];
        pixels.extend(vec![[150, 150, 150]; 250]);
        pixels.extend(vec![[220, 40, 40]; 150]);

        let color = for_light_text(dominant(&pixels).unwrap());
        let lch = color.oklch();

        assert!(is_card_color(color), "{color} is too light for white text");
        assert!(lch.chroma > 0.05, "{color} lost the red");
        assert!((lch.hue.to_degrees() - 29.0).abs() < 20.0, "{color} is not red");
    }
}
