//use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::{ev::Event, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (target_dir, set_target_dir) = create_signal(cx, String::new());

    let handle_upload_directory = move |event: Event| {
        if let Some(current_target) = event.current_target() {
            println!("{:?}", current_target);
        }
    };

    view! { cx,
        <main class="container">
            <input
                type="text"
                placeholder="Directory"
                value=move || target_dir.get()
            />
            <input
                type="file"
                class="directory-upload"
                on:change=handle_upload_directory
            />
            <label
                for="target_dir"
                class="directory-upload-button"
            >
                "Choose directory"
            </label>
        </main>
    }
}
