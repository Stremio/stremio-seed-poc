use crate::{PageId, Msg as RootMsg, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use serde::Deserialize;
use localsearch::LocalSearch;
use seed_hooks::{*, topo::nested as view};
use indexmap::{IndexMap, indexmap};
use stremio_core::types::resource::{MetaItemPreview, PosterShape};
use stremio_core::types::addon::{ResourceRequest, ResourceResponse, ResourcePath};
use crate::page;

const SEARCH_DEBOUNCE_TIME: u32 = 0;

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!"); })
}

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    context: &mut Context,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let root_url_base = &context.root_base_url;
    let base_url = url.to_hash_base_url();

    // @TODO load dynamically? (together with the `let resources` below)

    if model.is_none() {
        let video_groups = indexmap!{
            VideoGroupId::CinemetaTopMovie => VideoGroup {
                label: "Cinemeta - top movie".to_owned(),
                videos: Vec::new(),
                see_all_url: RootUrls::new(root_url_base).discover_urls().res_req(&ResourceRequest::new(
                    "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                    ResourcePath::without_extra("catalog", "movie", "top"),
                )),
            },
            VideoGroupId::CinemetaTopSeries => VideoGroup {
                label: "Cinemeta - top series".to_owned(),
                videos: Vec::new(),
                see_all_url: RootUrls::new(root_url_base).discover_urls().res_req(&ResourceRequest::new(
                    "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                    ResourcePath::without_extra("catalog", "series", "top"),
                )),
            },
            VideoGroupId::CinemetaImdbMovie => VideoGroup {
                label: "Cinemeta - imdbRating movie".to_owned(),
                videos: Vec::new(),
                see_all_url: RootUrls::new(root_url_base).discover_urls().res_req(&ResourceRequest::new(
                    "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                    ResourcePath::without_extra("catalog", "movie", "imdbRating"),
                )),
            },
            VideoGroupId::CinemetaImdbSeries => VideoGroup {
                label: "Cinemeta - imdbRating series".to_owned(),
                videos: Vec::new(),
                see_all_url: RootUrls::new(root_url_base).discover_urls().res_req(&ResourceRequest::new(
                    "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                    ResourcePath::without_extra("catalog", "series", "imdbRating"),
                )),
            },
            VideoGroupId::YoutubeTopChannel => VideoGroup {
                label: "YouTube - top channel".to_owned(),
                videos: Vec::new(),
                see_all_url: RootUrls::new(root_url_base).discover_urls().res_req(&ResourceRequest::new(
                    "https://v3-channels.strem.io/manifest.json".parse().expect("valid BASE url"),
                    ResourcePath::without_extra("catalog", "channel", "top"),
                )),
            },
        };

        *model = Some(Model {
            base_url,
            video_groups,
        });

        let resources = vec![
            (VideoGroupId::CinemetaTopMovie, "https://v4-cinemeta.strem.io/catalog/movie/top.json"),
            (VideoGroupId::CinemetaTopSeries, "https://v4-cinemeta.strem.io/catalog/series/top.json"),
            (VideoGroupId::CinemetaImdbMovie, "https://v4-cinemeta.strem.io/catalog/movie/imdbRating.json"),
            (VideoGroupId::CinemetaImdbSeries, "https://v4-cinemeta.strem.io/catalog/series/imdbRating.json"),
            (VideoGroupId::YoutubeTopChannel, "https://v3-channels.strem.io/catalog/channel/top.json"),
        ];
        for (video_group_id, url) in resources.into_iter() {
            orders.perform_cmd(async move { 
                Msg::VideosReceived(video_group_id, get_videos(url).await.unwrap()) 
            });
        }
    }
    Some(PageId::Board)
}

async fn get_videos(url: &str) -> Result<Vec<MetaItemPreview>, FetchError> {
    fetch(url)
        .await?
        .check_status()?
        .json::<FetchedResourceResponse>()
        .await
        .map(|response| response.metas.into_iter().take(10).collect())
}

#[derive(Deserialize)]
struct FetchedResourceResponse {
    metas: Vec<MetaItemPreview>
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    video_groups: IndexMap<VideoGroupId, VideoGroup>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoGroupId {
    CinemetaTopMovie,
    CinemetaTopSeries,
    CinemetaImdbMovie,
    CinemetaImdbSeries,
    YoutubeTopChannel,
}

struct VideoGroup {
    label: String,
    videos: Vec<MetaItemPreview>,
    see_all_url: Url,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    VideosReceived(VideoGroupId, Vec<MetaItemPreview>),
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::VideosReceived(video_group_id, videos) => {
            model.video_groups.get_mut(&video_group_id).unwrap().videos = videos;
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    let page_content = board_content(
        model.video_groups.values(), 
        !model.video_groups.is_empty(),
        &context.root_base_url,
    ).map_msg(msg_mapper);

    page::basic_layout(page::BasicLayoutArgs {
        page_content,
        container_class: "board-container",
        context,
        page_id,
        search_args: None,
    })
}

#[view]
fn board_content<'a>(
    video_groups: impl Iterator<Item = &'a VideoGroup>, 
    videos_loaded: bool, 
    root_base_url: &Url
) -> Node<Msg> {
    div![
        C!["board-content"],
        s()
            .height(pc(100))
            .overflow_y(CssOverflowY::Auto)
            .width(pc(100)),
        if !videos_loaded {
            vec![loading()]
        } else {
            board_rows(video_groups, root_base_url)
        }
    ]
}

#[view]
fn loading() -> Node<Msg> {
    div![
        C!["loading"],
        s()
            .color(Color::SecondaryVariant2Light1_90)
            .font_size(rem(1.8))
            .padding_left(rem(1))
            .margin(rem(2)),
        "Loading..."
    ]
}

#[view]
fn board_rows<'a>(video_groups: impl Iterator<Item = &'a VideoGroup>, root_base_url: &Url) -> Vec<Node<Msg>> {
    video_groups.enumerate().map(|(index, group)| board_row(index, group, root_base_url)).collect()
}

#[view]
fn board_row(index: usize, group: &VideoGroup, root_base_url: &Url) -> Node<Msg> {
    div![
        C!["board-row", "board-row-poster", "meta-row-container"],
        s()
            .margin("4rem 2rem")
            .overflow(CssOverflow::Visible),
        IF!(index == 0 => s().margin_top(rem(2))),
        board_row_header_container(group),
        board_row_meta_items_container(group, root_base_url),
    ]
}

#[view]
fn board_row_header_container(group: &VideoGroup) -> Node<Msg> {
    let see_all_title = "SEE ALL";
    div![
        C!["header-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .justify_content(CssJustifyContent::FlexEnd)
            .margin_bottom(rem(1))
            .padding("0 1rem"),
        div![
            C!["title-container"],
            s()
                .color(Color::SecondaryVariant2Light1_90)
                .flex("1")
                .font_size(rem(1.8))
                .max_height(em(2.4)),
            attrs!{
                At::Title => &group.label,
            },
            &group.label,
        ],
        a![
            C!["see-all-container", "button-container"],
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex(CssFlex::None)
                .flex_direction(CssFlexDirection::Row)
                .max_width(rem(12))
                .padding(rem(0.2))
                .cursor(CssCursor::Pointer),
            s()
                .style_other(":hover > .label")
                .color(Color::SecondaryVariant2Light2_90),
            s()
                .style_other(":hover > .icon")
                .fill(Color::SecondaryVariant2Light2_90),
            attrs!{
                At::TabIndex => 0,
                At::Title => see_all_title,
                At::Href => group.see_all_url,
            },
            div![
                C!["label"],
                s()
                    .color(Color::SecondaryVariant2Light1_90)
                    .flex("0 1 auto")
                    .font_size(rem(1.3))
                    .font_weight("500")
                    .max_height(em(1.2))
                    .text_transform(CssTextTransform::Uppercase),
                see_all_title,
            ],
            see_all_icon(),
        ]
    ]
}

#[view]
fn see_all_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .overflow(CssOverflow::Visible)
            .fill(Color::SecondaryVariant2Light1_90)
            .flex(CssFlex::None)
            .height(rem(1.3))
            .margin_left(rem(0.5)),
        attrs!{
            At::ViewBox => "0 0 565 1024",
            At::from("icon") => "ic_arrow_thin_right",
        },
        path![
            attrs!{
                At::D => "M84.932 14.155l465.016 463.511c8.963 8.73 14.578 20.859 14.757 34.301l0 0.033c-0.021 13.598-5.67 25.873-14.743 34.621l-0.015 0.014-464.113 463.209c-9.052 8.82-21.434 14.26-35.087 14.26s-26.035-5.44-35.098-14.27l0.011 0.010c-9.355-8.799-15.292-21.14-15.66-34.87l-0.001-0.066c-0.001-0.103-0.001-0.225-0.001-0.348 0-13.437 5.534-25.582 14.448-34.278l0.010-0.009 430.080-428.273-429.779-427.972c-9.101-8.684-14.76-20.907-14.76-34.451 0-0.171 0.001-0.341 0.003-0.511l-0 0.026c-0-0.043-0-0.094-0-0.145 0-13.595 5.526-25.899 14.455-34.789l0.002-0.002c9.099-8.838 21.532-14.287 35.238-14.287s26.138 5.449 35.25 14.299l-0.012-0.012z",
            }
        ]
    ]
}

#[view]
fn board_row_meta_items_container(group: &VideoGroup, root_base_url: &Url) -> Node<Msg> {
    div![
        C!["meta-items-container"],
        s()
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .overflow(CssOverflow::Visible),
        group.videos.iter().map(|video| meta_item(video, root_base_url)),
        (0..10 - group.videos.len()).map(|_| dummy_meta_item()),
    ]
}

#[view]
fn meta_item(video: &MetaItemPreview, root_base_url: &Url) -> Node<Msg> {
    a![
        el_key(&video.id),
        C!["meta-item", "poster-shape-poster", "meta-item-container", "button-container"],
        s()
            .flex(format!("calc(1 / {});", global::POSTER_SHAPE_RATIO).as_str())
            .padding(rem(1))
            .overflow(CssOverflow::Visible)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight3)
            .transition("background-color 100ms ease-out"),
        attrs!{
            At::TabIndex => 0,
            At::Title => video.name,
            At::Href => RootUrls::new(root_base_url).detail_urls().without_video_id(&video.r#type, &video.id),
        },
                video.poster.as_ref().map(poster_container),
        div![
            C!["title-bar-container"],
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .height(rem(2.8))
                .overflow(CssOverflow::Visible),
            div![
                C!["title-label"],
                s()
                    .padding_right(rem(0.5))
                    .color(Color::SurfaceLight5_90)
                    .flex("1")
                    .max_height(em(2.4))
                    .padding_left(rem(0.5)),
                &video.name,
            ]
        ]
    ]
}

#[view]
fn dummy_meta_item() -> Node<Msg> {
    div![
        C!["meta-item", "poster-shape-poster"],
        s()
            .flex(format!("calc(1 / {});", global::POSTER_SHAPE_RATIO).as_str())
            .padding(rem(1))
    ]
}

#[view]
fn poster_container(poster: &String) -> Node<Msg> {
    div![
        C!["poster-container"],
        s()
            .padding_top(format!("calc(100% * {})", global::POSTER_SHAPE_RATIO).as_str())
            .background_color(Color::Background)
            .position(CssPosition::Relative)
            .z_index("0"),
        div![
            C!["poster-image-layer"],
            s()
                .align_items(CssAlignItems::Center)
                .bottom("0")
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .justify_content(CssJustifyContent::Center)
                .left("0")
                .position(CssPosition::Absolute)
                .right("0")
                .top("0")
                .z_index("-3"),
            img![
                C!["poster-image"],
                s()
                    .flex(CssFlex::None)
                    .height(pc(100))
                    .object_fit("cover")
                    .object_position("center")
                    .opacity("0.9")
                    .width(pc(100)),
                attrs!{
                    At::Alt => " ",
                    At::Src => poster,
                },
            ]
        ]
    ]
}
