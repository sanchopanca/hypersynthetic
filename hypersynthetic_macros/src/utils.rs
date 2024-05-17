use quote::ToTokens as _;
use syn::{Ident, Path};

pub fn is_path_pascal_case(path: &Path) -> bool {
    is_pascal_case(&extract_ident_from_path(path))
}

pub fn extract_ident_from_path(path: &Path) -> Ident {
    path.segments.last().unwrap().ident.clone()
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
