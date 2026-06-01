use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::label::{self, LabelProps};
use dioxus_primitives::merge_attributes;

// See `crate::styled_module` for why we declare the Asset separately.
crate::styled_module!(const LABEL_CSS = "/src/ui/components/label/dx-label.css");

/// Themed `<label>` — pass `html_for: "input-id"` to wire it to a primitive.
#[component]
pub fn Label(props: LabelProps) -> Element {
    // Merge the catalog class with the caller's attributes before forwarding
    // to the primitive — otherwise the primitive spreads both as separate
    // `class=` attributes and the HTML parser drops one (the caller's class
    // and the catalog class can't coexist without this).
    let base = attributes!(label {
        class: Styles::dx_label
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: LABEL_CSS }
        label::Label {
            html_for: props.html_for,
            attributes: merged,
            {props.children}
        }
    }
}
