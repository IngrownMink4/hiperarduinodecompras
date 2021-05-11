#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[no_mangle]
pub static mut motorDer: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub static mut motorIzq: libc::c_int = 6 as libc::c_int;
#[no_mangle]
pub static mut ldrDer: libc::c_int = 0;
#[no_mangle]
pub static mut ldrIzq: libc::c_int = 0;
#[no_mangle]
pub static mut ValLdrDer: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ValLdrIzq: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut sensorpir: libc::c_int = 12 as libc::c_int;
#[no_mangle]
pub static mut ledVerde3: libc::c_int = 8 as libc::c_int;
#[no_mangle]
pub static mut ledVerde4: libc::c_int = 9 as libc::c_int;
#[no_mangle]
pub static mut ledRojo2: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub static mut ledRojo1: libc::c_int = 11 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn setup() { }
