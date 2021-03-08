use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use std::rc::Rc;
use stremio_core::types::profile::User;
use stremio_core::runtime::msg::{Action, ActionCtx, Msg as CoreMsg};
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};

mod side_menu;
use side_menu::{side_menu, SideMenuButton};

mod section;
use section::sections;

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    context: &mut Context,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();

    let model = model.get_or_insert_with(move || Model {
        active_side_menu_button: SideMenuButton::General
    });
    Some(PageId::Settings)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    active_side_menu_button: SideMenuButton 
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Logout
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Logout => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::Logout
            )))));
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    basic_layout(BasicLayoutArgs {
        page_content: settings_content(model, context).map_msg(msg_mapper),
        container_class: "settings-container",
        context,
        page_id,
        search_args: None,
    })
}

#[view]
fn settings_content<'a>(model: &Model, context: &Context) -> Node<Msg> {
    let user = context.core_model.ctx.profile.auth.as_ref().map(|auth| &auth.user);
    div![
        C!["settings-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(pc(100))
            .width(pc(100)),            
        side_menu(model.active_side_menu_button),
        sections(&context.root_base_url, user),
    ]
}


