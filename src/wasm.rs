#[cfg(target_arch = "wasm32")]
use crate::Millis;
#[cfg(target_arch = "wasm32")]
use crate::MonotonicClock;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub struct WasmMonotonicClock {
    started: f64,
}

#[cfg(target_arch = "wasm32")]
impl WasmMonotonicClock {
    pub fn new() -> Self {
        let window = web_sys::window().expect("should have a Window");
        let performance = window.performance().expect("should have a Performance");
        let now = performance.now();
        Self { started: now }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for WasmMonotonicClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
impl MonotonicClock for WasmMonotonicClock {
    fn now(&self) -> Millis {
        let window = web_sys::window().expect("should have a Window");
        let performance = window.performance().expect("should have a Performance");
        let current = performance.now();
        let elapsed = current - self.started;
        Millis::new(elapsed as u64)
    }
}
