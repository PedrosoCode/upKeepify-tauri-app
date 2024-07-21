use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = invoke)]
    fn invoke_js(command: &str) -> Promise;
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ping {
    pub id: i32,
    pub value: String,
}

pub async fn get_pings() -> Result<Vec<Ping>, JsValue> {
    let promise = invoke_js("get_pings");
    let future = JsFuture::from(promise);
    let result = future.await?;
    let pings: Vec<Ping> = from_value(result).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(pings)
}
