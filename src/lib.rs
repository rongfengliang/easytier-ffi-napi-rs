#![deny(clippy::all)]
use napi::bindgen_prelude::*;
mod easytier;
mod bindings;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn parse_config(conf: String) -> i32 {
 let result =  match easytier::parse_config(&conf) {
    Ok(val) => val,
    Err(_) => -1,
  };
  result
}

#[napi]
pub fn run_network_service(conf: String) -> i32 {
  match easytier::run_network_service(&conf) {
    Ok(_) => 0,
    Err(_) => -1,
  }
}


#[napi(object)]
pub struct NetworkInfo {
    pub key: String,
    pub value: String,
}

#[napi]
pub fn collect_network_infos() -> Vec<NetworkInfo> {
  let result = easytier::collect_network_infos()
    .unwrap_or_else(|_| vec![]);
  result.into_iter().map(|info| {
    NetworkInfo {
      key:  unsafe {
        std::ffi::CStr::from_ptr(info.key).to_string_lossy().into_owned()
      },
      value: unsafe {
        std::ffi::CStr::from_ptr(info.value).to_string_lossy().into_owned()
      },
    }
  }).collect()
}