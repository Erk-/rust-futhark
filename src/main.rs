#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate parking_lot;

use parking_lot::Mutex;
use std::sync::Arc;

#[macro_use]
mod macros;

mod bindings;
use bindings::*;


impl futhark_context {
    pub fn new() -> Self {
        unsafe {
            let mut cfg = *futhark_context_config_new();
            *futhark_context_new(&mut cfg)
        }
    }
}

#[inline(always)]
fn i32_1d_from_vec(mut vec: Vec<i32>) -> *mut futhark_i32_1d {
    let kmut = (*KERNEL).clone();
    unsafe {
        let mut k = kmut.lock();
        futhark_new_i32_1d(&mut *k,
                           vec.as_mut_ptr() as *mut i32,
                           vec.len() as i32)
    }
}

lazy_static! {
    static ref KERNEL: Arc<Mutex<futhark_context>> =
        Arc::new(Mutex::new(futhark_context::new()));
}

trait FutharkFunction {
    type InType;
    type OutType;

    fn execute(&self, in_var: Self::InType) -> Self::OutType;
}

futhark_entry!(dotprod2,
               futhark_entry_dotprod,
               (*const bindings::futhark_i32_1d, *const bindings::futhark_i32_1d),
               i32);

fn dotprod(vec1: *mut futhark_i32_1d, vec2: *mut futhark_i32_1d) -> i32 {
    let mut res: i32 = 0;
    let kmut = (*KERNEL).clone();
    let mut k = kmut.lock();
    unsafe {
        futhark_entry_dotprod(&mut *k,
                          &mut res,
                          vec1,
                          vec2);
        futhark_context_sync(&mut *k);
    }
    res
}

fn main() {
    let kmut = (*KERNEL).clone();
    use std::i32;
    let vec1t: Vec<i32> = (1..=1000).collect();
    let vec2t: Vec<i32> = (1..=1000).collect();

    let vec3t: Vec<i32> = (1..=1100).collect();
    let vec4t: Vec<i32> = (1..=1100).collect();
    
    
    let (vec1, vec2, vec3, vec4) = {
        let vec1 = i32_1d_from_vec(vec1t);
        let vec2 = i32_1d_from_vec(vec2t);
        let vec3 = i32_1d_from_vec(vec3t);
        let vec4 = i32_1d_from_vec(vec4t);
        (vec1, vec2, vec3, vec4)
    };
    let res1 = dotprod2.execute((vec1, vec2));
    let res2 = dotprod2.execute((vec3, vec4));
    println!("Resultat 1: {}", res1);
    println!("Resultat 2: {}", res2);
}
