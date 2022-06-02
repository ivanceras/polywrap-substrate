use mycelium::Api;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    log::info!("starting...");
    show_metadata();
    show_rpc_methods();
    show_blocks();
}

fn show_metadata() {
    spawn_local(async {
        let metadata = Api::new("http://localhost:9933")
            .fetch_metadata()
            .await
            .expect("must not error");
        //log::info!("metadata: {:#?}", metadata);
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let pre = document.create_element("pre").unwrap();
        let text = document.create_text_node(&format!("{:#?}", metadata));
        pre.append_with_node_1(&text).unwrap();
        body.append_with_node_1(&pre).expect("must be appended");
    });
}

fn show_rpc_methods() {
    spawn_local(async {
        let rpc_methods = Api::new("http://localhost:9933")
            .fetch_rpc_methods()
            .await
            .expect("must not error");
        log::info!("rpc_methods: {:#?}", rpc_methods);
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let pre = document.create_element("pre").unwrap();
        let text = document.create_text_node(&format!("{:#?}", rpc_methods));
        pre.append_with_node_1(&text).unwrap();
        body.append_with_node_1(&pre).expect("must be appended");
    });
}

fn show_blocks() {
    spawn_local(async {
        let block: Option<node_template_runtime::Block> = Api::new("http://localhost:9933")
            .fetch_block(0)
            .await
            .expect("must not error");
        log::debug!("block: {:#?}", block);
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let pre = document.create_element("pre").unwrap();
        let text = document.create_text_node(&format!("{:#?}", block));
        pre.append_with_node_1(&text).unwrap();
        body.append_with_node_1(&pre).expect("must be appended");
    });
}
