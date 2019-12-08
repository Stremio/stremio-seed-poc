use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>() -> impl View<Ms> {
    div!["Detail"]
}
