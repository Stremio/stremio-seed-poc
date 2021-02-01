use crate::Urls as RootUrls;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(root_base_url: &Url) -> Node<Ms> {
    div![
        h1![
            style! {
                St::Padding => px(20),
            },
            "Test Links",
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).root()
            },
            "Go to Board ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).discover_urls().root()
            },
            "Go to Discover ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).player()
            },
            "Go to Player ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).addons_urls().root()
            },
            "Go to Addons ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).search_urls().root()
            },
            "Go to Search ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).detail_urls().without_video_id("series", "tt8111088")
            },
            "Go to Detail (Mandalorian [series]) ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => RootUrls::new(root_base_url).detail_urls().with_video_id("movie", "tt11656172", "tt11656172")
            },
            "Go to Detail (Hard Kill [movie]) ▶"
        ],
    ]
}
