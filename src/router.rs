use lazy_static::lazy_static;
use regex::Regex;
use std::collections::*;
use std::hash::*;
use std::ops::Index;

lazy_static! {
    static ref PATH_REG: Regex = Regex::new(r"\A(/[^;#:\s/]+|/[:#;][^;#:\s/]+)*/?\z").unwrap();
}
lazy_static! {
    static ref DOM_REG_SIMPLE: Regex = Regex::new(r"\A([^\.\*\s]+\.[^\.\s]+)+\z").unwrap();
}
lazy_static! {
    static ref DOM_REG_WILDCARD: Regex = Regex::new(r"\A\*\.([^\.\*\s]+\.[^\.\s]+)+\z").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerbParam {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VerbParams {
    hashmap: HashMap<String, VerbParam>,
}

impl Index<&'static str> for VerbParams {
    type Output = VerbParam;
    fn index(&self, key: &'static str) -> &Self::Output {
        &self.hashmap[key]
    }
}

impl VerbParams {
    pub fn new() -> Self {
        VerbParams {
            hashmap: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: VerbParam) {
        self.hashmap.insert(key, value);
    }
}

pub enum Render {
    Plain(String),
    File(String, String),
    Mime(String, String),
    Json(String),
}

pub type Endpoint = fn(&UrlParams, &VerbParams, &VerbParams) -> Render;

#[derive(Debug, PartialEq, Clone)]
pub enum UrlParam {
    String(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UrlParams {
    hashmap: HashMap<&'static str, UrlParam>,
}

impl Index<&'static str> for UrlParams {
    type Output = UrlParam;
    fn index(&self, key: &'static str) -> &Self::Output {
        &self.hashmap[key]
    }
}

impl UrlParams {
    pub fn new() -> Self {
        UrlParams {
            hashmap: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &'static str, value: UrlParam) {
        self.hashmap.insert(key, value);
    }
}

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
pub enum RouteVar {
    Int(&'static str),
    Float(&'static str),
    String(&'static str),
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum RoutePart {
    Path(String),
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct RouteKey {
    pub domain: Option<String>,
    pub parts: Vec<RoutePart>,
    pub verb: Verb,
}

impl RouteKey {
    pub fn verb(&self) -> Verb {
        self.verb.clone()
    }

    pub fn parts(&self) -> &Vec<RoutePart> {
        &self.parts
    }

    pub fn domain(&self) -> &Option<String> {
        &self.domain
    }

    pub fn from_path(verb: Verb, path: &String) -> Result<RouteKey, &'static str> {
        RouteKey::new(verb, path, None)
    }

    pub fn new(
        verb: Verb,
        path: &String,
        domain: Option<&String>,
    ) -> Result<RouteKey, &'static str> {
        if !PATH_REG.is_match(path) {
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
        for token in path.split('/') {
            if token.len() == 0 {
                continue;
            }
            match token.chars().nth(0).unwrap() {
                ':' => {
                    // integer var
                    route_key.parts.push(RoutePart::Int);
                }
                '#' => {
                    // string var
                    route_key.parts.push(RoutePart::String);
                }
                ';' => {
                    // float var
                    route_key.parts.push(RoutePart::Float);
                }
                _ => {
                    route_key.parts.push(RoutePart::Path(token.to_string()));
                }
            }
        }
        Ok(route_key)
    }
}

#[derive(Clone)]
pub struct Route {
    pub domain: Option<String>,
    pub vars: Vec<RouteVar>,
    pub verb: Verb,
    pub target: Endpoint,
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

    pub fn post(mut self) -> RouteBuilder<'a> {
        self.verb = Verb::Post;
        self
    }

    pub fn get(mut self) -> RouteBuilder<'a> {
        self.verb = Verb::Get;
        self
    }

    pub fn put(mut self) -> RouteBuilder<'a> {
        self.verb = Verb::Put;
        self
    }

    pub fn patch(mut self) -> RouteBuilder<'a> {
        self.verb = Verb::Patch;
        self
    }

    pub fn delete(mut self) -> RouteBuilder<'a> {
        self.verb = Verb::Delete;
        self
    }

    pub fn route(self, target: Endpoint) -> Result<(), &'static str> {
        self.router.route(self.domain, self.verb, self.path, target)
    }
}

#[derive(Clone)]
pub struct Router {
    routes: HashMap<RouteKey, Route>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn find(&self, verb: Verb, path: &String, domain: Option<&String>) -> Option<Endpoint> {
        let key = match RouteKey::new(verb, path, domain) {
            Ok(k) => k,
            _ => return None,
        };
        match self.routes.get(&key) {
            Some(route) => Some(route.target),
            None => None,
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
        target: Endpoint,
    ) -> Result<(), &'static str> {
        if !PATH_REG.is_match(path) {
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
            target: target,
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
