#[macro_use]
extern crate seed;
use seed::prelude::*;
use stremio_core::state_types::*;
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
    h3![
        format!("Hello, World × {}", model.catalog.groups.len())
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
                let state = container.get_state_owned();
                if state.groups.iter().all(|x| match x.1 {
                    Loadable::Loading => false,
                    _ => true
                }) {
                    app_state.update(Msg::CatalogUpdated(state));
                }
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
