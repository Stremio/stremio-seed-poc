use seed::{prelude::*, *};
use crate::{default_resource_request, Page, Msg};

pub fn view() -> Node<Msg> {
    div![
        h1![
            "Board",
        ],
        a![
            attrs!{
                At::Href => Page::Discover(default_resource_request()).to_href()
            },
            "Go to Discover"
        ]
    ]
}
