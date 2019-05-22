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
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "cookie",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/base64-0.9.3:base64",
        "//server/vendor/ring-0.13.5:ring",
        "//server/vendor/time-0.1.42:time",
        "//server/vendor/url-1.7.2:url",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.11.1",
    crate_features = [
        "base64",
        "percent-encode",
        "ring",
        "secure",
        "url",
    ],
)

