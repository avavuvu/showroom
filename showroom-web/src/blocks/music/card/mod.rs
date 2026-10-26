use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

use super::{MusicAttrs, MusicKind, listen_label};
use crate::blocks::card::surface_style;

#[component]
pub fn music_card(#[positional] music: &MusicAttrs, attrs: Attrs) -> Markup {
    let href = music.primary().map(|(_, link)| link.url.as_str());
    let surface = music.surface();
    let style = surface_style(surface);
    html! {
        figure.media-card.music-card.has-color[surface.is_some()] data-kind=(music.kind.as_str()) style=[style] (..attrs) {
            @if let Some(artwork) = &music.artwork {
                a.music-artwork href=[href] target="_blank" rel="noopener noreferrer" tabindex="-1" aria-hidden="true" {
                    img src=(artwork) alt="" loading="lazy";
                }
            }
            div.media-card-details.music-details {
                @if music.kind == MusicKind::Album {
                    p.music-kind { "Album" }
                }
                @if !music.title.is_empty() {
                    p.media-card-title {
                        a href=[href] target="_blank" rel="noopener noreferrer" { (music.title) }
                    }
                }
                @if !music.artist.is_empty() {
                    p.media-card-byline { (music.artist) }
                }
                div.music-listen {
                    span.music-listen-label aria-hidden="true" { "Listen on" }
                    ul.music-links {
                        @for (service, link) in music.links.iter() {
                            li {
                                a href=(link.url) target="_blank" rel="noopener noreferrer" title=(listen_label(service)) {
                                    (service.label())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
