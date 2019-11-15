use seed::{prelude::*, *};
use crate::{default_resource_request, route::Route};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    div![
        h1![
            "Board",
        ],
        a![
            attrs!{
                At::Href => Route::Discover(default_resource_request()).to_href()
            },
            "Go to Discover"
        ]
    ]
}
