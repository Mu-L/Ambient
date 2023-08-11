use ambient_api::{
    core::wasm::components::{bytecode_from_url, module_enabled},
    prelude::*,
};

use crate::embers::ember_manager::messages::{WasmReload, WasmSetEnabled};

pub fn main() {
    WasmSetEnabled::subscribe(|_, msg| {
        entity::set_component(msg.id, module_enabled(), msg.enabled);
    });

    WasmReload::subscribe(|source, msg| {
        let Some(_user_id) = source.client_user_id() else { return; };
        let id = msg.id;

        run_async(async move {
            if let Some(url) = entity::get_component(id, bytecode_from_url()) {
                entity::set_component(id, bytecode_from_url(), url);
            }
        });
    });
}
