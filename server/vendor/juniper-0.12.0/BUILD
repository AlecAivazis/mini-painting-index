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


# Unsupported target "bench" with type "bench" omitted

rust_library(
    name = "juniper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/chrono-0.4.6:chrono",
        "//server/vendor/fnv-1.0.6:fnv",
        "//server/vendor/indexmap-1.0.2:indexmap",
        "//server/vendor/juniper_codegen-0.12.0:juniper_codegen",
        "//server/vendor/serde-1.0.91:serde",
        "//server/vendor/serde_derive-1.0.91:serde_derive",
        "//server/vendor/url-1.7.2:url",
        "//server/vendor/uuid-0.7.4:uuid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.12.0",
    crate_features = [
        "chrono",
        "default",
        "url",
        "uuid",
    ],
)

