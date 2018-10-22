#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate parking_lot;
// extern crate jemallocator;

// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use parking_lot::Mutex;
use std::sync::Arc;

#[macro_use]
mod macros;

mod bindings;
use bindings::*;


pub fn new_futhark_context() -> futhark_context {
    use std::ffi::CString;
    unsafe {
        let cfg = futhark_context_config_new();
        futhark_context_config_set_platform(cfg, CString::new("NVIDIA CUDA")
                                            .unwrap().as_ptr());
        futhark_context_config_set_device(cfg, CString::new("Quadro")
                                          .unwrap().as_ptr());
        let tmp = futhark_context_new(cfg);
        // futhark_context_sync(tmp);
        *tmp
    }
}


#[inline(always)]
fn i32_1d_from_vec(mut vec: Vec<i32>) -> *mut futhark_i32_1d {
    let kmut = (*KERNEL).clone();
    unsafe {
        let mut k = kmut.lock();
        let tmp = futhark_new_i32_1d(&mut *k,
                           vec.as_mut_ptr() as *mut i32,
                           vec.len() as i32);
        // futhark_context_sync(&mut *k);
        tmp
    }
}

macro_rules! futhark_vec {
    ($from:ident, $to:ident, $cfun:ident, $name:ident) => {
        fn $name(mut input: Vec<$from>) -> *mut $to {
            let kmut = (*KERNEL).clone();
            let mut k = kmut.lock();
            unsafe {
                $cfun(&mut *k,
                      input.as_mut_ptr() as *mut $from,
                      input.len() as i32)
            }
        }
    }
}

futhark_vec!(i64, futhark_i64_1d, futhark_new_i64_1d, i64_1d_from_vec);

// impl Drop for futhark_i32_1d {
//     fn drop(&mut self) {
//         let kmut = (*KERNEL).clone();
//         let mut k = kmut.lock();
//         unsafe {
//             futhark_free_i32_1d(&mut *k, &mut self)
//         }
//     }
// }

lazy_static! {
    static ref KERNEL: Arc<Mutex<futhark_context>> =
        Arc::new(Mutex::new(new_futhark_context()));
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

futhark_entry!(dotprod64,
               futhark_entry_dotprod64,
               (*const futhark_i64_1d, *const futhark_i64_1d),
               i64);

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
    use std::time::Instant;
    let counter = (1_000..=3_000).step_by(100);
    for i in counter {
        print!("{} ", i);
        let vec1t: Vec<i32> = (1..=i).collect();
        let vec2t: Vec<i32> = (1..=i).collect();
        let vec_now = Instant::now();
        let vec1 = i32_1d_from_vec(vec1t);
        let vec2 = i32_1d_from_vec(vec2t);
        let vec_elapsed = vec_now.elapsed();
        let now = Instant::now();
        let res1 = dotprod2.execute((vec1, vec2));
        let elapsed = now.elapsed();
        let vec_sec =
            (vec_elapsed.as_secs() as f64) + (vec_elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let sec =
            (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("{} {}", vec_sec, sec);
    }
}
