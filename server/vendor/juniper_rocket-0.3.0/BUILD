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


# Unsupported target "custom_response_tests" with type "test" omitted

rust_library(
    name = "juniper_rocket",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/juniper-0.12.0:juniper",
        "//server/vendor/rocket-0.4.1:rocket",
        "//server/vendor/serde-1.0.91:serde",
        "//server/vendor/serde_derive-1.0.91:serde_derive",
        "//server/vendor/serde_json-1.0.39:serde_json",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.0",
    crate_features = [
    ],
)

# Unsupported target "rocket_server" with type "example" omitted
