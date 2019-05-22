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


# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "compile-test" with type "test" omitted
# Unsupported target "expansion" with type "test" omitted
# Unsupported target "from_form" with type "test" omitted
# Unsupported target "from_form_value" with type "test" omitted
# Unsupported target "responder" with type "test" omitted

rust_library(
    name = "rocket_codegen",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/devise-0.2.0:devise",
        "//server/vendor/indexmap-1.0.2:indexmap",
        "//server/vendor/quote-0.6.12:quote",
        "//server/vendor/rocket_http-0.4.1:rocket_http",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.1",
    crate_features = [
    ],
)

# Unsupported target "route" with type "test" omitted
# Unsupported target "route-data" with type "test" omitted
# Unsupported target "route-format" with type "test" omitted
# Unsupported target "route-params" with type "test" omitted
# Unsupported target "route-ranking" with type "test" omitted
# Unsupported target "typed-uris" with type "test" omitted
# Unsupported target "uri_display" with type "test" omitted
