//use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ExecutionArgs<'r> {
    target_dir: &'r str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (target_dir, set_target_dir) = create_signal(cx, String::new());
    let (visible, set_visible) = create_signal(cx, false);

    let handle_upload_directory = move |event: leptos::ev::Event| {
        set_target_dir.update(|value| *value = event_target_value(&event).to_string());

        if !target_dir.get().is_empty() {
            set_visible.update(|value| *value = true);
        }
    };

    let handle_execution = move |event: leptos::ev::MouseEvent| {
        event.prevent_default();
        spawn_local(async move {
            if target_dir.get().is_empty() {
                return;
            }

            if let Ok(args) = to_value(&ExecutionArgs {
                target_dir: &target_dir.get(),
            }) {
                invoke("exec", args).await.as_string().unwrap();
            }
        });
    };

    let execute_button = move || {
        if visible.get() {
            view! { cx, <button type="submit" class="exec-button" on:click=handle_execution>"Execute"</button> }
        } else {
            view! { cx, <button class="hidden-exec-button"></button> }
        }
    };

    view! { cx,
        <main class="container">
            <input
                type="text"
                name="dir-holder"
                class="directory-placeholder"
                placeholder="Directory"
                readonly="true"
                value=move || target_dir.get()
            />
            <div class="action-section">
                <label class="directory-upload-button">
                    <input
                        type="file"
                        class="directory-uploader"
                        name="target-dir"
                        on:change=handle_upload_directory
                    />
                    "Choose Directory"
                </label>
                {execute_button}
            </div>
        </main>
    }
}
