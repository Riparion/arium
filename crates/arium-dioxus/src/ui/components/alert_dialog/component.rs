use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps, AlertDialogRootProps, AlertDialogTitleProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

// See `crate::styled_module` for why we declare the Asset separately.
crate::styled_module!(const ALERT_DIALOG_CSS = "/src/ui/components/alert_dialog/dx-alert-dialog.css");

/// Modal confirmation dialog. Compose with [`AlertDialogTitle`],
/// [`AlertDialogDescription`], and [`AlertDialogActions`] children.
#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    // The primitive forwards `attributes` onto the backdrop element; passing
    // `class:` as a separate prop adds the catalog class to that attributes
    // vec, so any caller-supplied `class:` ends up emitted twice. Merge first.
    let base = attributes!(div {
        class: Styles::dx_alert_dialog_backdrop
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogRoot {
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: merged,
            // AlertDialogContent exposes `class: Option<String>` as a dedicated
            // string slot, so this one is safe to pass directly.
            alert_dialog::AlertDialogContent {
                class: Styles::dx_alert_dialog.to_string(),
                {props.children}
            }
        }
    }
}

/// Heading text inside an [`AlertDialog`].
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_alert_dialog_title
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogTitle { attributes: merged, {props.children} }
    }
}

/// Body text inside an [`AlertDialog`].
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_alert_dialog_description
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogDescription { attributes: merged, {props.children} }
    }
}

/// Footer row holding [`AlertDialogCancel`] / [`AlertDialogAction`] buttons.
#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_alert_dialog_actions
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogActions { attributes: merged, {props.children} }
    }
}

/// Dismiss button — closes the dialog without confirming.
#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_alert_dialog_cancel
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogCancel { on_click: props.on_click, attributes: merged, {props.children} }
    }
}

/// Confirm button — closes the dialog and invokes `on_click`.
#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_alert_dialog_action
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        document::Stylesheet { href: ALERT_DIALOG_CSS }
        alert_dialog::AlertDialogAction { on_click: props.on_click, attributes: merged, {props.children} }
    }
}
