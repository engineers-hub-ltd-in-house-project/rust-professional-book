use wasm_bindgen::prelude::*;

// `window.alert`関数をインポート
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// JavaScriptに公開する`greet`関数を定義
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
