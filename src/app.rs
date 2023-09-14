//use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use leptos::leptos_dom::ev;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(args: JsValue) -> JsValue;
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ExecutionArgs<'e> {
    target_dir: &'e str,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Directory<'d> {
    path: &'d str,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenDialogOptions<'odo> {
    directory: bool,
    title: &'odo str
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (target_dir, set_target_dir) = create_signal(cx, String::new());
    let (total_files, set_total_files) = create_signal(cx, 0f64);
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
                        if let Some(total) = invoke("retrieve_total_files", arg).await.as_f64() {
                            set_total_files.set(total);
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
            let args = to_value(&ExecutionArgs {
                target_dir: &target_dir.get(),
            }).unwrap();
            invoke("exec", args).await;
        });
    };
 

    let execute_button = move || {
        if visible.get() {
            view! { cx, <button class="exec-button" on:click=handle_execution>"Execute"</button> }
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
            <label class="total-file-label">"Total files in "</label>
            {move || target_dir.get()}
            <h2 class="total-file">{move || total_files.get()}</h2>
            <div class="action-section">
                <button on:click=handle_open_dir>"Choose Directory"</button>
                {execute_button}
            </div>
        </main>
    }
}
