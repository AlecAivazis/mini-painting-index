//! This tiny build script ensures that rocket is not compiled with an
//! incompatible version of rust.

extern crate yansi;
extern crate version_check;

use yansi::Color::{Red, Yellow, Blue};
use version_check::{supports_features, is_min_version, is_min_date};

// Specifies the minimum nightly version needed to compile Rocket.
const MIN_DATE: &'static str = "2018-10-05";
const MIN_VERSION: &'static str = "1.31.0-nightly";

fn main() {
    let ok_channel = supports_features();
    let ok_version = is_min_version(MIN_VERSION);
    let ok_date = is_min_date(MIN_DATE);
    let triple = (ok_channel, ok_version, ok_date);

    let print_version_err = |version: &str, date: &str| {
        eprintln!("{} {}. {} {}.",
                  "Installed version is:",
                  Yellow.paint(format!("{} ({})", version, date)),
                  "Minimum required:",
                  Yellow.paint(format!("{} ({})", MIN_VERSION, MIN_DATE)));
    };

    if let (Some(ok_channel), Some((ok_version, version)), Some((ok_date, date))) = triple {
        if !ok_channel {
            eprintln!("{} {}",
                      Red.paint("Error:").bold(),
                      "Rocket requires a nightly or dev version of Rust.");
            print_version_err(&*version, &*date);
            eprintln!("{}{}{}",
                      Blue.paint("See the getting started guide ("),
                      "https://rocket.rs/v0.4/guide/getting-started/",
                      Blue.paint(") for more information."));
            panic!("Aborting compilation due to incompatible compiler.")
        }

        if !ok_version || !ok_date {
            eprintln!("{} {}",
                      Red.paint("Error:").bold(),
                      "Rocket requires a more recent version of rustc.");
            eprintln!("{}{}{}",
                      Blue.paint("Use `"), "rustup update",
                      Blue.paint("` or your preferred method to update Rust."));
            print_version_err(&*version, &*date);
            panic!("Aborting compilation due to incompatible compiler.")
        }
    } else {
        println!("cargo:warning={}", "Rocket was unable to check rustc compatibility.");
        println!("cargo:warning={}", "Build may fail due to incompatible rustc version.");
    }
}
