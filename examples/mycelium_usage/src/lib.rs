use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    log::info!("starting...");
    show_metadata();
}

fn show_metadata() {
    spawn_local(async {
        let metadata = mycelium::fetch_metadata().await.expect("must not error");
        log::info!("metadata: {:#?}", metadata);
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let pre = document.create_element("pre").unwrap();
        let text = document.create_text_node(&format!("{:#?}", metadata));
        pre.append_with_node_1(&text).unwrap();
        body.append_with_node_1(&pre).expect("must be appended");
    });
}
