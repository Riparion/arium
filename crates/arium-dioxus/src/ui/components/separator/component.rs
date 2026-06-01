use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

// See `crate::styled_module` for why we declare the Asset separately.
crate::styled_module!(const SEPARATOR_CSS = "/src/ui/components/separator/dx-separator.css");

/// Horizontal or vertical hairline. Set `horizontal: false` for a vertical
/// rule; set `decorative: true` to keep it out of the accessibility tree.
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_separator,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: SEPARATOR_CSS }
        separator::Separator {
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: merged,
            {props.children}
        }
    }
}
