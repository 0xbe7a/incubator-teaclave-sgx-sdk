// Copyright (c) 2017 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! # Diffie–Hellman (DH) Session Establishment Functions
//! 
//! These functions allow an ISV to establish secure session between two enclaves using the EC DH Key exchange protocol.
//! 
#![crate_name = "sgx_tdh"]
#![crate_type = "rlib"]

#![cfg_attr(not(feature = "use_std"), no_std)]

#![cfg_attr(not(feature = "use_std"), feature(alloc, collections))]

#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

#[cfg(feature = "use_std")]
#[macro_use]
extern crate std as core;

#[cfg(not(feature = "use_std"))]
extern crate alloc;

#[cfg(not(feature = "use_std"))]
#[macro_use]
extern crate collections;

extern crate sgx_types;
extern crate sgx_trts;
extern crate sgx_tcrypto;
extern crate sgx_tse;

pub mod dh;
pub use self::dh::*;

mod ecp;


