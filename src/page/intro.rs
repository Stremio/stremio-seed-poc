use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};

// ---- url parts ----

const LOGIN: &str = "login";

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

    let form_type = match url.next_hash_path_part() {
        None => FormType::SignUp,
        Some(LOGIN) => FormType::LogIn,
        _ => return None,
    };

    let model = model.get_or_insert_with(move || Model {
        base_url,
        form_type,
    });
    Some(PageId::Intro)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    form_type: FormType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FormType {
    SignUp,
    LogIn,
} 

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().add_hash_path_part(LOGIN)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["route-content"],
        s()
            .bottom("0")
            .left("0")
            .overflow(CssOverflow::Hidden)
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
        div![
            C!["intro-container"],
            s()
                .background(format!("linear-gradient(hsla(243,24.4%,6%,0.8),hsla(243,24.4%,6%,0.8)),url(\"{}\"", global::image_url("intro_background.jpg")).as_str())
                .background_origin("border-box")
                .background_position(CssBackgroundPosition::Center)
                .background_repeat(CssBackgroundRepeat::NoRepeat)
                .background_size("cover")
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .height(pc(100))
                .overflow_y(CssOverflowY::Auto)
                .width(pc(100)),
            form_container(model),
        ]
    ]
}

#[view]
fn form_container(model: &Model) -> Node<Msg> {
    div![
        C!["form-container"],
        s()
            .flex(CssFlex::None)
            .margin(CssMargin::Auto)
            .padding("2rem 0")
            .width(rem(30)),
        logo_container(),
        facebook_button(),
        IF!(model.form_type == FormType::SignUp => {
            login_form_button()
        }),
        email_input(),
        password_input(),
        IF!(model.form_type == FormType::SignUp => {
            vec![
                confirm_password_input(),
                terms_and_conditions_checkbox(),
                privacy_policy_checkbox(),
                marketing_checkbox(),
                sign_up_button(),
                guest_login_button(),
            ]
        }),
        IF!(model.form_type == FormType::LogIn => {
            vec![
                forgot_password_button(),
                login_button(),
                sign_up_with_email_button(),
            ]
        }),
    ]
}

#[view]
fn logo_container() -> Node<Msg> {
    div![

    ]
}

#[view]
fn facebook_button() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn login_form_button() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn email_input() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn password_input() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn confirm_password_input() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn terms_and_conditions_checkbox() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn privacy_policy_checkbox() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn marketing_checkbox() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn sign_up_button() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn guest_login_button()-> Node<Msg> {
    div![
        
    ]
}

#[view]
fn forgot_password_button() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn login_button() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn sign_up_with_email_button()-> Node<Msg> {
    div![
        
    ]
}
