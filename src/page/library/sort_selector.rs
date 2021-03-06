use crate::multi_select;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::library_with_filters::{LibraryWithFilters, LibraryRequest, Sort};
use seed_hooks::{*, topo::nested as view};

// ------ ------
//     View
// ------ ------

#[view]
pub fn view<Ms: 'static, F>(
    library: &LibraryWithFilters<F>,
    send_library_request_msg: impl Fn(LibraryRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(library, send_library_request_msg);
    multi_select::view("Select sort", items, true)
}

pub fn items<Ms: 'static, F>(
    library: &LibraryWithFilters<F>,
    send_library_request_msg: impl Fn(LibraryRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    library
        .selectable
        .sorts
        .iter()
        .map(|sort| {
            let library_request = sort.request.clone();
            multi_select::Item {
                title: match &sort.sort {
                    Sort::LastWatched => "Last Watched",
                    Sort::Name => "Name",
                    Sort::TimesWatched => "Times Watched",
                }.to_owned(),
                selected: sort.selected,
                on_click: Rc::new(move || {
                    send_library_request_msg(library_request.clone())
                }),
            }
        })
        .collect()
}
