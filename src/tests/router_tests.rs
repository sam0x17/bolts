use crate::router::*;

fn target(_url: &UrlParams, _get: &VerbParams, _post: &VerbParams) -> Render {
    Render::Plain("this is a test".to_string())
}

#[test]
fn test_route_requires_leading_slash() {
    let mut router = Router::new();
    assert_ne!(
        router.route(None, Verb::Get, "this/is/a/test", target),
        Ok(())
    );
}

#[test]
fn test_route_trailing_slash_optional() {
    let mut router = Router::new();
    assert_eq!(
        router.route(None, Verb::Get, "/this/is/a/test/", target),
        Ok(())
    );
    assert_eq!(
        router.route(None, Verb::Get, "/this/is/another/test", target),
        Ok(())
    );
}

#[test]
fn test_route_trailing_slash_non_trailing_slash_equivalent() {
    let mut router = Router::new();
    assert_eq!(
        router.route(None, Verb::Get, "/this/is/a/test/", target),
        Ok(())
    );
    assert_ne!(
        router.route(None, Verb::Get, "/this/is/a/test", target),
        Ok(())
    );
}

#[test]
fn test_route_parses_parts() {
    let mut router = Router::new();
    router
        .route(None, Verb::Patch, "/some/#string/cool/;f/:id/:id2", target)
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
        .route(Some(domain), Verb::Post, "/some/path", target)
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
        .route(Some(domain), Verb::Post, "/some/path", target)
        .unwrap();
    let route = router.routes().nth(0).unwrap();
    assert_eq!(route.verb, Verb::Post);
    assert_eq!(route.vars.len(), 0);
    assert_eq!(route.domain, Some(domain.to_string()));
}

#[test]
pub fn test_domain_rejection() {
    let mut router = Router::new();
    assert_ne!(
        router.route(Some(".bad.com"), Verb::Get, "/p", target),
        Ok(())
    );
    assert_ne!(
        router.route(Some(" bad.com"), Verb::Get, "/path", target),
        Ok(())
    );
    assert_ne!(router.route(Some("."), Verb::Get, "/path", target), Ok(()));
    assert_ne!(
        router.route(Some(".com"), Verb::Get, "/path", target),
        Ok(())
    );
    assert_ne!(
        router.route(Some("googl e.com"), Verb::Get, "/path", target),
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
            .route(target),
        Ok(())
    );
    assert_eq!(router.path("/hello").get().route(target), Ok(()));
}

#[test]
pub fn test_url_parameters() {
    let mut params = UrlParams::new();
    params.add("id", UrlParam::Int(36));
    params.add("name", UrlParam::String("sam".to_string()));
    params.add("score", UrlParam::Float(33.24));
    assert_eq!(params["id"], UrlParam::Int(36));
    assert_eq!(params["name"], UrlParam::String("sam".to_string()));
    assert_eq!(params["score"], UrlParam::Float(33.24));
}

#[test]
pub fn test_route_key_from_path() {
    let key = RouteKey::from_path(Verb::Get, &"/contact/:id".to_string()).unwrap();
    assert_eq!(key.verb, Verb::Get);
    assert_eq!(key.domain, None);
    assert_eq!(key.parts[0], RoutePart::Path("contact".to_string()));
    assert_eq!(key.parts[1], RoutePart::Int);
}

#[test]
pub fn test_route_key_from_path_with_domain() {
    let key = RouteKey::new(
        Verb::Get,
        &"/contact/:id".to_string(),
        Some(&"domain.com".to_string()),
    )
    .unwrap();
    assert_eq!(key.verb, Verb::Get);
    assert_eq!(key.domain, Some("domain.com".to_string()));
    assert_eq!(key.parts[0], RoutePart::Path("contact".to_string()));
    assert_eq!(key.parts[1], RoutePart::Int);
}

#[test]
pub fn test_router_find() {
    let mut router = Router::new();
    router
        .path("/hello/world")
        .domain("domain.com")
        .verb(Verb::Post)
        .route(target)
        .unwrap();
    router
        .path("/hello/puppet")
        .domain("domain.com")
        .verb(Verb::Patch)
        .route(target)
        .unwrap();
    router
        .path("/goodbye/:id")
        .verb(Verb::Delete)
        .route(target)
        .unwrap();
    router.find(Verb::Post, &("/hello/world".to_string()), Some(&("domain.com".to_string()))).unwrap();
    router.find(Verb::Patch, &("/hello/puppet".to_string()), Some(&("domain.com".to_string()))).unwrap();
    router.find(Verb::Delete, &("/goodbye/33".to_string()), None).unwrap();
}
