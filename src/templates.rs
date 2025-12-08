//! Template loader for embedded gitignore templates.
//!
//! Templates are auto-generated from the `templates/` submodule at compile time.
//! See `build.rs` for the generation logic.

// Include the auto-generated templates
include!(concat!(env!("OUT_DIR"), "/templates_gen.rs"));
