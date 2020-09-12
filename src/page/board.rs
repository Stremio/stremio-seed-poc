use seed::{prelude::*, *};
use crate::Urls as RootUrls;

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
                At::Href => RootUrls::new(root_base_url).discover(None)
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
                At::Href => RootUrls::new(root_base_url).addons(None)
            },
            "Go to Addons ▶"
        ]
    ]
}
