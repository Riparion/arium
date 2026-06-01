use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

// `CARD_CSS` is the explicit Asset declaration for the `#[css_module]` style.css
// below — used to render a `document::Stylesheet { href: CARD_CSS }` from every
// component in the module so the link tag survives SSR + post-hydration remount
// (the `#[css_module]` macro's own emit is a one-shot OnceLock, fine for fully
// client-rendered apps but unreliable here; see `crate::styled_module` for the
// full explanation).
crate::styled_module!(const CARD_CSS = "/src/ui/components/card/dx-card.css");

/// Outer card surface. Compose with `Card*` subcomponents inside.
#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // Merge the catalog class with the caller's attributes — emitting a literal
    // `class:` alongside `..attributes` produces two `class=` attributes in the
    // HTML, of which the browser keeps only the first, silently dropping the
    // caller's class (and any subsequent class composition like Tailwind).
    let base = attributes!(div {
        class: Styles::dx_card,
        "data-slot": "card",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Top section of a [`Card`] — typically holds title + description + action.
#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_header,
        "data-slot": "card-header",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Heading text inside a [`CardHeader`].
#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_title,
        "data-slot": "card-title",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Secondary text inside a [`CardHeader`].
#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_description,
        "data-slot": "card-description",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Right-aligned action slot inside a [`CardHeader`].
#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_action,
        "data-slot": "card-action",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Main body of a [`Card`].
#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_content,
        "data-slot": "card-content",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}

/// Bottom section of a [`Card`] — typically holds buttons or status.
#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_footer,
        "data-slot": "card-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: CARD_CSS }
        div { ..merged, {children} }
    }
}
