use lazy_static::lazy_static;
use regex::Regex;
use std::collections::*;
use std::hash::*;
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Verb {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum RouteVar {
    Int(String),
    Float(String),
    String(String),
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum RoutePart {
    Path(String),
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct RouteKey {
    parts: Vec<RoutePart>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Route {
    vars: Vec<RouteVar>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Router {
    routes: HashMap<RouteKey, Route>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn route(&mut self, verb: Verb, path: &'static str) -> Result<(), &'static str> {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"\A(/[^;#:\s/]+|/[:#;][^;#:\s/]+)*/?\z").unwrap();
        }
        if !REG.is_match(path) {
            return Err("invalid route format!");
        }
        let mut route_key = RouteKey {
            parts: Vec::new()
        };
        let mut route = Route {
            vars: Vec::new()
        };
        for token in path.split('/') {
            if token.len() == 0 { continue; }
            match token.chars().nth(0).unwrap() {
                ':' => { // integer var
                    route_key.parts.push(RoutePart::Int);
                    route.vars.push(RouteVar::Int((&token[1..]).to_string()));
                }
                '#' => { // string var
                    route_key.parts.push(RoutePart::String);
                    route.vars.push(RouteVar::String((&token[1..]).to_string()));
                }
                ';' => { // float var
                    route_key.parts.push(RoutePart::Float);
                    route.vars.push(RouteVar::Float((&token[1..]).to_string()));
                }
                _ => {
                    route_key.parts.push(RoutePart::Path(token.to_string()));
                }
            }
        }
        let size = self.routes.len();
        self.routes.insert(route_key, route);
        if size != self.routes.len() - 1 {
            return Err("a route identical to this one has already been defined!");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_route_requires_leading_slash() {
        let mut router = Router::new();
        assert_ne!(router.route(Verb::Get, "this/is/a/test"), Ok(()));
    }

    #[test]
    fn test_route_trailing_slash_optional() {
        let mut router = Router::new();
        assert_eq!(router.route(Verb::Get, "/this/is/a/test/"), Ok(()));
        assert_eq!(router.route(Verb::Get, "/this/is/a/test"), Ok(()));
    }
}
