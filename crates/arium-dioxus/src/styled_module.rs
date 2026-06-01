//! [`styled_module!`](macro@crate::styled_module) — single-statement form of the css-module + asset pair
//! every catalog widget needs.
//!
//! ## The pattern this replaces
//!
//! Every widget that participates in the SSR/remount-safe styling pipeline
//! (see [`crate::ui::AuthStylesheets`] for the full rationale) needs **two**
//! declarations pointing at the same `style.css`:
//!
//! ```ignore
//! use dioxus::prelude::*;
//!
//! pub(crate) const INPUT_CSS: Asset = asset!(
//!     "/src/ui/components/input/dx-input.css",
//!     AssetOptions::css_module()
//! );
//!
//! #[css_module("/src/ui/components/input/dx-input.css")]
//! struct Styles;
//! ```
//!
//! The path is repeated twice and the same dance shows up in every catalog
//! widget. Easy to forget one half and silently ship an unstyled widget.
//!
//! ## With this macro
//!
//! ```ignore
//! use dioxus::prelude::*;
//!
//! arium_dioxus::styled_module!(
//!     pub(crate) const INPUT_CSS = "/src/ui/components/input/dx-input.css"
//! );
//! ```
//!
//! Expands to the same two declarations. The struct is fixed to `Styles` (the
//! convention every catalog widget uses); if a module needs a different name
//! or more than one css_module per file, fall back to writing both lines by
//! hand.
//!
//! Caller must have `Asset`, `asset!`, `AssetOptions`, and `css_module` in
//! scope. `use dioxus::prelude::*;` covers all four.

/// Declare a CSS-module asset and its accompanying `Styles` struct in one shot.
/// See the module docs for the expansion shape and the caller-side imports it
/// assumes.
#[macro_export]
macro_rules! styled_module {
    ($vis:vis const $name:ident = $path:literal $(,)?) => {
        $vis const $name: Asset = asset!($path, AssetOptions::css_module());

        #[css_module($path)]
        struct Styles;
    };
}
