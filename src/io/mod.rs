// Copyright (c) 2016 DWANGO Co., Ltd. All Rights Reserved.
// See the LICENSE file at the top-level directory of this distribution.

//! I/O related functionalities.
pub use self::stdio::{stdin, Stdin};

pub mod poll;
mod stdio;
