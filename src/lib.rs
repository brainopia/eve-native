#![feature(link_args)]
#![feature(dropck_eyepatch)]
#![feature(generic_param_attrs)]
#![feature(sip_hash_13)]
#![feature(core_intrinsics)]
#![feature(shared)]
#![feature(unique)]
#![feature(placement_new_protocol)]
#![feature(fused)]
#![feature(alloc)]
#![feature(heap_api)]
#![feature(oom)]
#![feature(slice_patterns)]

// #[link_args = "-s EXPORTED_FUNCTIONS=['_coolrand','_makeIter','_next']"]
extern {}

#[macro_use]
extern crate nom;

pub mod ops;
pub mod indexes;
pub mod hash;
pub mod parser;