use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

// See `crate::styled_module` for why we declare the Asset separately.
crate::styled_module!(const SKELETON_CSS = "/src/ui/components/skeleton/dx-skeleton.css");

/// Pulsing placeholder shown while async content is loading.
#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_skeleton,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: SKELETON_CSS }
        div { ..merged }
    }
}
