#![allow(non_snake_case)]

use project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn new_proxy(target: JsValue, handler: js_sys::Object) -> js_sys::Proxy {
                js_sys::Proxy::new(&target, &handler)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const target = { a: 100 };
                const handler = {
                     get: function(obj, prop) {
                         return prop in obj ? obj[prop] : 37;
                     }
                };
                const proxy = wasm.new_proxy(target, handler);
                assert.equal(proxy.a, 100);
                assert.equal(proxy.b, 37);
            }
        "#)
        .test()
}

#[test]
fn revocable() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn new_revocable_proxy(target: JsValue, handler: js_sys::Object) -> js_sys::Object {
                js_sys::Proxy::revocable(&target, &handler)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const target = { a: 100 };
                const handler = {
                     get: function(obj, prop) {
                         return prop in obj ? obj[prop] : 37;
                     }
                };
                const { proxy, revoke } =
                   wasm.new_revocable_proxy(target, handler);
                assert.equal(proxy.a, 100);
                assert.equal(proxy.b, 37);
                revoke();
                assert.throws(() => { proxy.a }, TypeError);
                assert.throws(() => { proxy.b }, TypeError);
                assert.equal(typeof proxy, "object");
            }
        "#)
        .test()
}
