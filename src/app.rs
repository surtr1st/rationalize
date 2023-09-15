use leptos::*;
use leptos::leptos_dom::ev;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use crate::wasm_bingen_tauri_api::*;
use crate::types::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (target_dir, set_target_dir) = create_signal(cx, String::new());
    let (total_items, set_total_items) = create_signal(cx, String::new());
    let (visible, set_visible) = create_signal(cx, false);

    let handle_open_dir = move |event: ev::MouseEvent| {
        event.prevent_default();
        spawn_local(async move {
            if let Ok(args) = to_value(&OpenDialogOptions {
                directory: true,
                title: "Choose directory to be executed"
            }) {
                if let Some(dir) = open(args).await.as_string() {
                    if dir.is_empty() {
                        set_visible.set(false);
                        return;
                    }
                    set_target_dir.set(dir);
                    set_visible.set(true);

                    if let Ok(arg) = to_value(&Directory { path: &target_dir.get() }) {
                        let result = invoke("retrieve_total_items", arg).await;
                        let js_value = JsValue::from(result);
                        if let Some(value) =  DirItems::extract_object(js_value).await {
                            let total_in_text = format!("Folders: {}, Files: {}", value.folders, value.files);
                            set_total_items.set(total_in_text);
                        }
                    }
                }
            }
        }) 
    };

    let handle_execution = move |event: ev::MouseEvent| {
        event.prevent_default();
        spawn_local(async move {
            if target_dir.get().is_empty() {
                return;
            }
            if let Ok(args) = to_value(&ExecutionArgs {
                target_dir: &target_dir.get(),
            }) {
                if let Some(result) = invoke("exec", args).await.as_string() {
                    if let Ok(options) = to_value(&MessageDialogOptions { title: "Notify" }) {
                        message(&result, options).await;
                    }
                }
            }
        });
    };
 

    let execute_button = move || {
        if visible.get() {
            view! { cx, <button class="exec-button" on:click=handle_execution>"Execute"</button> }
        } else {
            view! { cx, <button class="hidden-exec-button"></button> }
        }
    };

    // TODO: Implementing execution progress
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
            <label class="total-items-label">"Total items in surface "</label>
            {move || target_dir.get()}
            <h2 class="total-items">{move || total_items.get()}</h2>
            <div class="action-section">
                <button on:click=handle_open_dir>"Choose Directory"</button>
                {execute_button}
            </div>
        </main>
    }
}
