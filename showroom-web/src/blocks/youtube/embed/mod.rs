use boutique::html;
use bq_components::component;
use maud::Markup;

use super::{WATCH_LABEL, YoutubeAttrs, play_badge};
use crate::blocks::card::surface_style;

#[component(setup = "youtube-embed")]
pub fn youtube_embed(#[positional] video: &YoutubeAttrs) -> Markup {
    let surface = video.surface();
    let label = if video.title.is_empty() { "Play video".to_string() } else { format!("Play {}", video.title) };
    html! {
        figure.media-card.youtube-card.is-short[video.short].has-color[surface.is_some()]
            style=[surface_style(surface)]
            data-embed=(video.embed_url())
            bq-setup
        {
            div.youtube-frame {
                a.youtube-poster
                    href=(video.watch_url())
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label=(label)
                    title=(WATCH_LABEL)
                    bq-ref="poster"
                    bq-on:click|left|exact|prevent="play"
                    bq-on:pointerover|once="warm"
                    bq-on:focus|once="warm"
                {
                    img src=(video.thumbnail_url()) alt="" loading="lazy" decoding="async";
                    (play_badge())
                }
            }
            @if video.has_details() {
                div.media-card-details {
                    @if !video.title.is_empty() {
                        p.media-card-title {
                            a href=(video.watch_url()) target="_blank" rel="noopener noreferrer" { (video.title) }
                        }
                    }
                    @if !video.channel.is_empty() {
                        p.media-card-byline {
                            @if let Some(channel_url) = &video.channel_url {
                                a href=(channel_url) target="_blank" rel="noopener noreferrer" { (video.channel) }
                            } @else {
                                (video.channel)
                            }
                        }
                    }
                }
            }
        }
    }
}
