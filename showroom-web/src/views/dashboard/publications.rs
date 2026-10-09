use boutique::html;
use maud::Markup;
use crate::components::color_input::{DEFAULT_BRAND_COLORS, DEFAULT_INK_COLORS, DEFAULT_PAPER_COLORS, ThemePreview};
use crate::components::dirty_form::DirtyForm;
use crate::components::{ArticleHeader, Button, ColorInput, Input, StyleOverrides, Textarea};
use crate::views::context::PageContext;
use crate::renderer::greeting;
use crate::models::publication::{self, BANNER_HEIGHT, BANNER_WIDTH, DEFAULT_PICTURES, Picture};
use crate::theme::{Font, Layout};
use crate::views::layouts::{page, dashboard_shell};

fn font_list(name: &str, label: &str, current: Font) -> Markup {
    html! {
        fieldset.option-list {
            legend { (label) }
            @for font in Font::ALL {
                label.option style={ "font-family: " (font.stack()) } {
                    input type="radio" name=(name) value=(font.key()) checked[font == current];
                    span { (font.label()) }
                }
            }
        }
    }
}

fn layout_list(current: Layout) -> Markup {
    html! {
        fieldset.option-list.layout-fields {
            legend { "Layout" }
            @for layout in Layout::ALL {
                label.option {
                    input type="radio" name="layout" value=(layout.key()) checked[layout == current];
                    span { (layout.label()) }
                }
            }
        }
    }
}

pub fn new_form(ctx: &PageContext, slug: &str, name: &str, error: Option<&str>) -> Markup {
    let domain = format!(".{}", ctx.urls.domain());

    dashboard_shell(
        page("New publication").class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                form.settings-form method="POST" action={ (ctx.urls.app()) "/new" } {
                    @if let Some(error) = error {
                        p.error { (error) }
                    }
                    Input("name", label = "Name") value=(name) placeholder="My newsletter" required;
                    Input(
                        "slug",
                        label = "Address",
                        suffix = &domain,
                        hint = "Lowercase letters, numbers and hyphens. This cannot be changed later.",
                    )
                        value=(slug)
                        placeholder="my-newsletter"
                        pattern="[a-z0-9-]{3,40}"
                        minlength="3"
                        maxlength="40"
                        required;
                    Button(submit = true) .primary { "Create publication" }
                }
            }
        }
    )
}

fn picture_section(ctx: &PageContext, publication: &publication::Model, dashboard_url: &str) -> Markup {
    let current = publication.picture();

    html! {
        section.settings-section.picture-section {
            h2 { "Picture" }
            p.hint { "Shown on your pages and at the top of your emails." }
            form.settings-form method="POST" action={ (dashboard_url) "/settings/picture" } enctype="multipart/form-data" {
                fieldset.picture-choices {
                    legend { "Choose a picture" }
                    @if let Picture::Upload(public_id) = current {
                        @if !publication.pictures.contains(public_id) {
                            label.picture-choice {
                                input type="radio" name="picture" value=(publication.image) checked;
                                img src=(current.url(&ctx.urls)) alt="Your picture";
                            }
                        }
                    }
                    @for public_id in &publication.pictures.0 {
                        label.picture-choice {
                            input type="radio" name="picture" value=(public_id) checked[current == Picture::Upload(public_id)];
                            img src=(Picture::Upload(public_id).url(&ctx.urls)) alt="A picture you uploaded";
                        }
                    }
                    @for index in 0..DEFAULT_PICTURES {
                        label.picture-choice {
                            input type="radio" name="picture" value=(publication::default_picture(index)) checked[current == Picture::Default(index)];
                            img src=(publication::default_picture_path(index)) alt={ "Picture " (index + 1) };
                        }
                    }
                }
                label {
                    "Or upload your own"
                    input type="file" name="file" accept="image/png,image/jpeg,image/gif,image/webp";
                }
                p.hint { "Square images work best. PNG, JPEG, GIF or WebP, up to 10 MB." }
                Button(submit = true) .primary { "Save picture" }
            }
        }
    }
}

fn banner_section(ctx: &PageContext, publication: &publication::Model, dashboard_url: &str) -> Markup {
    html! {
        section.settings-section.banner-section {
            h2 { "Banner" }
            p.hint {
                "Shown in place of the Showroom logo at the top of your pages. Use a 4:1 image, for example "
                (BANNER_WIDTH) " × " (BANNER_HEIGHT) " pixels. Other sizes are cropped to fit."
            }
            @if let Some(banner) = publication.banner_url(&ctx.urls) {
                img.banner-preview src=(banner) alt="Your banner";
            }
            form.settings-form method="POST" action={ (dashboard_url) "/settings/banner" } enctype="multipart/form-data" {
                input type="file" name="file" accept="image/png,image/jpeg,image/gif,image/webp" required;
                Button(submit = true) .primary { "Upload banner" }
            }
            @if publication.banner.is_some() {
                form.banner-remove method="POST" action={ (dashboard_url) "/settings/banner/remove" } {
                    Button(submit = true) .secondary { "Remove banner" }
                }
            }
        }
    }
}

pub fn settings(ctx: &PageContext, error: Option<&str>) -> Markup {
    let publication = ctx.publication();
    let dashboard_url = ctx.dashboard_url();
    let address = format!("{}.{}", publication.slug, ctx.urls.domain());
    let theme = publication.theme();
    let today = chrono::Utc::now().format("%B %-d, %Y").to_string();

    dashboard_shell(
        page("Settings").class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                h2 { "Details" }
                form.settings-form method="POST" action={ (dashboard_url) "/settings" } bq-setup=(DirtyForm) {
                    @if let Some(error) = error {
                        p.error { (error) }
                    }
                    Input("name", label = "Name") value=(publication.name) required;
                    Textarea("description", label = "Description", value = publication.description.as_deref().unwrap_or("")) rows="3";
                    Input(
                        "greeting",
                        label = "Greeting",
                        hint = "Shown at the top of each newsletter. Write {{name}} where the subscriber's name goes. Subscribers without a name skip a greeting that uses {{name}}. Leave empty to turn the greeting off.",
                    )
                        value=(publication.greeting)
                        placeholder=(greeting::DEFAULT)
                        maxlength=(greeting::MAX_LEN);
                    Input("address", label = "Address") value=(address) disabled;
                    Button(submit = true) .primary { "Save" }
                }
            }

            (picture_section(ctx, publication, &dashboard_url))
            (banner_section(ctx, publication, &dashboard_url))

            section.settings-section.style-section {
                h2 { "Style" }
                form.settings-form
                    method="POST"
                    action={ (dashboard_url) "/settings/style" }
                    bq-setup={ (DirtyForm) " " (ThemePreview) } {
                    ColorInput("ink", label = "Ink", current_color = &theme.ink, presets = DEFAULT_INK_COLORS);
                    ColorInput("paper", label = "Paper", current_color = &theme.paper, presets = DEFAULT_PAPER_COLORS);
                    ColorInput("brand", label = "Brand", current_color = &theme.brand, presets = DEFAULT_BRAND_COLORS);
                    StyleOverrides(&theme);
                    fieldset.font-fields {
                        legend { "Fonts" }
                        (font_list("font_title", "Title", theme.fonts.title))
                        (font_list("font_body", "Body", theme.fonts.body))
                    }
                    (layout_list(theme.layout))
                    Button(submit = true) .primary { "Save" }
                }

                figure.style-preview inert {
                    article.prose.flow {
                        ArticleHeader(
                            date = &today,
                            title = "Marley’s Ghost",
                            subtitle = Some("A Christmas Carol, Stave One"),
                            name = &publication.name,
                            href = "#",
                            picture = &publication.picture_url(&ctx.urls),
                        );
                        p {
                            "MARLEY was dead, to begin with. There is no doubt whatever about that. The register of his "
                            "burial was signed by the clergyman, the clerk, the undertaker, and the chief mourner. Scrooge "
                            "signed it. And Scrooge’s name was good upon ’Change, for anything he chose to put his hand to."
                        }
                        p {
                            "Old Marley was as dead as a "
                            a href="#" { "door-nail" }
                            "."
                        }
                        p {
                            "Mind! I don’t mean to say that I know, of my own knowledge, what there is particularly dead "
                            "about a door-nail. I might have been inclined, myself, to regard a coffin-nail as the deadest "
                            "piece of ironmongery in the trade. But the wisdom of our ancestors is in the simile; and my "
                            "unhallowed hands shall not disturb it, or the Country’s done for. You will therefore permit "
                            "me to repeat, emphatically, that Marley was as dead as a door-nail."
                        }
                        h2 { "Scrooge and Marley" }
                        p {
                            "Scrooge knew he was dead? Of course he did. How could it be otherwise? Scrooge and he were "
                            "partners for I don’t know how many years. Scrooge was his sole executor, his sole "
                            "administrator, his sole assign, his sole residuary legatee, his sole friend, and sole mourner. "
                            "And even Scrooge was not so dreadfully cut up by the sad event, but that he was an excellent "
                            "man of business on the very day of the funeral, and solemnised it with an undoubted bargain."
                        }
                    }
                }
            }

            @if !publication.is_default {
                section.settings-section.danger-zone {
                    h2 { "Delete publication" }
                    p.hint { "This removes the publication, its newsletters and its subscribers. This cannot be undone." }
                    form method="POST" action={ (dashboard_url) "/delete" }
                        onsubmit="return confirm('Delete this publication and everything in it?')" {
                        Button(submit = true) .danger { "Delete " (publication.name) }
                    }
                }
            }
        }
    )
}
