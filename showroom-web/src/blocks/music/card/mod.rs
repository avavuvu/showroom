use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

use super::{MusicAttrs, MusicKind, listen_label};

#[component]
pub fn music_card(#[positional] music: &MusicAttrs, attrs: Attrs) -> Markup {
    let href = music.primary().map(|(_, link)| link.url.as_str());
    let surface = music.surface();
    let style = surface.map(|surface| format!("--music-surface:{surface}"));
    html! {
        figure.music-card.has-color[surface.is_some()] data-kind=(music.kind.as_str()) style=[style] (..attrs) {
            @if let Some(artwork) = &music.artwork {
                a.music-artwork href=[href] target="_blank" rel="noopener noreferrer" tabindex="-1" aria-hidden="true" {
                    img src=(artwork) alt="" loading="lazy";
                }
            }
            div.music-details {
                @if music.kind == MusicKind::Album {
                    p.music-kind { "Album" }
                }
                @if !music.title.is_empty() {
                    p.music-title {
                        a href=[href] target="_blank" rel="noopener noreferrer" { (music.title) }
                    }
                }
                @if !music.artist.is_empty() {
                    p.music-artist { (music.artist) }
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
