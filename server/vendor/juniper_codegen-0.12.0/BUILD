"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//server", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "restricted", # "BSD-2-Clause"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "juniper_codegen",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/lazy_static-1.3.0:lazy_static",
        "//server/vendor/proc-macro2-0.4.30:proc_macro2",
        "//server/vendor/quote-0.6.12:quote",
        "//server/vendor/regex-1.1.6:regex",
        "//server/vendor/syn-0.15.34:syn",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.12.0",
    crate_features = [
    ],
)

