use ambient_api::{
    element::{use_module_message, use_module_message_effect, use_state, use_state_with},
    prelude::*,
};

use crate::packages::{
    input_schema::messages::{InputRelease, InputRequest},
    this::messages,
};

#[element_component]
pub fn PackageLoad(_hooks: &mut Hooks) -> Element {
    Group::el([PackageLoadDialog::el(), ErrorMessage::el()])
}

#[element_component]
fn PackageLoadDialog(hooks: &mut Hooks) -> Element {
    let (visible, set_visible) = use_state(hooks, false);
    use_module_message::<messages::PackageLoadShow>(hooks, {
        let set_visible = set_visible.clone();
        move |_, _, _| {
            set_visible(true);
        }
    });

    let close = cb(move || set_visible(false));
    Window::el(
        "Package load".to_string(),
        visible,
        Some(close.clone()),
        PackageLoadDialogInner::el(close),
    )
}

#[element_component]
fn PackageLoadDialogInner(hooks: &mut Hooks, close: Cb<dyn Fn() + Sync + Send>) -> Element {
    use_module_message_effect::<InputRequest, InputRelease>(hooks, None);
    let (url, set_url) = use_state_with(hooks, |_| String::new());

    FlowColumn::el([
        Text::el("Enter URL/deployment ID for a built package:").with_margin_even(STREET),
        TextEditor::new(url, set_url.clone())
            .auto_focus()
            .placeholder(Some("URL/deployment ID"))
            .on_submit(move |url| {
                messages::PackageLoad { url }.send_server_reliable();
                set_url(String::new());
                close();
            })
            .el()
            .with_background(vec4(0.0, 0.0, 0.0, 0.5))
            .with_padding_even(4.0)
            .with(fit_horizontal(), Fit::Parent)
            .with(min_height(), 22.0)
            .with(margin(), vec4(0.0, STREET, STREET, STREET)),
    ])
    .with(min_width(), 600.0)
}

#[element_component]
fn ErrorMessage(hooks: &mut Hooks) -> Element {
    let (reason, set_reason) = use_state(hooks, None);
    use_module_message::<messages::PackageLoadFailure>(hooks, {
        let set_reason = set_reason.clone();
        move |_, source, msg| {
            if !source.server() {
                return;
            }
            set_reason(Some(msg.reason.clone()));
        }
    });
    let close = cb(move || set_reason(None));
    Window::el(
        "Package load fail".to_string(),
        reason.is_some(),
        Some(close.clone()),
        ErrorMessageInner::el(reason.unwrap_or_default(), close),
    )
}

#[element_component]
fn ErrorMessageInner(
    hooks: &mut Hooks,
    reason: String,
    close: Cb<dyn Fn() + Send + Sync>,
) -> Element {
    use_module_message_effect::<InputRequest, InputRelease>(hooks, None);
    FlowColumn::el([Text::el(reason), Button::new("OK", move |_| close()).el()])
        .with(space_between_items(), 4.0)
        .with_margin_even(STREET)
}
