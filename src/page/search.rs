use crate::{PageId, Msg as RootMsg, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use serde::Deserialize;
use localsearch::LocalSearch;
use seed_hooks::{*, topo::nested as view};
use crate::basic_layout::{basic_layout, BasicLayoutArgs, SearchArgs};
use std::rc::Rc;
use stremio_core::types::addon::{ResourceRequest, ResourceResponse, ResourcePath};

const SEARCH_DEBOUNCE_TIME: u32 = 0;

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); })
}

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();
    let search_query = url.next_hash_path_part().map(ToOwned::to_owned);
    let input_search_query = search_query.clone().unwrap_or_default();

    if let Some(model) = model {
        if model.search_query != search_query {
            model.input_search_query = input_search_query;
            model.search_query = search_query;
            orders.send_msg(Msg::Search);
        }
    } else {
        *model = Some(Model {
            base_url,
            input_search_query,
            search_query,
            debounced_search_query_change: None,
            video_groups: Vec::new(),
            search_results: Vec::new(),
        });
        orders.perform_cmd(async { 
            Msg::VideosReceived(get_videos().await.unwrap()) 
        });
    }
    Some(PageId::Search)
}

async fn get_videos() -> Result<Vec<Video>, FetchError> {
    fetch("/data/cinemeta_20_000.json")
        .await?
        .check_status()?
        .json::<Vec<Video>>()
        .await
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    input_search_query: String,
    search_query: Option<String>,
    debounced_search_query_change: Option<CmdHandle>,
    video_groups: Vec<VideoGroup>,
    search_results: Vec<VideoGroupResults>,
}

struct VideoGroup {
    label: String,
    videos: LocalSearch<Video>,
    see_all_url: Url,
}

#[derive(Debug)]
struct VideoGroupResults {
    label: String,
    videos: Vec<Video>,
    see_all_url: Url,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
    name: String,
    poster: String,
    r#type: String,
    imdb_rating: f64,
    popularity: f64,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn query(self, query: &str) -> Url {
        self.base_url().add_hash_path_part(query)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    SearchQueryInputChanged(String),
    UpdateSearchQuery,
    VideosReceived(Vec<Video>),
    Search,
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SearchQueryInputChanged(query) => {
            model.input_search_query = query;
            model.debounced_search_query_change = Some(
                orders.perform_cmd_with_handle(cmds::timeout(SEARCH_DEBOUNCE_TIME, || Msg::UpdateSearchQuery))
            );
        },
        Msg::UpdateSearchQuery => {
            orders.request_url(Urls::new(&model.base_url).query(&model.input_search_query));
        }
        Msg::VideosReceived(videos) => {
            let mut cinemeta_top_movie = Vec::new();
            let mut cinemeta_top_series = Vec::new();

            for video in videos {
                match video.r#type.as_str() {
                    "movie" => cinemeta_top_movie.push(video),  
                    "series" => cinemeta_top_series.push(video),
                    unknown => {
                        log!("Unhandled MetaItem type:", unknown);
                    }
                }
            }
            model.video_groups = vec![
                VideoGroup {
                    label: "Cinemeta - top movie".to_owned(),
                    videos: index(cinemeta_top_movie),
                    see_all_url: RootUrls::new(&context.root_base_url).discover_urls().res_req(&ResourceRequest::new(
                        "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                        ResourcePath::without_extra("catalog", "movie", "top"),
                    )),
                },
                VideoGroup {
                    label: "Cinemeta - top series".to_owned(),
                    videos: index(cinemeta_top_series),
                    see_all_url: RootUrls::new(&context.root_base_url).discover_urls().res_req(&ResourceRequest::new(
                        "https://v4-cinemeta.strem.io/manifest.json".parse().expect("valid BASE url"),
                        ResourcePath::without_extra("catalog", "series", "top"),
                    )),
                },
            ];
            orders.send_msg(Msg::Search);
        }
        Msg::Search => {
            let mut search_results = Vec::new();
            if let Some(search_query) = &model.search_query {
                for group in &model.video_groups {

                    let group_results = group
                        .videos
                        .search(search_query, 10)
                        .into_iter()
                        .map(|(video, _)| video.clone())
                        .collect::<Vec<_>>();

                    if !group_results.is_empty() {
                        search_results.push(VideoGroupResults {
                            label: group.label.clone(),
                            videos: group_results,
                            see_all_url: group.see_all_url.clone(),
                        });
                    }
                }
            }
            model.search_results = search_results;
        }
    }
}

fn index(videos: Vec<Video>) -> LocalSearch<Video> {
    let max_imdb_rating = 10.;
    let imdb_rating_weight = 1.0;
    let popularity_weight = 1.0;
    let score_threshold = 0.48;

    let max_popularity = videos
        .iter()
        .map(|video| video.popularity)
        .max_by(|popularity_a, popularity_b| popularity_a.partial_cmp(popularity_b).unwrap())
        .unwrap_or_default();

    let boost_computer = move |video: &Video| {
        let imdb_rating_boost = (video.imdb_rating / max_imdb_rating * imdb_rating_weight).exp();
        let popularity_boost = (video.popularity / max_popularity * popularity_weight).exp();
        imdb_rating_boost * popularity_boost
    };

    LocalSearch::builder(videos, |video: &Video| &video.name)
        .boost_computer(boost_computer)
        .score_threshold(score_threshold)
        .build()
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    let page_content = search_content(
        &model.search_results, 
        !model.video_groups.is_empty(),
        &context.root_base_url,
    ).map_msg(msg_mapper);
    
    basic_layout(BasicLayoutArgs {
        page_content,
        container_class: "search-container",
        context,
        page_id,
        search_args: Some(SearchArgs {
            input_search_query: &model.input_search_query,
            on_search_query_input_changed: Rc::new(move |query| msg_mapper(Msg::SearchQueryInputChanged(query))),
            on_search: Rc::new(move || msg_mapper(Msg::Search)),
        }),
    })
}

#[view]
fn search_content(search_results: &[VideoGroupResults], videos_loaded: bool, root_base_url: &Url) -> Node<Msg> {
    div![
        C!["search-content"],
        s()
            .height(pc(100))
            .overflow_y(CssOverflowY::Auto)
            .width(pc(100)),
        if !videos_loaded {
            vec![loading()]
        } else if search_results.is_empty() {
            vec![search_hints_container()]
        } else {
            search_rows(search_results, root_base_url)
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
fn search_hints_container() -> Node<Msg> {
    div![
        C!["search-hints-container"],
        s()
            .align_content(CssAlignContent::FlexStart)
            .align_items(CssAlignItems::FlexStart)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .justify_content(CssJustifyContent::FlexStart)
            .padding(rem(4)),
        search_hint_container(
            "Search for movies, series, YouTube and TV channels",
            "ic_movies",
            "0 0 840 1024",
            vec![
                path![attrs!{At::D => "M813.176 1024h-708.969c-14.3-3.367-24.781-16.017-24.781-31.115 0-0.815 0.031-1.623 0.090-2.422l-0.006 0.107q0-215.642 0-430.984v-4.819c0.015 0 0.033 0 0.051 0 30.976 0 58.991-12.673 79.146-33.116l0.013-0.013c19.218-19.773 31.069-46.796 31.069-76.586 0-1.134-0.017-2.265-0.051-3.391l0.004 0.165h649.939v558.381c-1.037 2.541-2.047 4.621-3.168 6.63l0.157-0.306c-4.8 8.938-13.235 15.394-23.273 17.431l-0.219 0.037zM796.612 481.882h-126.795c-1.944 0.438-3.547 1.646-4.5 3.28l-0.018 0.033-60.235 95.473c-0.466 0.866-0.972 1.957-1.422 3.076l-0.084 0.237h128.301c3.012 0 3.915 0 5.421-3.313l56.922-95.172c0.887-1.056 1.687-2.24 2.356-3.505l0.053-0.11zM393.638 583.078h128.602c0.156 0.017 0.337 0.026 0.52 0.026 2.3 0 4.246-1.517 4.892-3.604l0.010-0.036c18.974-30.118 37.948-62.645 56.621-94.268l2.711-4.518h-125.892c-0.179-0.018-0.387-0.028-0.597-0.028-2.519 0-4.694 1.473-5.711 3.604l-0.016 0.038-58.428 94.268zM377.675 481.882h-126.193c-0.024-0-0.052-0.001-0.080-0.001-2.57 0-4.763 1.609-5.629 3.875l-0.014 0.041-58.428 93.064-2.711 4.216h124.386c0.165 0.018 0.357 0.028 0.551 0.028 2.127 0 3.968-1.225 4.856-3.008l0.014-0.031 60.235-95.473z"}],
                path![attrs!{At::D => "M707.464 0c4.931 1.519 9.225 3.567 13.143 6.142l-0.192-0.119c4.632 3.831 8.386 8.548 11.033 13.909l0.11 0.247c18.372 44.574 36.442 90.353 54.814 134.325l-602.353 243.652c-18.275-41.26-58.864-69.523-106.054-69.523-14.706 0-28.77 2.745-41.71 7.75l0.79-0.269c-4.819-12.047-10.842-24.094-14.758-37.045-0.883-2.705-1.392-5.818-1.392-9.050 0-13.254 8.561-24.508 20.455-28.534l0.212-0.062c18.673-6.626 39.153-14.456 58.428-20.48l542.118-217.751 43.972-19.275 10.24-3.915zM123.181 271.059h1.807l93.064 67.464c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 90.353-35.84 26.504-10.842-2.409-1.807-91.859-65.656c-0.846-0.572-1.889-0.914-3.012-0.914s-2.166 0.341-3.031 0.926l0.019-0.012-77.402 30.118zM535.793 214.739l-2.711-2.108-90.353-66.56c-0.933-0.622-2.080-0.993-3.313-0.993s-2.38 0.371-3.335 1.007l0.022-0.014-118.061 45.779 2.108 1.807 92.461 67.162c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 87.341-34.635zM730.353 135.529h-1.807l-91.859-68.969c-0.803-0.547-1.794-0.874-2.861-0.874s-2.059 0.327-2.879 0.885l0.018-0.011-90.353 36.744c-8.433 3.012-16.565 6.325-24.998 9.939l2.409 2.108 90.353 65.355c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 75.294-30.118z"}],
                path![attrs!{At::D => "M0 433.393c0-3.614 1.506-7.228 2.409-10.541 8.935-34.682 39.932-59.894 76.818-59.894 4.782 0 9.465 0.424 14.014 1.236l-0.48-0.071c37.902 5.909 66.564 38.317 66.564 77.421 0 2.432-0.111 4.839-0.328 7.214l0.023-0.305c-3.944 40.578-37.878 72.037-79.159 72.037-39.144 0-71.681-28.287-78.286-65.534l-0.070-0.48c-0.474-1.046-0.977-1.935-1.547-2.775l0.041 0.064z"}],
            ]
        ),
        search_hint_container(
            "Search for actors, directors and writers",
            "ic_actor",
            "0 0 1085 1024",
            vec![
                path![attrs!{At::D => "M1079.416 397.252c-11.403-64.785-36.251-122.282-71.634-171.727l0.858 1.261c-55.818-86.588-135.669-153.436-230.111-191.866l-3.301-1.188c-51.351-21.358-111-33.763-173.546-33.763-1.882 0-3.762 0.011-5.639 0.034l0.286-0.003c-74.242 1.759-143.267 22.563-202.841 57.688l1.956-1.067c-2.088 1.58-4.728 2.53-7.59 2.53-0.616 0-1.221-0.044-1.814-0.129l0.068 0.008c-16.962-3.079-37.648-5.545-58.648-6.848l-1.588-0.079c-9.62-0.825-20.817-1.296-32.124-1.296-48.771 0-95.497 8.756-138.692 24.781l2.759-0.897c-55.32 21.387-99.741 60.74-127.065 110.769l-0.634 1.268c0 2.409-3.915 5.12-1.807 7.529s5.12 0 7.529 0c20.216-6.76 44.065-12.15 68.646-15.176l1.829-0.184c2.919-0.427 6.289-0.67 9.716-0.67 9.865 0 19.258 2.018 27.79 5.664l-0.462-0.175c1.807 0 4.216 2.108 3.915 4.819s-2.409 2.409-4.216 3.012-11.746 5.12-17.468 8.132c-57.246 31.332-98.926 85.046-113.552 149.064l-0.293 1.524c-6.173 26.883-9.711 57.753-9.711 89.449s3.538 62.567 10.24 92.237l-0.529-2.788c20.112 99.687 51.459 188.161 93.388 270.336l-2.734-5.903c0 1.807 0 4.518 4.819 3.614 0.069-1.080 0.109-2.343 0.109-3.614s-0.039-2.534-0.117-3.786l0.009 0.172c-2.122-23.756-3.332-51.39-3.332-79.306 0-16.916 0.444-33.729 1.322-50.427l-0.098 2.335c2.143-41.776 8.279-81.046 18.068-118.845l-0.901 4.097c6.237-25.012 15.119-46.977 26.591-67.286l-0.69 1.328c10.556 50.436 44.321 91.249 89.362 111.342l0.991 0.395c6.927 3.915 9.939 2.108 10.842-5.421 2.446-16.541 6.335-31.358 11.641-45.481l-0.497 1.509c24.206-77.879 83.745-138.211 159.382-163.042l1.748-0.497c13.713-5.728 29.646-9.055 46.357-9.055 21.655 0 42.004 5.588 59.685 15.4l-0.63-0.321c30.563 19.089 55.771 43.912 74.731 73.162l0.563 0.928c29.693 44.54 54.732 95.808 72.53 150.348l1.258 4.456c3.614 10.24 4.518 10.842 13.252 4.819 37.504-25.775 69.958-54.976 98.226-87.878l0.56-0.667c35.014-36.387 56.734-85.784 57.223-140.253l0.001-0.096c0-5.12 2.108-5.722 6.024-3.614 11.716 5.659 21.692 13.036 30.070 21.935l0.048 0.051c22.879 25.437 41.269 55.452 53.583 88.431l0.629 1.922c30.128 75.686 53.532 163.968 66.179 255.684l0.682 6.038c0 3.313 0 7.831 3.614 8.734s4.216-3.915 5.722-6.626c25.167-40.726 44.986-87.981 56.877-138.3l0.648-3.253c10.527-41.368 16.569-88.858 16.569-137.759 0-30.89-2.411-61.216-7.054-90.802l0.424 3.281z"}],
                path![attrs!{At::D => "M756.555 634.278c-77.097 7.493-140.17 60.141-162.865 130.873l-0.372 1.343c-3.012 7.529-4.819 9.638-12.649 5.421-7.816-4.402-17.158-6.995-27.106-6.995s-19.29 2.593-27.388 7.14l0.282-0.145c-9.035 4.518-10.541 0-13.252-6.325-27.343-76.927-99.515-131.018-184.32-131.018-107.785 0-195.162 87.377-195.162 195.162 0 0.002 0 0.004 0 0.006l-0-0c0.177 107.652 87.486 194.852 195.162 194.852 58.836 0 111.592-26.036 147.374-67.215l0.203-0.239c29.71-32.853 47.891-76.621 47.891-124.636 0-0.442-0.002-0.883-0.005-1.324l0 0.068c-0.165-1.105-0.259-2.379-0.259-3.676 0-8.942 4.479-16.837 11.315-21.565l0.087-0.057c5.139-3.437 11.459-5.485 18.258-5.485 5.541 0 10.765 1.36 15.354 3.765l-0.182-0.087c8.284 4.103 13.879 12.499 13.879 22.201 0 1.413-0.119 2.798-0.347 4.146l0.020-0.145c-0.008 0.56-0.012 1.222-0.012 1.885 0 7.411 0.552 14.692 1.617 21.806l-0.099-0.802c12.467 97.023 94.545 171.237 193.956 171.237 107.952 0 195.464-87.512 195.464-195.464 0-10.799-0.876-21.393-2.56-31.716l0.152 1.128c-14.378-94.161-94.789-165.461-191.853-165.461-7.959 0-15.806 0.479-23.513 1.411l0.928-0.091zM326.475 988.762c-87.611-1.361-158.111-72.702-158.111-160.509 0-88.657 71.87-160.527 160.527-160.527s160.527 71.87 160.527 160.527c0 0.523-0.003 1.046-0.007 1.567l0.001-0.080c-1.183 88.082-72.864 159.031-161.116 159.031-0.64 0-1.279-0.004-1.918-0.011l0.097 0.001zM778.24 988.762c-88.136-0.684-159.32-72.29-159.32-160.523 0-88.657 71.87-160.527 160.527-160.527s160.527 71.87 160.527 160.527c0 0.316-0.001 0.632-0.003 0.948l0-0.049c-0.675 88.309-72.419 159.637-160.824 159.637-0.743 0-1.484-0.005-2.225-0.015l0.112 0.001z"}],
                path![attrs!{At::D => "M486.701 652.047c3.028 4.352 8.005 7.164 13.639 7.164 3.71 0 7.135-1.22 9.897-3.28l-0.044 0.031c4.286-3.098 7.042-8.082 7.042-13.709 0-3.669-1.172-7.065-3.161-9.833l0.034 0.050-53.609-74.993 76.499-20.179-93.967-114.146c-3.117-3.818-7.823-6.237-13.095-6.237-4.075 0-7.812 1.445-10.727 3.85l0.029-0.023c-3.751 3.17-6.117 7.877-6.117 13.138 0 4.042 1.397 7.757 3.734 10.69l-0.027-0.035 60.235 73.487-72.885 19.576z"}],
            ]
        ),
    ]
}

#[view]
fn search_hint_container(
    label: &str,
    icon: &str, 
    view_box: &str, 
    paths: Vec<Node<Msg>>,
) -> Node<Msg> {
    div![
        C!["search-hint-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("0 0 50%")
            .flex_direction(CssFlexDirection::Column)
            .justify_content(CssJustifyContent::Center)
            .margin_bottom(rem(4))
            .padding("0 2rem"),
        svg![
            C!["icon"],
            s()
                .overflow(CssOverflow::Visible)
                .fill(Color::SurfaceLight5_90)
                .flex(CssFlex::None)
                .height(rem(6))
                .margin_bottom(rem(2))
                .width(rem(6)),
            attrs!{
                At::from("icon") => icon,
                At::ViewBox => view_box,
            },
            paths,
        ],
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .font_size(rem(1.2))
                .text_align(CssTextAlign::Center),
            label,
        ]
    ]
}

#[view]
fn search_rows(search_results: &[VideoGroupResults], root_base_url: &Url) -> Vec<Node<Msg>> {
    search_results
        .iter()
        .enumerate()
        .map(|(index, group)| search_row(index, group, root_base_url))
        .collect()
}

#[view]
fn search_row(index: usize, group: &VideoGroupResults, root_base_url: &Url) -> Node<Msg> {
    div![
        C!["search-row", "search-row-poster", "meta-row-container"],
        s()
            .margin("4rem 2rem")
            .overflow(CssOverflow::Visible),
        IF!(index == 0 => s().margin_top(rem(2))),
        search_row_header_container(group),
        search_row_meta_items_container(group, root_base_url),
    ]
}

#[view]
fn search_row_header_container(group: &VideoGroupResults) -> Node<Msg> {
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
fn search_row_meta_items_container(group: &VideoGroupResults, root_base_url: &Url) -> Node<Msg> {
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
fn meta_item(video: &Video, root_base_url: &Url) -> Node<Msg> {
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
        poster_container(&video.poster),
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
fn poster_container(poster: &str) -> Node<Msg> {
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
