use dioxus::prelude::*;
use dioxus_icons::lucide::Check;
use dioxus_primitives::checkbox::{self, CheckboxProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

// See `crate::styled_module` for why we declare the Asset separately.
crate::styled_module!(const CHECKBOX_CSS = "/src/ui/components/checkbox/dx-checkbox.css");

/// Themed checkbox primitive.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    // Merge the catalog class into `props.attributes` before forwarding —
    // emitting `class:` as a separate prop alongside `attributes: props.attributes`
    // adds the catalog class to a second slot in the inner vec, and the primitive
    // spreads everything onto its `<button>` (yielding two `class=` attributes
    // that the HTML parser silently de-dupes).
    let base = attributes!(button {
        class: Styles::dx_checkbox
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: CHECKBOX_CSS }
        checkbox::Checkbox {
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: merged,
            checkbox::CheckboxIndicator { class: Styles::dx_checkbox_indicator,
                Check { size: "1rem" }
            }
        }
    }
}
