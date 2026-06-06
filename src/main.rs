// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

mod ui;

use tiexiu::Error;
use tiexiu::error::Result;

fn main() -> Result<()> {
    use std::io::{self, Write};
    let mut out_handle = io::stdout().lock();

    match ui::cli::cli(&mut out_handle) {
        Ok(_) => {
            let _ = out_handle.flush();
            Ok(())
        }
        Err(err) => {
            let mut err_handle = io::stderr().lock();
            match &err {
                Error::Io(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
                Error::AndNowAMessageFromYourFriendlyTest(_) => Err(err),
                _ => {
                    #[cfg(debug_assertions)]
                    writeln!(err_handle, "{:#?}", err).ok();
                    #[cfg(not(debug_assertions))]
                    writeln!(err_handle, "{}", err).ok();
                    let _ = err_handle.flush();
                    Err(err)
                }
            }
        }
    }
}
