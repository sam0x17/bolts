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
    domain: Option<String>,
    parts: Vec<RoutePart>,
    verb: Verb,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Route {
    domain: Option<String>,
    vars: Vec<RouteVar>,
    verb: Verb,
}

pub struct RouteBuilder<'a> {
    domain: Option<&'static str>,
    verb: Verb,
    path: &'static str,
    router: &'a mut Router,
}

impl<'a> RouteBuilder<'a> {
    pub fn domain(mut self, domain: &'static str) -> RouteBuilder<'a> {
        self.domain = Some(domain);
        self
    }

    pub fn verb(mut self, verb: Verb) -> RouteBuilder<'a> {
        self.verb = verb;
        self
    }

    pub fn route(self) -> Result<(), &'static str> {
        self.router.route(self.domain, self.verb, self.path)
    }
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

    pub fn path(&mut self, path: &'static str) -> RouteBuilder {
        RouteBuilder {
            domain: None,
            verb: Verb::Get,
            path: path,
            router: self,
        }
    }

    pub fn routes(&self) -> hash_map::Values<'_, RouteKey, Route> {
        self.routes.values()
    }

    pub fn route(
        &mut self,
        domain: Option<&'static str>,
        verb: Verb,
        path: &'static str,
    ) -> Result<(), &'static str> {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"\A(/[^;#:\s/]+|/[:#;][^;#:\s/]+)*/?\z").unwrap();
        }
        lazy_static! {
            static ref DOM_REG_SIMPLE: Regex = Regex::new(r"\A([^\.\*\s]+\.[^\.\s]+)+\z").unwrap();
        }
        lazy_static! {
            static ref DOM_REG_WILDCARD: Regex =
                Regex::new(r"\A\*\.([^\.\*\s]+\.[^\.\s]+)+\z").unwrap();
        }
        if !REG.is_match(path) {
            return Err("invalid route format!");
        }
        let domain = match domain {
            Some(dom) => {
                let dom = dom.to_lowercase();
                if !DOM_REG_SIMPLE.is_match(&dom) && !DOM_REG_WILDCARD.is_match(&dom) {
                    return Err("invalid domain!");
                }
                Some(dom.to_string())
            }
            None => None,
        };
        let mut route_key = RouteKey {
            domain: domain.clone(),
            parts: Vec::new(),
            verb: verb.clone(),
        };
        let mut route = Route {
            domain: domain,
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
        assert_ne!(router.route(None, Verb::Get, "this/is/a/test"), Ok(()));
    }

    #[test]
    fn test_route_trailing_slash_optional() {
        let mut router = Router::new();
        assert_eq!(router.route(None, Verb::Get, "/this/is/a/test/"), Ok(()));
        assert_eq!(
            router.route(None, Verb::Get, "/this/is/another/test"),
            Ok(())
        );
    }

    #[test]
    fn test_route_trailing_slash_non_trailing_slash_equivalent() {
        let mut router = Router::new();
        assert_eq!(router.route(None, Verb::Get, "/this/is/a/test/"), Ok(()));
        assert_ne!(router.route(None, Verb::Get, "/this/is/a/test"), Ok(()));
    }

    #[test]
    fn test_route_parses_parts() {
        let mut router = Router::new();
        router
            .route(None, Verb::Patch, "/some/#string/cool/;f/:id/:id2")
            .unwrap();
        let routes = router.routes();
        assert_eq!(routes.len(), 1);
        let route = router.routes().nth(0).unwrap();
        assert_eq!(route.verb, Verb::Patch);
        assert_eq!(route.vars.len(), 4);
        assert_eq!(route.vars[0], RouteVar::String("string"));
        assert_eq!(route.vars[1], RouteVar::Float("f"));
        assert_eq!(route.vars[2], RouteVar::Int("id"));
        assert_eq!(route.vars[3], RouteVar::Int("id2"));
    }

    #[test]
    pub fn test_domain() {
        let mut router = Router::new();
        let domain = "my-cool-domain.co.uk";
        router
            .route(Some(domain), Verb::Post, "/some/path")
            .unwrap();
        let route = router.routes().nth(0).unwrap();
        assert_eq!(route.verb, Verb::Post);
        assert_eq!(route.vars.len(), 0);
        assert_eq!(route.domain, Some(domain.to_string()));
    }

    #[test]
    pub fn test_domain_wildcard() {
        let mut router = Router::new();
        let domain = "*.staging.mysite.something.com";
        router
            .route(Some(domain), Verb::Post, "/some/path")
            .unwrap();
        let route = router.routes().nth(0).unwrap();
        assert_eq!(route.verb, Verb::Post);
        assert_eq!(route.vars.len(), 0);
        assert_eq!(route.domain, Some(domain.to_string()));
    }

    #[test]
    pub fn test_domain_rejection() {
        let mut router = Router::new();
        assert_ne!(router.route(Some(".bad.com"), Verb::Get, "/p"), Ok(()));
        assert_ne!(router.route(Some(" bad.com"), Verb::Get, "/path"), Ok(()));
        assert_ne!(router.route(Some("."), Verb::Get, "/path"), Ok(()));
        assert_ne!(router.route(Some(".com"), Verb::Get, "/path"), Ok(()));
        assert_ne!(
            router.route(Some("googl e.com"), Verb::Get, "/path"),
            Ok(())
        );
    }

    #[test]
    pub fn test_route_builder() {
        let mut router = Router::new();
        assert_eq!(
            router
                .path("/hello/world")
                .domain("domain.com")
                .verb(Verb::Post)
                .route(),
            Ok(())
        );
    }
}
