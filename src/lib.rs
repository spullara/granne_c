use std::collections::HashMap;
use std::sync::Mutex;
use std::os::raw::{c_char};
use std::ffi::{CStr};
use ::safer_ffi::prelude::*;
use granne::{angular, BuildConfig, Builder, GranneBuilder, Index, Writeable};
use angular::Vector;
use std::slice;
use granne::angular::Vectors;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ANN_INDEX_MANAGER: Mutex<HashMap<String, Box<GranneBuilder<granne::angular::Vectors<'static>>>>> = Mutex::new(HashMap::new());
}

fn cchar_to_string(name: *const c_char) -> String {
    let idx_name;
    unsafe {
        idx_name = CStr::from_ptr(name).to_string_lossy().into_owned();
    }
    idx_name
}

#[ffi_export]
pub extern fn granne_new_index(
    name: *const c_char,
) {
    let idx_name = cchar_to_string(name);

    ANN_INDEX_MANAGER.lock().unwrap().insert(
        idx_name,
        Box::new(GranneBuilder::new(BuildConfig::default(), granne::angular::Vectors::new())),
    );
}

#[ffi_export]
pub extern fn granne_add(
    name: *const c_char,
    features: *const f32,
    dimension: usize
) -> usize {
    let idx_name: String = cchar_to_string(name);
    let data_slice = unsafe { slice::from_raw_parts(features as *const f32, dimension) };
    let buf = data_slice.to_vec();

    match &mut ANN_INDEX_MANAGER.lock().unwrap().get_mut(&idx_name) {
        Some(index) => {
            index.push(Vector::from(buf));
            index.get_elements().len()
        }
        None => 0
    }
}

#[ffi_export]
pub extern fn granne_build(
    name: *const c_char,
) {
    let idx_name: String = cchar_to_string(name);

    match &mut ANN_INDEX_MANAGER.lock().unwrap().get_mut(&idx_name) {
        Some(index) => {
            index.build();
        }
        None => {
        }
    }
}

#[ffi_export]
pub extern fn granne_search(
    name: *const c_char,
    k: usize,
    features: *const f32,
    dimension: usize
) -> repr_c::Vec<usize> {
    let idx_name: String = cchar_to_string(name);
    let data_slice = unsafe { slice::from_raw_parts(features, dimension) };
    let buf = data_slice.to_vec();
    let topk = k;

    let mut result: Vec<usize> = vec![];
    if let Some(index) = ANN_INDEX_MANAGER.lock().unwrap().get(&idx_name) {
        index.get_index().search(&Vector::from(buf), topk, 100).iter().for_each( |x| {
            result.push(x.0)
        })
    }
    result.into()
}

#[ffi_export]
pub extern fn granne_save(
    name: *const c_char,
    _index_filename: *const c_char,
    _elements_filename: *const c_char,
) {
    let idx_name = cchar_to_string(name);
    let index_filename = cchar_to_string(_index_filename);
    let elements_filename = cchar_to_string(_elements_filename);
    let mut index_file = std::fs::File::create(index_filename).unwrap();
    let mut element_file = std::fs::File::create(elements_filename).unwrap();

    match &mut ANN_INDEX_MANAGER.lock().unwrap().get_mut(&idx_name) {
        Some(index) => {
            index.write_index(&mut index_file).unwrap();
            index.get_elements().write(&mut element_file).unwrap();
        }
        None => {
        }
    }
}

#[ffi_export]
pub extern fn granne_load(
    name: *const c_char,
    _index_filename: *const c_char,
    _elements_filename: *const c_char,
) {
    let idx_name = cchar_to_string(name);
    let index_filename = cchar_to_string(_index_filename);
    let index_file = std::fs::File::open(index_filename).unwrap();
    let element_filename = cchar_to_string(_elements_filename);
    let element_file = std::fs::File::open(element_filename).unwrap();
    let elements = unsafe { Vectors::from_file(&element_file) }.unwrap();

    ANN_INDEX_MANAGER.lock().unwrap().insert(
        idx_name,
        Box::new(GranneBuilder::from_file(BuildConfig::default(), &index_file, elements).unwrap()),
    );

}

#[test]
fn granne_test() {
    ANN_INDEX_MANAGER.lock().unwrap().insert(
        "test".parse().unwrap(),
        Box::new(GranneBuilder::new(BuildConfig::default(), granne::angular::Vectors::new())),
    );
    let buf: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let buf2: Vec<f32> = vec![2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0];
    let buf3: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    match &mut ANN_INDEX_MANAGER.lock().unwrap().get_mut("test") {
        Some(index) => {
            index.push(Vector::from(buf.clone()));
            index.push(Vector::from(buf2.clone()));
            index.push(Vector::from(buf3.clone()));
            println!("Added nodes");
            index.build();
            println!("Built\n");
            let granne = index.get_index();
            let result = granne.search(&Vector::from(buf2), 10, 100);
            for x in result {
                println!("{} {}", x.0, x.1);
            }
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    /// The following test function is necessary for the header generation.
    #[::safer_ffi::cfg_headers]
    #[test]
    fn generate_headers() -> ::std::io::Result<()> {
        ::safer_ffi::headers::builder()
            .to_file("include/granne_c.h")?
            .generate()
    }
}