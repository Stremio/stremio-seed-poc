use crate::{PageId, Actions};
use seed::{prelude::*, *};
use std::rc::Rc;
use stremio_core::state_types::{Action, ActionLoad, Msg as CoreMsg};
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use crate::styles::{self, themes::{Color, get_color_value}};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let type_name = url.next_hash_path_part()?.to_owned();
    let id = url.next_hash_path_part()?.to_owned();
    let video_id = url.next_hash_path_part().map(ToOwned::to_owned);

    // @TODO refactor and integrate
    // @TODO - wait until branch `details_model` or `development` is merged into `master` (?)
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::Detail {
            type_name,
            id,
            video_id,
        },
    )))));

    model.get_or_insert_with(|| Model {});
    Some(PageId::Detail)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    // @TODO
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn without_video_id(self, type_name: &str, id: &str) -> Url {
        self.base_url()
            .add_hash_path_part(type_name)
            .add_hash_path_part(id)
    }
    pub fn with_video_id(self, type_name: &str, id: &str, video_id: &str) -> Url {
        self.base_url()
            .add_hash_path_part(type_name)
            .add_hash_path_part(id)
            .add_hash_path_part(video_id)
    }
}

// ------ ------
//    Update
// ------ ------

pub struct Msg;

pub fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    unimplemented!()
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>() -> Node<Ms> {
    let list_style = s()
        .flex_grow("0")
        .flex_shrink("0")
        .flex_basis(CssFlexBasis::Auto)
        .align_self(CssAlignSelf::Stretch);

    div![
        C!["detail-container",],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .width(pc(100))
            .height(pc(100))
            .background_color(Color::Background),
        view_nav(),
        div![
            C!["detail-content"],
            s()
                .position(CssPosition::Relative)
                .z_index("0")
                .flex("1")
                .align_self(CssAlignSelf::Stretch)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row),
            view_background_image_layer(),
            view_meta_preview_container(),
            // @TODO switch by `type_name` (?)
            if true {
                view_streams_list_container(&list_style)
            } else {
                view_videos_list_container(&list_style)
            }
        ]
    ]
}

fn view_nav<Ms: 'static>() -> Node<Ms> {
    div![
        C!["nav-bar", "nav-bar-container",],
        s()
            .flex(CssFlex::None)
            .align_self(CssAlignSelf::Stretch),
        div![
            C![
                "nav-tab-button",
                "nav-tab-button-container",
                "button-container",
            ],
            attrs! {
                At::TabIndex => -1,
                At::Title => "back",
            },
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 607 1024",
                    "icon" => "ic_back_ios",
                },
                path![attrs! {
                    At::D => "M607.473 926.419l-412.009-414.419 412.009-414.419-97.28-97.581-510.193 512 510.193 512z"
                }]
            ],
            div![C!["label",], "Back"]
        ],
        h2![C!["title"], "Underworld",]
    ]
}

// ------ view background image ------

fn view_background_image_layer<Ms: 'static>() -> Node<Ms> {
    div![
        C!["background-image-layer"],
        s()
            .position(CssPosition::Absolute)
            .top("0")
            .right("0")
            .bottom("0")
            .left("0")
            .z_index("-1"),
        img![
            C!["background-image"],
            s()
                .display(CssDisplay::Block)
                .width(pc(100))
                .height(pc(100))
                .raw("object-fit: cover;")
                .raw("object-position: top left;")
                .filter("brightness(50%)"),
            attrs! {
                At::Src => "https://images.metahub.space/background/medium/tt0320691/img",
                At::Alt => " ",
            }
        ]
    ]
}

// ------ view meta preview ------

fn view_meta_preview_container<Ms: 'static>() -> Node<Ms> {
    div![
        C!["meta-preview", "meta-preview-container",],
        s()
            .flex("1")
            .align_self(CssAlignSelf::Stretch),
        view_meta_info_container(),
        view_action_buttons_container(),
    ]
}

fn view_meta_info_container<Ms: 'static>() -> Node<Ms> {
    div![
        C![
            "meta-info-container",
        ],
        img![
            C![
                "logo",
            ],
            attrs!{
                At::Src => "https://images.metahub.space/logo/medium/tt0320691/img",
                At::Alt => " ",
            }
        ],
        div![
            C![
                "runtime-release-info-container",
            ],
            div![
                C![
                    "release-info-label"
                ],
                "2003"
            ]
        ],
        div![
            C![
                "name-container"
            ],
            "Underworld"
        ],
        div![
            C![
                "description-container",
            ],
            "Selene, a vampire warrior, is entrenched in a conflict between vampires and werewolves, while falling in love with Michael, a human who is sought by werewolves for unknown reasons."
        ],
        view_meta_links_containers(),
    ]
}

// ------ view meta links containers ------

fn view_meta_links_containers<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        div![
            C!["meta-links", "meta-links-container"],
            div![C!["label-container",], "Genres:"],
            div![C!["links-container"], view_genres(),]
        ],
        div![
            C!["meta-links", "meta-links-container"],
            div![C!["label-container",], "Writers:"],
            div![C!["links-container"], view_writers(),]
        ],
        div![
            C!["meta-links", "meta-links-container"],
            div![C!["label-container",], "Directors:"],
            div![C!["links-container"], view_directors(),]
        ],
        div![
            C!["meta-links", "meta-links-container"],
            div![C!["label-container",], "Cast:"],
            div![C!["links-container"], view_cast(),]
        ],
    ]
}

fn view_genres<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Action",
                At::Href => "#/discover/xxx/yyy/?genre=Action"
            },
            "Action, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Fantasy",
                At::Href => "#/discover/xxx/yyy/?genre=Fantasy"
            },
            "Fantasy, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Thriller",
                At::Href => "#/discover/xxx/yyy/?genre=Thriller"
            },
            "Thriller",
        ],
    ]
}

fn view_writers<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Kevin Grevioux",
                At::Href => "#/search?q=Kevin Grevioux"
            },
            "Kevin Grevioux, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Len Wiseman",
                At::Href => "#/search?q=Len Wiseman"
            },
            "Len Wiseman, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Danny McBride",
                At::Href => "#/search?q=Danny McBride"
            },
            "Danny McBride"
        ],
    ]
}

fn view_directors<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![a![
        C!["link-container", "button-container",],
        attrs! {
            At::TabIndex => -1,
            At::Title => "Len Wiseman",
            At::Href => "#/search?q=Len Wiseman"
        },
        "Len Wiseman"
    ]]
}

fn view_cast<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Kate Beckinsale",
                At::Href => "#/search?q=Kate Beckinsale"
            },
            "Kate Beckinsale, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Scott Speedman",
                At::Href => "#/search?q=Scott Speedman"
            },
            "Scott Speedman, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Michael Sheen",
                At::Href => "#/search?q=Michael Sheen"
            },
            "Michael Sheen, "
        ],
        a![
            C!["link-container", "button-container",],
            attrs! {
                At::TabIndex => -1,
                At::Title => "Shane Brolly",
                At::Href => "#/search?q=Shane Brolly"
            },
            "Shane Brolly"
        ],
    ]
}

// ------ view action buttons container ------

fn view_action_buttons_container<Ms: 'static>() -> Node<Ms> {
    div![
        C!["action-buttons-container",],
        view_action_button_add_to_library(),
        view_action_button_trailer(),
        view_action_button_imdb(),
        view_action_button_share(),
    ]
}

fn view_action_button_add_to_library<Ms: 'static>() -> Node<Ms> {
    div![
        C![
            "meta-preview-action-button",
            "action-button-container",
            "button-container",
        ],
        attrs! {
            At::TabIndex => -1,
            At::Title => "Add to library"
        },
        div![
            C!["icon-container",],
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 1264 1024",
                    "icon" => "ic_addlib",
                },
                path![attrs! {
                    At::D => "M78.306 0c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v764.988c2.636 41.27 36.754 73.744 78.456 73.744s75.82-32.474 78.445-73.514l0.012-0.23v-764.988c-0.171-43.284-35.299-78.306-78.606-78.306-0 0-0 0-0.001 0l0 0z"
                }],
                path![attrs! {
                    At::D => "M341.835 153.901c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v611.087c0 43.663 35.396 79.059 79.059 79.059s79.059-35.396 79.059-79.059v0-611.087c-0.166-43.288-35.296-78.315-78.607-78.315-0.424 0-0.847 0.003-1.269 0.010l0.064-0.001z"
                }],
                path![attrs! {
                    At::D => "M963.765 421.647c-166.335 0-301.176 134.841-301.176 301.176s134.841 301.176 301.176 301.176c166.335 0 301.176-134.841 301.176-301.176v0c0-166.335-134.841-301.176-301.176-301.176v0zM1156.518 768.602h-148.179v147.275h-90.353v-148.179h-147.878v-90.353h147.275v-147.878h90.353v147.275h147.275z"
                }],
                path![attrs! {
                    At::D => "M683.972 465.016v-386.711c-2.636-41.27-36.754-73.744-78.456-73.744s-75.82 32.474-78.445 73.514l-0.012 0.23v764.988c-0 0-0 0-0 0.001 0 43.247 35.059 78.306 78.306 78.306 0.106 0 0.212-0 0.318-0.001l-0.016 0c0.068 0 0.147 0 0.227 0 10.82 0 21.097-2.329 30.355-6.513l-0.465 0.188c-32.753-54.79-52.119-120.857-52.119-191.447 0-99.528 38.499-190.064 101.417-257.529l-0.206 0.223z"
                }],
                path![attrs! {
                    At::D => "M817.092 371.351c42.987-18.759 93.047-29.807 145.652-30.117l0.117-0.001h8.433l-60.235-262.325c-8.294-35.054-39.322-60.736-76.348-60.736-43.274 0-78.355 35.081-78.355 78.355 0 6.248 0.731 12.325 2.113 18.151l-0.106-0.532z"
                }],
            ],
        ],
        div![C!["label-container",], div![C!["label"], "Add to library",]]
    ]
}

fn view_action_button_trailer<Ms: 'static>() -> Node<Ms> {
    a![
        C![
            "meta-preview-action-button",
            "action-button-container",
            "button-container",
        ],
        attrs! {
            At::TabIndex => 0,
            At::Title => "Trailer",
            At::Href => "#/player?stream=mn4O3iQ8B_s",
        },
        div![
            C!["icon-container",],
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 840 1024",
                    "icon" => "ic_movies",
                },
                path![attrs! {
                    At::D => "M813.176 1024h-708.969c-14.3-3.367-24.781-16.017-24.781-31.115 0-0.815 0.031-1.623 0.090-2.422l-0.006 0.107q0-215.642 0-430.984v-4.819c0.015 0 0.033 0 0.051 0 30.976 0 58.991-12.673 79.146-33.116l0.013-0.013c19.218-19.773 31.069-46.796 31.069-76.586 0-1.134-0.017-2.265-0.051-3.391l0.004 0.165h649.939v558.381c-1.037 2.541-2.047 4.621-3.168 6.63l0.157-0.306c-4.8 8.938-13.235 15.394-23.273 17.431l-0.219 0.037zM796.612 481.882h-126.795c-1.944 0.438-3.547 1.646-4.5 3.28l-0.018 0.033-60.235 95.473c-0.466 0.866-0.972 1.957-1.422 3.076l-0.084 0.237h128.301c3.012 0 3.915 0 5.421-3.313l56.922-95.172c0.887-1.056 1.687-2.24 2.356-3.505l0.053-0.11zM393.638 583.078h128.602c0.156 0.017 0.337 0.026 0.52 0.026 2.3 0 4.246-1.517 4.892-3.604l0.010-0.036c18.974-30.118 37.948-62.645 56.621-94.268l2.711-4.518h-125.892c-0.179-0.018-0.387-0.028-0.597-0.028-2.519 0-4.694 1.473-5.711 3.604l-0.016 0.038-58.428 94.268zM377.675 481.882h-126.193c-0.024-0-0.052-0.001-0.080-0.001-2.57 0-4.763 1.609-5.629 3.875l-0.014 0.041-58.428 93.064-2.711 4.216h124.386c0.165 0.018 0.357 0.028 0.551 0.028 2.127 0 3.968-1.225 4.856-3.008l0.014-0.031 60.235-95.473z"
                }],
                path![attrs! {
                    At::D => "M707.464 0c4.931 1.519 9.225 3.567 13.143 6.142l-0.192-0.119c4.632 3.831 8.386 8.548 11.033 13.909l0.11 0.247c18.372 44.574 36.442 90.353 54.814 134.325l-602.353 243.652c-18.275-41.26-58.864-69.523-106.054-69.523-14.706 0-28.77 2.745-41.71 7.75l0.79-0.269c-4.819-12.047-10.842-24.094-14.758-37.045-0.883-2.705-1.392-5.818-1.392-9.050 0-13.254 8.561-24.508 20.455-28.534l0.212-0.062c18.673-6.626 39.153-14.456 58.428-20.48l542.118-217.751 43.972-19.275 10.24-3.915zM123.181 271.059h1.807l93.064 67.464c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 90.353-35.84 26.504-10.842-2.409-1.807-91.859-65.656c-0.846-0.572-1.889-0.914-3.012-0.914s-2.166 0.341-3.031 0.926l0.019-0.012-77.402 30.118zM535.793 214.739l-2.711-2.108-90.353-66.56c-0.933-0.622-2.080-0.993-3.313-0.993s-2.38 0.371-3.335 1.007l0.022-0.014-118.061 45.779 2.108 1.807 92.461 67.162c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 87.341-34.635zM730.353 135.529h-1.807l-91.859-68.969c-0.803-0.547-1.794-0.874-2.861-0.874s-2.059 0.327-2.879 0.885l0.018-0.011-90.353 36.744c-8.433 3.012-16.565 6.325-24.998 9.939l2.409 2.108 90.353 65.355c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 75.294-30.118z"
                }],
                path![attrs! {
                    At::D => "M0 433.393c0-3.614 1.506-7.228 2.409-10.541 8.935-34.682 39.932-59.894 76.818-59.894 4.782 0 9.465 0.424 14.014 1.236l-0.48-0.071c37.902 5.909 66.564 38.317 66.564 77.421 0 2.432-0.111 4.839-0.328 7.214l0.023-0.305c-3.944 40.578-37.878 72.037-79.159 72.037-39.144 0-71.681-28.287-78.286-65.534l-0.070-0.48c-0.474-1.046-0.977-1.935-1.547-2.775l0.041 0.064z"
                }],
            ],
        ],
        div![C!["label-container",], div![C!["label"], "Trailer",]]
    ]
}

fn view_action_button_imdb<Ms: 'static>() -> Node<Ms> {
    a![
        C![
            "meta-preview-action-button",
            "action-button-container",
            "button-container",
        ],
        attrs! {
            At::TabIndex => 0,
            At::Title => "7.0 / 10",
            At::Href => "https://imdb.com/title/tt0320691",
            At::Target => "_blank",
        },
        div![
            C!["icon-container",],
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 1762 1024",
                    "icon" => "ic_imdb",
                },
                path![attrs! {
                    At::D => "M1598.645 0h-1435.106c-90.32 0-163.539 73.219-163.539 163.539v0 696.922c0 90.32 73.219 163.539 163.539 163.539v0h1435.106c90.32 0 163.539-73.219 163.539-163.539h-0v-696.922c0-90.32-73.219-163.539-163.539-163.539h-0zM1650.146 860.461c-0.17 28.375-23.126 51.331-51.485 51.501l-0.016 0h-1435.106c-28.375-0.17-51.331-23.126-51.501-51.485l-0-0.016v-696.922c0.17-28.375 23.126-51.331 51.485-51.501l0.016-0h1435.106c28.375 0.17 51.331 23.126 51.501 51.485l0 0.016z"
                }],
                path![attrs! {
                    At::D => "M246.965 267.445h89.449v458.391h-89.449v-458.391z"
                }],
                path![attrs! {
                    At::D => "M606.268 576.452l-87.341-309.007h-113.845v458.391h78.607v-319.247l84.329 268.348h76.499l84.329-274.673v325.572h78.607v-458.391h-113.845l-87.341 309.007z"
                }],
                path![attrs! {
                    At::D => "M1033.939 267.445h-150.588v458.391h150.588c3.777 0.267 8.184 0.419 12.627 0.419 46.147 0 88.459-16.422 121.41-43.742l-0.315 0.254c29.323-28.212 47.538-67.787 47.538-111.617 0-3.115-0.092-6.208-0.273-9.277l0.020 0.423v-130.409c0.139-2.475 0.219-5.372 0.219-8.286 0-43.945-18.065-83.671-47.175-112.157l-0.028-0.027c-32.988-27.57-75.855-44.31-122.631-44.31-4.008 0-7.987 0.123-11.934 0.365l0.541-0.027zM1126.701 564.104c0.085 1.316 0.134 2.853 0.134 4.401 0 21.296-9.21 40.441-23.863 53.67l-0.064 0.057c-16.699 12.886-37.923 20.654-60.961 20.654-2.076 0-4.137-0.063-6.181-0.187l0.281 0.014h-67.162v-291.84h67.162c1.905-0.131 4.13-0.206 6.371-0.206 22.889 0 43.96 7.785 60.709 20.851l-0.22-0.165c14.712 13.184 23.926 32.244 23.926 53.457 0 1.537-0.048 3.063-0.144 4.576l0.010-0.207z"
                }],
                path![attrs! {
                    At::D => "M1436.311 393.939c-0.178-0.001-0.389-0.002-0.6-0.002-17.816 0-34.259 5.882-47.492 15.811l0.205-0.147c-11.386 9.029-20.763 20.040-27.727 32.576l-0.282 0.554v-175.285h-86.438v458.391h86.438v-43.068c6.124 13.388 15.135 24.523 26.291 32.975l0.213 0.155c13.088 9.255 29.376 14.794 46.957 14.794 0.857 0 1.71-0.013 2.56-0.039l-0.125 0.003c1.284 0.061 2.788 0.095 4.301 0.095 31.288 0 59.116-14.817 76.849-37.817l0.168-0.226c18.98-26.686 30.343-59.939 30.343-95.847 0-3.047-0.082-6.074-0.243-9.081l0.018 0.419v-51.501c0.143-2.602 0.224-5.648 0.224-8.712 0-36.189-11.357-69.725-30.701-97.238l0.359 0.539c-17.796-22.891-45.337-37.477-76.284-37.477-1.77 0-3.53 0.048-5.277 0.142l0.243-0.010zM1459.802 589.101c0.060 1.075 0.094 2.332 0.094 3.598 0 15.836-5.361 30.42-14.366 42.037l0.117-0.157c-8.604 9.843-21.183 16.026-35.206 16.026-0.859 0-1.712-0.023-2.559-0.069l0.118 0.005c-0.398 0.010-0.865 0.016-1.335 0.016-9.083 0-17.645-2.234-25.165-6.182l0.297 0.142c-7.292-3.783-13.028-9.713-16.469-16.945l-0.095-0.222c-3.627-7.519-5.747-16.351-5.747-25.678 0-0.608 0.009-1.214 0.027-1.818l-0.002 0.089v-73.487c-0.022-0.631-0.035-1.371-0.035-2.115 0-9.607 2.122-18.718 5.922-26.89l-0.164 0.394c3.604-7.587 9.311-13.682 16.368-17.667l0.197-0.102c7.223-3.806 15.784-6.040 24.868-6.040 0.469 0 0.937 0.006 1.404 0.018l-0.069-0.001c0.743-0.043 1.612-0.068 2.488-0.068 14.131 0 26.756 6.445 35.097 16.555l0.062 0.077c8.389 11.489 13.422 25.892 13.422 41.471 0 1.728-0.062 3.441-0.184 5.138l0.013-0.228z"
                }],
            ],
        ],
        div![C!["label-container",], div![C!["label"], "7.0 / 10",]]
    ]
}

fn view_action_button_share<Ms: 'static>() -> Node<Ms> {
    div![
        C![
            "meta-preview-action-button",
            "action-button-container",
            "button-container",
        ],
        attrs! {
            At::TabIndex => -1,
            At::Title => "Share"
        },
        div![
            C!["icon-container",],
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 1024 1024",
                    "icon" => "ic_share",
                },
                path![attrs! {
                    At::D => "M846.005 679.454c-62.726 0.19-117.909 32.308-150.171 80.95l-0.417 0.669-295.755-96.979c2.298-11.196 3.614-24.064 3.614-37.239 0-0.038-0-0.075-0-0.113l0 0.006c0-0.039 0-0.085 0-0.132 0-29.541-6.893-57.472-19.159-82.272l0.486 1.086 221.967-143.059c42.092 37.259 97.727 60.066 158.685 60.235l0.035 0c0.81 0.010 1.768 0.016 2.726 0.016 128.794 0 233.38-103.646 234.901-232.079l0.001-0.144c0-131.737-106.794-238.532-238.532-238.532s-238.532 106.794-238.532 238.532h0c0.012 33.532 7.447 65.325 20.752 93.828l-0.573-1.367-227.087 146.372c-32.873-23.074-73.687-36.92-117.729-37.045l-0.031-0c-0.905-0.015-1.974-0.023-3.044-0.023-108.186 0-196.124 86.69-198.139 194.395l-0.003 0.189c2.017 107.893 89.956 194.583 198.142 194.583 1.070 0 2.139-0.008 3.205-0.025l-0.161 0.002c0.108 0 0.235 0 0.363 0 60.485 0 114.818-26.336 152.159-68.168l0.175-0.2 313.826 103.002c-0.004 0.448-0.006 0.976-0.006 1.506 0 98.47 79.826 178.296 178.296 178.296s178.296-79.826 178.296-178.296c0-98.468-79.823-178.293-178.29-178.296l-0-0zM923.106 851.727c0.054 1.079 0.084 2.343 0.084 3.614 0 42.748-34.654 77.402-77.402 77.402s-77.402-34.654-77.402-77.402c0-42.748 34.654-77.402 77.402-77.402 0.076 0 0.152 0 0.229 0l-0.012-0c0.455-0.010 0.99-0.015 1.527-0.015 41.12 0 74.572 32.831 75.572 73.711l0.002 0.093zM626.748 230.4c3.537-73.358 63.873-131.495 137.788-131.495s134.251 58.137 137.776 131.179l0.012 0.316c-3.537 73.358-63.873 131.495-137.788 131.495s-134.251-58.137-137.776-131.179l-0.012-0.316zM301.176 626.748c-1.34 53.35-44.907 96.087-98.456 96.087-0.54 0-1.078-0.004-1.616-0.013l0.081 0.001c-1.607 0.096-3.486 0.151-5.377 0.151-53.061 0-96.075-43.014-96.075-96.075s43.014-96.075 96.075-96.075c1.892 0 3.77 0.055 5.635 0.162l-0.258-0.012c0.459-0.008 1-0.012 1.543-0.012 53.443 0 96.943 42.568 98.445 95.648l0.003 0.139z"
                }],
            ],
        ],
        div![C!["label-container",], div![C!["label"], "Share",]]
    ]
}

// ------ view streams list ------

fn view_streams_list_container<Ms: 'static>(list_style: &Style) -> Node<Ms> {
    div![
        C!["streams-list", "streams-list-container",],
        list_style,
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .background_color(Color::BackgroundDark80),
        div![
            C!["streams-scroll-container",],
            s()
                .flex_grow("0")
                .flex_shrink("1")
                .flex_basis(CssFlexBasis::Auto)
                .align_self(CssAlignSelf::Stretch)
                .overflow_y(CssOverflowY::Auto),
            // stream
            div![
                C!["stream", "stream-container", "button-container",],
                s()
                    .width(styles::global::ITEM_SIZE)
                    .margin("1rem 2rem"),
                s()
                    .first_child()
                    .margin_top(rem(2)),
                s()
                    .last_child()
                    .margin_bottom("0"),
                attrs! {
                    At::TabIndex => 0,
                    At::Title => "Google Sample Videos",
                },
                div![
                    C!["stream-addon-container",],
                    div![C!["addon-name",], "Google",]
                ],
                div![
                    C!["info-container",],
                    div![C!["description-label",], "Google sample videos",]
                ],
                div![
                    C!["play-icon-container",],
                    svg![
                        C!["play-icon",],
                        attrs! {
                            At::ViewBox => "0 0 899 1024",
                            "icon" => "ic_play",
                        },
                        path![attrs! {
                            At::D => "M891.482 512l-884.254 512v-1024z"
                        }],
                    ],
                ],
            ],
            // stream
            div![
                C!["stream", "stream-container", "button-container",],
                attrs! {
                    At::TabIndex => 0,
                    At::Title => "Stremio demo videos",
                },
                div![
                    C!["stream-addon-container",],
                    div![C!["addon-name",], "Stremio",]
                ],
                div![
                    C!["info-container",],
                    div![C!["description-label",], "Stremio demo videos",]
                ],
                div![
                    C!["play-icon-container",],
                    svg![
                        C!["play-icon",],
                        attrs! {
                            At::ViewBox => "0 0 899 1024",
                            "icon" => "ic_play",
                        },
                        path![attrs! {
                            At::D => "M891.482 512l-884.254 512v-1024z"
                        }],
                    ],
                ],
                div![
                    C!["progress-bar-container",],
                    div![
                        C!["progress-bar",],
                        style! {
                            St::Width => unit!(30, %),
                        }
                    ]
                ]
            ],
        ],
        view_install_addons_button(),
    ]
}

fn view_install_addons_button<Ms: 'static>() -> Node<Ms> {
    a![
        C!["install-addons-container", "button-container",],
        s()
            .flex(CssFlex::None)
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .align_items(CssAlignItems::Center)
            .justify_content(CssJustifyContent::Center)
            .width(styles::global::ITEM_SIZE)
            .margin("1rem 2rem 2rem 2rem")
            .padding(rem(1))
            .background_color(Color::Signal5),
        s()
            .hover()
            .filter("brightness(1.2)"),
        s()
            .focus()
            .filter("brightness(1.2)"),
        attrs! {
            At::TabIndex => 0,
            At::Title => "Install addons",
            At::Href => "#/addons",
        },
        svg![
            C!["icon",],
            s()
                .flex(CssFlex::None)
                .width(rem(3))
                .height(rem(3))
                .margin_right(rem(1))
                .fill(Color::SurfaceLighter),
            attrs! {
                At::ViewBox => "0 0 1043 1024",
                "icon" => "ic_addons",
            },
            path![attrs! {
                At::D => "M145.468 679.454c-40.056-39.454-80.715-78.908-120.471-118.664-33.431-33.129-33.129-60.235 0-90.353l132.216-129.807c5.693-5.938 12.009-11.201 18.865-15.709l0.411-0.253c23.492-15.059 41.864-7.529 48.188 18.974 0 7.228 2.711 14.758 3.614 22.287 3.801 47.788 37.399 86.785 82.050 98.612l0.773 0.174c10.296 3.123 22.128 4.92 34.381 4.92 36.485 0 69.247-15.94 91.702-41.236l0.11-0.126c24.858-21.654 40.48-53.361 40.48-88.718 0-13.746-2.361-26.941-6.701-39.201l0.254 0.822c-14.354-43.689-53.204-75.339-99.907-78.885l-0.385-0.023c-18.372-2.409-41.562 0-48.188-23.492s11.445-34.635 24.998-47.887q65.054-62.946 130.409-126.795c32.527-31.925 60.235-32.226 90.353 0 40.659 39.153 80.715 78.908 120.471 118.362 8.348 8.594 17.297 16.493 26.82 23.671l0.587 0.424c8.609 7.946 20.158 12.819 32.846 12.819 24.823 0 45.29-18.653 48.148-42.707l0.022-0.229c3.012-13.252 4.518-26.805 8.734-39.755 12.103-42.212 50.358-72.582 95.705-72.582 3.844 0 7.637 0.218 11.368 0.643l-0.456-0.042c54.982 6.832 98.119 49.867 105.048 104.211l0.062 0.598c0.139 1.948 0.218 4.221 0.218 6.512 0 45.084-30.574 83.026-72.118 94.226l-0.683 0.157c-12.348 3.915-25.299 5.722-37.948 8.433-45.779 9.638-60.235 46.984-30.118 82.824 15.265 17.569 30.806 33.587 47.177 48.718l0.409 0.373c31.925 31.925 64.452 62.946 96.075 94.871 13.698 9.715 22.53 25.511 22.53 43.369s-8.832 33.655-22.366 43.259l-0.164 0.111c-45.176 45.176-90.353 90.353-137.035 134.325-5.672 5.996-12.106 11.184-19.169 15.434l-0.408 0.227c-4.663 3.903-10.725 6.273-17.341 6.273-13.891 0-25.341-10.449-26.92-23.915l-0.012-0.127c-2.019-7.447-3.714-16.45-4.742-25.655l-0.077-0.848c-4.119-47.717-38.088-86.476-82.967-97.721l-0.76-0.161c-9.584-2.63-20.589-4.141-31.947-4.141-39.149 0-74.105 17.956-97.080 46.081l-0.178 0.225c-21.801 21.801-35.285 51.918-35.285 85.185 0 1.182 0.017 2.36 0.051 3.533l-0.004-0.172c1.534 53.671 40.587 97.786 91.776 107.115l0.685 0.104c12.649 2.409 25.901 3.313 38.249 6.626 22.588 6.325 30.118 21.685 18.372 41.864-4.976 8.015-10.653 14.937-17.116 21.035l-0.051 0.047c-44.875 44.574-90.353 90.353-135.228 133.12-10.241 14.067-26.653 23.106-45.176 23.106s-34.935-9.039-45.066-22.946l-0.111-0.159c-40.659-38.852-80.414-78.908-120.471-118.362z"
            }],
        ],
        div![
            C!["label",], 
            s()
                .flex_grow("0")
                .flex_shrink("1")
                .flex_basis(CssFlexBasis::Auto)
                .font_size(rem(1.5))
                .max_height(em(3.6))
                .color(Color::SurfaceLighter),
            "Install addons",
        ]
    ]
}

// ------ view videos list ------

fn view_videos_list_container<Ms: 'static>(list_style: &Style) -> Node<Ms> {
    div![
        C!["videos-list", "videos-list-container",],
        list_style,
        view_season_bar_container(),
        view_video_scroll_container(),
    ]
}

fn view_season_bar_container<Ms: 'static>() -> Node<Ms> {
    div![
        C!["seasons-bar", "seasons-bar-container",],
        div![
            C!["prev-season-button", "button-container",],
            attrs! {
                At::TabIndex => 0,
            },
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 606 1024",
                    "icon" => "ic_arrow_left",
                },
                path![attrs! {
                    At::D => "M264.132 512l309.609-319.247c19.848-20.685 32.069-48.821 32.069-79.812s-12.221-59.127-32.107-79.852l0.038 0.040c-19.51-20.447-46.972-33.16-77.402-33.16s-57.892 12.713-77.363 33.118l-0.040 0.042-387.012 399.059c-19.713 20.744-31.839 48.862-31.839 79.812s12.126 59.067 31.886 79.861l-0.047-0.050 387.012 399.059c19.51 20.447 46.972 33.16 77.402 33.16s57.892-12.713 77.363-33.118l0.040-0.042c19.848-20.685 32.069-48.821 32.069-79.812s-12.221-59.127-32.107-79.852l0.038 0.040z"
                }],
            ],
        ],
        div![
            C!["seasons-popup-label-container", "button-container",],
            attrs! {
                At::TabIndex => 0,
                At::Title => "Season 1",
            },
            div![C!["season-label"], "Season",],
            div![C!["number-label",], "1",]
        ],
        div![
            C!["next-season-button", "button-container",],
            attrs! {
                At::TabIndex => 0,
            },
            svg![
                C!["icon",],
                attrs! {
                    At::ViewBox => "0 0 606 1024",
                    "icon" => "ic_arrow_right",
                },
                path![attrs! {
                    At::D => "M341.534 512l-309.609-319.247c-19.713-20.744-31.839-48.862-31.839-79.812s12.126-59.067 31.886-79.861l-0.047 0.050c19.51-20.447 46.972-33.16 77.402-33.16s57.892 12.713 77.363 33.118l0.040 0.042 387.012 399.059c19.848 20.685 32.069 48.821 32.069 79.812s-12.221 59.127-32.107 79.852l0.038-0.040-387.012 399.059c-19.51 20.447-46.972 33.16-77.402 33.16s-57.892-12.713-77.363-33.118l-0.040-0.042c-19.713-20.744-31.839-48.862-31.839-79.812s12.126-59.067 31.886-79.861l-0.047 0.050z"
                }],
            ],
        ],
    ]
}

fn view_video_scroll_container<Ms: 'static>() -> Node<Ms> {
    div![
        C!["videos-scroll-container",],
        div![
            C!["video", "video-container", "button-container",],
            attrs! {
                At::TabIndex => 0,
                At::Title => "How to create a Stremio add-on with Node.js",
            },
            div![
                C!["poster-container",],
                img![
                    C!["poster",],
                    attrs! {
                        At::Src => "https://theme.zdassets.com/theme_assets/2160011/77a6ad5aee11a07eb9b87281070f1aadf946f2b3.png",
                        At::Alt => " ",
                    }
                ]
            ],
            div![
                C!["info-container",],
                div![
                    C!["name-container",],
                    "1. How to create a Stremio add-on with Node.js",
                ],
                div![C!["released-container",], "Jun 30, 19",]
            ],
            div![
                C!["next-icon-container",],
                svg![
                    C!["next-icon",],
                    attrs! {
                        At::ViewBox => "0 0 565 1024",
                        "icon" => "ic_arrow_thin_right",
                    },
                    path![attrs! {
                        At::D => "M84.932 14.155l465.016 463.511c8.963 8.73 14.578 20.859 14.757 34.301l0 0.033c-0.021 13.598-5.67 25.873-14.743 34.621l-0.015 0.014-464.113 463.209c-9.052 8.82-21.434 14.26-35.087 14.26s-26.035-5.44-35.098-14.27l0.011 0.010c-9.355-8.799-15.292-21.14-15.66-34.87l-0.001-0.066c-0.001-0.103-0.001-0.225-0.001-0.348 0-13.437 5.534-25.582 14.448-34.278l0.010-0.009 430.080-428.273-429.779-427.972c-9.101-8.684-14.76-20.907-14.76-34.451 0-0.171 0.001-0.341 0.003-0.511l-0 0.026c-0-0.043-0-0.094-0-0.145 0-13.595 5.526-25.899 14.455-34.789l0.002-0.002c9.099-8.838 21.532-14.287 35.238-14.287s26.138 5.449 35.25 14.299l-0.012-0.012z"
                    }],
                ],
            ],
        ]
    ]
}
