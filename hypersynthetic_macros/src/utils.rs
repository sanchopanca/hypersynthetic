use quote::ToTokens as _;
use syn::{Ident, Path};

/// The method that turns a props builder into the props. `#[component]` gives it this
/// name and `html!` calls it. typed-builder's own name for it is `build`, which a
/// prop couldn't be called then: a prop's setter is a method of the builder too.
/// Any name takes one away from props; this one nobody will miss.
pub const BUILD_METHOD: &str = "__hs_build";

pub fn is_path_pascal_case(path: &Path) -> bool {
    is_pascal_case(extract_ident_from_path(path))
}

pub fn extract_ident_from_path(path: &Path) -> &Ident {
    // A parsed `Path` always has at least one segment
    &path.segments.last().unwrap().ident
}

pub fn path_to_string(path: &Path) -> String {
    path.to_token_stream()
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

// This is one lazy implementation that only checks that the first character is uppercase
pub fn is_pascal_case(name: &Ident) -> bool {
    let first_char = name.to_string().chars().next();
    matches!(first_char, Some(ch) if ch.is_uppercase())
}
