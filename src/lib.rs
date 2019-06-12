#[macro_use]
extern crate seed;
use seed::prelude::*;
// required to make stremio_derive work :(
pub use stremio_core::state_types;
use stremio_core::state_types::*;
use stremio_core::types::{MetaPreview};
use stremio_derive::*;
mod env;
use env::*;

// Model
#[derive(Model, Default)]
struct Model {
    ctx: Ctx<Env>,
    catalog: CatalogGrouped,
}

fn default_load() -> Msg {
    Action::Load(ActionLoad::CatalogGrouped { extra: vec![] }).into()
}

// Update
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    let fx = model.update(&msg);
    if !fx.has_changed {
        orders.skip();
    }
    for fut in fx.effects.into_iter() {
        orders.perform_cmd(fut);
    }
}


// View
fn view(model: &Model) -> El<Msg> {
    let groups: Vec<El<Msg>> = model
        .catalog
        .groups
        .iter()
        .map(|group| {
            let el = match &group.content {
                Loadable::Err(m) => h3![m],
                Loadable::Loading => h3!["Loading"],
                Loadable::Ready(items) if items.len() == 0 => div![],
                Loadable::Ready(items) => div![class!["meta-items-container"],
                    items
                        .iter()
                        .take(7)
                        .map(meta_item)
                        .collect::<Vec<El<Msg>>>()
                ],
            };
            div![class!["board-row"], class!["addon-catalog-row"], el]
        })
        .collect();
    let content = div![class!["board-content"], groups];
    div![class!["board-container"], content]
}

fn meta_item(m: &MetaPreview) -> El<Msg> {
    let default_poster = "https://www.stremio.com/images/add-on-money.png".to_owned();
    let default_shape = "poster".to_owned();
    let poster_shape = m.poster_shape.as_ref().unwrap_or(&default_shape);

    let poster = m.poster
        .as_ref()
        .unwrap_or(&default_poster);

    div![
        attrs! {
            At::Class => format!("meta-item meta-item-container poster-shape-{}", poster_shape);
            At::Title => &m.name
        },
        div![
            class!["poster-image-container"],
            div![
                class!["poster-image-layer"],
                div![
                    class!["poster-image"],
                    style! { "background-image" => format!("url({})", poster) },
                    raw_ev(Ev::Click, |_| default_load())
                    //raw_ev(Ev::Click, |_| Msg::Action(Action::UserOp(ActionUser::Login{ email, password })))
                ]
            ]
        ],
        div![
            class!["title-bar-container"],
            div![class!["title"], &m.name]
        ],
    ]
}

#[wasm_bindgen]
pub fn render() {
    let model = Model::default();

    let app_state = seed::App::build(model, update, view)
        .finish()
        .run();

    app_state.update(default_load());
}
