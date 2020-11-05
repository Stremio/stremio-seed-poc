use crate::{PageId, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let search_query = url.next_hash_path_part().map(ToOwned::to_owned);

    if let Some(model) = model {
        model.search_query = search_query;
    } else {
        *model = Some(Model {
            base_url: url.to_hash_base_url(),
            search_query
        })
    }
    Some(PageId::Search)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    search_query: Option<String>,
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
    
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {

}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, context: &Context ) -> Node<Msg> {
    div![
        C!["route-content"],
        s()
            .position(CssPosition::Absolute)
            .bottom("0")
            .left("0")
            .right("0")
            .top("0")
            .overflow(CssOverflow::Hidden)
            .z_index("0"),
        div![
            C!["search-container", "main-nav-bars-container"],
            s()
                .background_color(Color::BackgroundDark2)
                .height(pc(100))
                .width(pc(100))
                .position(CssPosition::Relative)
                .z_index("0"),
            horizontal_nav_bar(&context.root_base_url),
            vertical_nav_bar(),
            nav_content_container(),
        ]
    ]
}

fn horizontal_nav_bar(root_base_url: &Url) -> Node<Msg> {
    nav![
        C!["horizontal-nav-bar", "horizontal-nav-bar-container"],
        s()
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0")
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .overflow(CssOverflow::Visible)
            .padding_right(rem(1)),
        logo_container(),
        spacer(None),
        search_bar(),
        spacer(Some("11rem")),
        addons_top_button(root_base_url),
        fullscreen_button(),
        menu_button(),
    ]
}

fn logo_container() -> Node<Msg> {
    div![
        C!["logo-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::VERTICAL_NAV_BAR_SIZE),
    ]
}

fn spacer(max_width: Option<&str>) -> Node<Msg> {
    div![
        C!["spacing"],
        s()
            .flex("1 0 0"),
        max_width.map(|max_width| {
            s()
                .max_width(max_width)
        }),
    ]
}

fn search_bar() -> Node<Msg> {
    label![
        C!["search-bar", "search-bar-container"],
        s()
            .flex("2 0 9.5rem")
            .max_width(rem(30))
            .background_color(Color::BackgroundLight2)
            .border_radius(global::SEARCH_BAR_SIZE)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::SEARCH_BAR_SIZE),
        s()
            .hover()
            .background_color(Color::BackgroundLight3)
    ]
}

fn addons_top_button(root_base_url: &Url) -> Node<Msg> {
    a![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer)
            .outline_color(Color::SurfaceLight5)
            .raw(format!("outline-offset: calc(-1 * {});", global::FOCUS_OUTLINE_SIZE).as_str())
            .outline_width(global::FOCUS_OUTLINE_SIZE),
        s()
            .focus()
            .outline_style(CssOutlineStyle::Solid),
        attrs!{
            At::TabIndex => -1,
            At::Title => "Addons",
            At::Href => RootUrls::new(root_base_url).addons_urls().root(),
        },
    ]
}

fn fullscreen_button() -> Node<Msg> {
    div![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer)
            .outline_color(Color::SurfaceLight5)
            .raw(format!("outline-offset: calc(-1 * {});", global::FOCUS_OUTLINE_SIZE).as_str())
            .outline_width(global::FOCUS_OUTLINE_SIZE),
        s()
            .focus()
            .outline_style(CssOutlineStyle::Solid),
        attrs!{
            At::TabIndex => -1,
            At::Title => "Enter Fullscreen",
        },
    ]
}

fn menu_button() -> Node<Msg> {
    label![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer)
            .outline_color(Color::SurfaceLight5)
            .raw(format!("outline-offset: calc(-1 * {});", global::FOCUS_OUTLINE_SIZE).as_str())
            .outline_width(global::FOCUS_OUTLINE_SIZE)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative),
        s()
            .focus()
            .outline_style(CssOutlineStyle::Solid),
        attrs!{
            At::TabIndex => -1,
        },
    ]
}

fn vertical_nav_bar() -> Node<Msg> {
    nav![
        C!["vertical-nav-bar", "vertical-nav-bar-container"],
        s()
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .top(global::HORIZONTAL_NAV_BAR_SIZE)
            .z_index("1")
            .background_color(Color::BackgroundDark1)
            .overflow_y(CssOverflowY::Auto)
            .raw("scrollbar-width: none;")
            .width(global::VERTICAL_NAV_BAR_SIZE),
    ]
}

fn nav_content_container() -> Node<Msg> {
    div![
        C!["nav-content-container"],
        s()
            .bottom("0")
            .left(global::VERTICAL_NAV_BAR_SIZE)
            .position(CssPosition::Absolute)
            .right("0")
            .top(global::HORIZONTAL_NAV_BAR_SIZE)
            .z_index("0")
    ]
}
