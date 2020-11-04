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
            "Board",
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
                At::Href => RootUrls::new(root_base_url).search("office")
            },
            "Go to Search ▶"
        ]
    ]
}
