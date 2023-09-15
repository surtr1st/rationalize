use js_sys::{Reflect, Object};
use wasm_bindgen::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecutionArgs<'e> {
    pub target_dir: &'e str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Directory<'d> {
    pub path: &'d str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenDialogOptions<'odo> {
    pub directory: bool,
    pub title: &'odo str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageDialogOptions<'mdo> {
    pub title: &'mdo str,
}

#[derive(serde::Deserialize)]
pub struct DirItems {
    pub folders: f64,
    pub files: f64,
}

impl DirItems {
    pub async fn extract_object(js_value: JsValue) -> Option<DirItems> {
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
}
