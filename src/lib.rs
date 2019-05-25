#[macro_use]
extern crate seed;
use seed::prelude::*;
use stremio_core::state_types::*;
use stremio_core::types::{MetaPreview};
use stremio_core::middlewares::*;
use std::rc::Rc;
use futures::sync::mpsc::*;
use futures::stream::Stream;
use futures::future;

mod env;
use env::*;

// Model
struct Model {
    action_tx: Sender<Action>,
    catalog: CatalogGrouped,
}
impl Model {
    fn default_load() -> Action {
        Action::Load(ActionLoad::CatalogGrouped { extra: vec![] })
    }
}


// Update
#[derive(Clone)]
enum Msg {
    Action(Action),
    CatalogUpdated(CatalogGrouped),
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::Action(a) => model.action_tx.clone().try_send(a.to_owned()).unwrap(),
        Msg::CatalogUpdated(c) => model.catalog = c,
    }
}


// View
fn view(model: &Model) -> El<Msg> {
    let groups: Vec<El<Msg>> = model
        .catalog
        .groups
        .iter()
        .map(|group| {
            let el = match group.as_ref() {
                (_, Loadable::Message(m)) => h3![m],
                (_, Loadable::Loading) => h3!["Loading"],
                (_, Loadable::Ready(items)) => div![class!["meta-items-container"],
                    items
                        .iter()
                        .take(7)
                        .map(meta_item)
                        .collect::<Vec<El<Msg>>>()
                ],
                (_, Loadable::ReadyEmpty) => div![],
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
                    raw_ev(Ev::Click, |_| Msg::Action(Model::default_load()))
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
    let (action_tx, rx) = channel(1000);

    let model = Model {
        catalog: Default::default(),
        action_tx: action_tx.clone()
    };

    let app_state = seed::App::build(model, update, view)
        .finish()
        .run();

    let middlewares: Vec<Box<dyn Handler>> = vec![
        Box::new(ContextMiddleware::<Env>::new()),
        Box::new(AddonsMiddleware::<Env>::new()),
    ];
    let container = Rc::new(ContainerHolder::new(CatalogGrouped::new()));
    let muxer = ContainerMuxer::new(
        middlewares,
        vec![
            ((), container.clone() as Rc<dyn ContainerInterface>)
        ],
        Box::new(move |ev| {
            if let MuxerEvent::NewState(_) = ev {
                app_state.update(Msg::CatalogUpdated(container.get_state_owned()));
            }
        }),
    );

    Env::exec(Box::new(
        rx
            .for_each(move |action| {
                muxer.dispatch(&action);
                future::ok(())
            })
    ));

    action_tx.clone().try_send(Model::default_load()).unwrap();
}
