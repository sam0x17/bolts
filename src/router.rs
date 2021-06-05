use lazy_static::lazy_static;
use regex::Regex;
use std::collections::*;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum HttpVerb {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

#[derive(Debug, PartialEq, Clone, Hash)]
enum RouteVar {
    Int(String),
    Float(String),
    String(String),
}

#[derive(Debug, PartialEq, Clone, Hash)]
struct Route {
    path: String,
    vars: Vec<RouteVar>,
    verb: HttpVerb,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Router {
    routes: HashMap<String, Route>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn route(&mut self, verb: HttpVerb, path: &'static str) -> Result<(), &'static str> {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"\A(/[^;#:\s/]+|/[:#;][^;#:\s/]+)*/?\z").unwrap();
        }
        if !REG.is_match(path) {
            return Err("invalid route format!");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_route_require_leading_slash() {
        let mut router = Router::new();
        assert_ne!(router.route(HttpVerb::Get, "this/is/a/test"), Ok(()));
    }
}
