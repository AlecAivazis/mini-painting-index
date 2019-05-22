extern crate yansi;
extern crate version_check;

use yansi::Color::{Red, Yellow, Blue, White};
use version_check::{supports_features, is_min_version, is_min_date};

// Specifies the minimum nightly version needed to compile Pear.
const MIN_DATE: &'static str = "2018-10-05";
const MIN_VERSION: &'static str = "1.31.0-nightly";

fn main() {
    let ok_channel = supports_features();
    let ok_version = is_min_version(MIN_VERSION);
    let ok_date = is_min_date(MIN_DATE);
    let triple = (ok_channel, ok_version, ok_date);

    let print_version_err = |version: &str, date: &str| {
        eprintln!("{} {}. {} {}.",
                  White.paint("Installed version is:"),
                  Yellow.paint(format!("{} ({})", version, date)),
                  White.paint("Minimum required:"),
                  Yellow.paint(format!("{} ({})", MIN_VERSION, MIN_DATE)));
    };

    if let (Some(ok_channel), Some((ok_version, version)), Some((ok_date, date))) = triple {
        if !ok_channel {
            eprintln!("{} {}",
                      Red.paint("Error:").bold(),
                      White.paint("Pear requires a nightly or dev version of Rust."));
            print_version_err(&*version, &*date);
            panic!("Aborting compilation due to incompatible compiler.")
        }

        if !ok_version || !ok_date {
            eprintln!("{} {}",
                      Red.paint("Error:").bold(),
                      White.paint("Pear requires a more recent version of rustc."));
            eprintln!("{}{}{}",
                      Blue.paint("Use `"),
                      White.paint("rustup update"),
                      Blue.paint("` or your preferred method to update Rust."));
            print_version_err(&*version, &*date);
            panic!("Aborting compilation due to incompatible compiler.")
        }
    } else {
        println!("cargo:warning={}", "Pear was unable to check rustc compatibility.");
        println!("cargo:warning={}", "Build may fail due to incompatible rustc version.");
    }
}
