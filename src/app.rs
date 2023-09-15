use leptos::*;
use leptos::leptos_dom::ev;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Reflect, Object};

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

#[derive(serde::Deserialize)]
struct DirItems {
    folders: f64,
    files: f64
}

async fn extract_object_from_dir_items(js_value: JsValue) -> Option<DirItems> {
    let js_object = js_value.dyn_into::<Object>().ok()?;
    
    let files = {
        let files_prop = Reflect::get(&js_object, &"files".into()).ok()?;
        files_prop.as_f64().unwrap_or(0.0) as f64
    };
    
    let folders = {
        let folder_prop = Reflect::get(&js_object, &"folder".into()).ok()?;
        folder_prop.as_f64().unwrap_or(0.0) as f64
    };

    Some(DirItems { folders, files })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (target_dir, set_target_dir) = create_signal(cx, String::new());
    let (total_items, set_total_items) = create_signal(cx, String::new());
    let (executed_time, set_executed_time) = create_signal(cx, String::new());
    let (visible, set_visible) = create_signal(cx, false);
    let (finished_executing, set_finished_executing) = create_signal(cx, false);

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
                        if let Some(value) =  extract_object_from_dir_items(js_value).await {
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
                    set_finished_executing.set(true);
                    set_executed_time.set(result);
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

    let executed_time_label = move || {
        if finished_executing.get() {
            view! { cx, 
                <h3 class="exec-time">
                    { move || executed_time.get() }
                </h3>
            }
        }
        else {
            view! { cx, <h3></h3> }
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
            <div class="finish-section">
                {executed_time_label}
            </div>
        </main>
    }
}
