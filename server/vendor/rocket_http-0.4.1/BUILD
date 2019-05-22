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
    name = "rocket_http",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/cookie-0.11.1:cookie",
        "//server/vendor/hyper-0.10.16:hyper",
        "//server/vendor/indexmap-1.0.2:indexmap",
        "//server/vendor/pear-0.1.2:pear",
        "//server/vendor/percent-encoding-1.0.1:percent_encoding",
        "//server/vendor/smallvec-0.6.9:smallvec",
        "//server/vendor/state-0.4.1:state",
        "//server/vendor/time-0.1.42:time",
        "//server/vendor/unicode-xid-0.1.0:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.1",
    crate_features = [
        "cookie",
        "default",
        "private-cookies",
    ],
)

