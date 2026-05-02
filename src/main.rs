// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod ui;

use tiexiu::Error;
use tiexiu::error::Result;

fn main() -> Result<()> {
    use std::io::{self, Write};
    let mut out_handle = io::stdout().lock();
    let mut err_handle = io::stderr().lock();

    match ui::cli::cli(&mut out_handle) {
        Ok(_) => {
            let _ = out_handle.flush();
            std::process::exit(0);
        }
        Err(err) => match &err {
            Error::Io(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                std::process::exit(0);
            }
            _ => {
                #[cfg(debug_assertions)]
                writeln!(err_handle, "{:#?}", err).ok();
                #[cfg(not(debug_assertions))]
                writeln!(err_handle, "{}", err).ok();
                let _ = err_handle.flush();
                std::process::exit(1);
            }
        },
    }
}
