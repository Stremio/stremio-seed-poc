use seed::{prelude::*, *};
use std::str::FromStr;
use stremio_core::types::addons::{ParseResourceErr, ResourceRef, ResourceRequest};

// ------ Route ------

#[derive(Clone, Eq, PartialEq)]
pub enum Route {
    Board,
    Discover(Option<ResourceRequest>),
    Detail {
        type_name: String,
        id: String,
        video_id: Option<String>,
    },
    Player,
    Addons(Option<ResourceRequest>),
    NotFound,
}

impl Route {
    pub fn to_href(&self) -> String {
        match self {
            Self::Board => "#/board".into(),
            Self::Discover(req) => format!("#/discover{}", resource_request_to_url_path(req)),
            Self::Detail {
                type_name,
                id,
                video_id,
            } => format!(
                "#/detail/{}/{}/{}",
                type_name,
                id,
                video_id.as_ref().map(String::as_str).unwrap_or_default()
            ),
            Self::Player => "#/player".into(),
            Self::Addons(req) => format!("#/addons{}", resource_request_to_url_path(req)),
            Self::NotFound => "#/404".into(),
        }
    }
}

impl From<Url> for Route {
    fn from(url: Url) -> Self {
        let hash = match url.hash {
            Some(hash) => hash,
            None => match url.path.first().map(String::as_str) {
                None | Some("") => return Self::Board,
                _ => return Self::NotFound,
            },
        };
        let mut hash = hash.split('/');
        // skip the part before the first `/`
        hash.next();

        match hash.next() {
            Some("") | Some("board") => Self::Board,
            Some("discover") => {
                let encoded_base = if let Some(base) = hash.next() {
                    base
                } else {
                    return Self::Discover(None);
                };

                let encoded_path = if let Some(base) = hash.next() {
                    base
                } else {
                    error!("cannot find request path");
                    return Self::NotFound;
                };

                let req = match resource_request_try_from_url_parts(encoded_base, encoded_path) {
                    Ok(req) => req,
                    Err(error) => {
                        error!(error);
                        return Self::NotFound;
                    }
                };

                Self::Discover(Some(req))
            }
            Some("detail") => {
                let type_name = if let Some(type_name) = hash.next() {
                    type_name.to_owned()
                } else {
                    error!("cannot find detail type_name");
                    return Self::NotFound;
                };

                let id = if let Some(id) = hash.next() {
                    id.to_owned()
                } else {
                    error!("cannot find detail id");
                    return Self::NotFound;
                };

                let video_id = hash.next().map(ToOwned::to_owned);

                Self::Detail {
                    type_name,
                    id,
                    video_id,
                }
            }
            Some("player") => Self::Player,
            Some("addons") => {
                let encoded_base = if let Some(base) = hash.next() {
                    base
                } else {
                    return Self::Addons(None);
                };

                let encoded_path = if let Some(base) = hash.next() {
                    base
                } else {
                    error!("cannot find request path");
                    return Self::NotFound;
                };

                let req = match resource_request_try_from_url_parts(encoded_base, encoded_path) {
                    Ok(req) => req,
                    Err(error) => {
                        error!(error);
                        return Self::NotFound;
                    }
                };

                Self::Addons(Some(req))
            }
            _ => Self::NotFound,
        }
    }
}

// ------ ResourceRequest conversion  ------

// @TODO make it less ugly and add into stremio-core?

fn resource_request_to_url_path(req: &Option<ResourceRequest>) -> String {
    let req = if let Some(req) = req {
        req
    } else {
        return String::new();
    };

    let encoded_base = String::from(js_sys::encode_uri_component(&req.base));
    let encoded_path = String::from(js_sys::encode_uri_component(&req.path.to_string()));
    format!("/{}/{}", encoded_base, encoded_path)
}

#[derive(Debug)]
enum ParseResourceRequestError {
    UriDecode(String),
    Resource(ParseResourceErr),
}

fn resource_request_try_from_url_parts(
    uri_encoded_base: &str,
    uri_encoded_path: &str,
) -> Result<ResourceRequest, ParseResourceRequestError> {
    let base: String = {
        js_sys::decode_uri_component(uri_encoded_base)
            .map_err(|_| ParseResourceRequestError::UriDecode(uri_encoded_base.to_owned()))?
            .into()
    };

    let path: String = {
        js_sys::decode_uri_component(uri_encoded_path)
            .map_err(|_| ParseResourceRequestError::UriDecode(uri_encoded_path.to_owned()))?
            .into()
    };

    Ok(ResourceRequest {
        base,
        path: ResourceRef::from_str(&path).map_err(ParseResourceRequestError::Resource)?,
    })
}
