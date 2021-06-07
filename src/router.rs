use lazy_static::lazy_static;
use regex::Regex;
use std::collections::*;
use std::hash::*;

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
    Int(&'static str),
    Float(&'static str),
    String(&'static str),
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum RoutePart {
    Path(String),
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct RouteKey {
    parts: Vec<RoutePart>,
    verb: Verb,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Route {
    vars: Vec<RouteVar>,
    verb: Verb,
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

    pub fn routes(&self) -> Vec<Route> {
        let mut vec = Vec::new();
        for route in self.routes.values() {
            vec.push(route.clone());
        }
        vec
    }

    pub fn route(&mut self, verb: Verb, path: &'static str) -> Result<(), &'static str> {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"\A(/[^;#:\s/]+|/[:#;][^;#:\s/]+)*/?\z").unwrap();
        }
        if !REG.is_match(path) {
            return Err("invalid route format!");
        }
        let mut route_key = RouteKey {
            parts: Vec::new(),
            verb: verb.clone(),
        };
        let mut route = Route {
            vars: Vec::new(),
            verb: verb,
        };
        for token in path.split('/') {
            if token.len() == 0 {
                continue;
            }
            match token.chars().nth(0).unwrap() {
                ':' => {
                    // integer var
                    route_key.parts.push(RoutePart::Int);
                    route.vars.push(RouteVar::Int(&token[1..]));
                }
                '#' => {
                    // string var
                    route_key.parts.push(RoutePart::String);
                    route.vars.push(RouteVar::String(&token[1..]));
                }
                ';' => {
                    // float var
                    route_key.parts.push(RoutePart::Float);
                    route.vars.push(RouteVar::Float(&token[1..]));
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
        assert_eq!(router.route(Verb::Get, "/this/is/another/test"), Ok(()));
    }

    #[test]
    fn test_route_trailing_slash_non_trailing_slash_equivalent() {
        let mut router = Router::new();
        assert_eq!(router.route(Verb::Get, "/this/is/a/test/"), Ok(()));
        assert_ne!(router.route(Verb::Get, "/this/is/a/test"), Ok(()));
    }

    #[test]
    fn test_route_parses_parts() {
        let mut router = Router::new();
        router
            .route(Verb::Patch, "/some/#string/cool/;f/:id/:id2")
            .unwrap();
        let routes = router.routes();
        assert_eq!(routes.len(), 1);
        let route = &routes[0];
        assert_eq!(route.verb, Verb::Patch);
        assert_eq!(route.vars.len(), 4);
        assert_eq!(route.vars[0], RouteVar::String("string"));
        assert_eq!(route.vars[1], RouteVar::Float("f"));
        assert_eq!(route.vars[2], RouteVar::Int("id"));
        assert_eq!(route.vars[3], RouteVar::Int("id2"));
    }
}
