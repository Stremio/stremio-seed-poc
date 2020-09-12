use crate::route::Route;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>() -> Node<Ms> {
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
                At::Href => Route::Discover(None).to_href()
            },
            "Go to Discover ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => Route::Player.to_href()
            },
            "Go to Player ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => Route::Addons(None).to_href()
            },
            "Go to Addons ▶"
        ]
    ]
}
