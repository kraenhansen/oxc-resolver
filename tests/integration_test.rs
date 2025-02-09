//! Test public APIs

use std::{env, path::PathBuf};

use oxc_resolver::{EnforceExtension, Resolution, ResolveContext, ResolveOptions, Resolver};

fn dir() -> PathBuf {
    env::current_dir().unwrap()
}

fn resolve(specifier: &str) -> Resolution {
    let path = dir();
    Resolver::new(ResolveOptions::default()).resolve(path, specifier).unwrap()
}

#[test]
fn clone() {
    let resolution = resolve("./tests/package.json");
    assert_eq!(resolution.clone(), resolution);
}

#[test]
fn debug() {
    let resolution = resolve("./tests/package.json");
    let s = format!("{:?}", resolution);
    assert!(!s.is_empty());
}

#[test]
fn eq() {
    let resolution = resolve("./tests/package.json");
    assert_eq!(resolution, resolution);
}

#[test]
fn package_json() {
    let resolution = resolve("./tests/package.json");
    assert!(resolution.package_json().is_some_and(|json| json.raw_json().is_object()));
}

#[test]
fn clear_cache() {
    let resolver = Resolver::new(ResolveOptions::default());
    resolver.clear_cache(); // exists
}

#[test]
fn options() {
    let resolver = Resolver::new(ResolveOptions::default());
    let options = resolver.options();
    assert!(!format!("{options:?}").is_empty());
}

#[test]
fn debug_resolver() {
    let resolver = Resolver::new(ResolveOptions::default());
    assert!(!format!("{resolver:?}").is_empty());
}

#[test]
fn dependencies() {
    let path = dir();
    let mut ctx = ResolveContext::default();
    let _ = Resolver::new(ResolveOptions::default()).resolve_with_context(
        path,
        "./tests/package.json",
        &mut ctx,
    );
    assert!(!ctx.file_dependencies.is_empty());
    assert!(ctx.missing_dependencies.is_empty());
}

#[test]
fn options_api() {
    _ = ResolveOptions::default()
        .with_condition_names(&[])
        .with_builtin_modules(true)
        .with_root(PathBuf::new())
        .with_extension(".js")
        .with_force_extension(EnforceExtension::Auto)
        .with_main_field("asdf")
        .with_fully_specified(true)
        .with_prefer_relative(true)
        .with_prefer_absolute(true);
}
