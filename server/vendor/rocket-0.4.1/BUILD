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


# Unsupported target "absolute-uris-okay-issue-443" with type "test" omitted
# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "conditionally-set-server-header-996" with type "test" omitted
# Unsupported target "fairing_before_head_strip-issue-546" with type "test" omitted
# Unsupported target "flash-lazy-removes-issue-466" with type "test" omitted
# Unsupported target "form_method-issue-45" with type "test" omitted
# Unsupported target "form_value_decoding-issue-82" with type "test" omitted
# Unsupported target "format-routing" with type "bench" omitted
# Unsupported target "head_handling" with type "test" omitted
# Unsupported target "limits" with type "test" omitted
# Unsupported target "local-request-content-type-issue-505" with type "test" omitted
# Unsupported target "local_request_private_cookie-issue-368" with type "test" omitted
# Unsupported target "mount_point" with type "test" omitted
# Unsupported target "nested-fairing-attaches" with type "test" omitted
# Unsupported target "precise-content-type-matching" with type "test" omitted
# Unsupported target "ranked-routing" with type "bench" omitted
# Unsupported target "redirect_from_catcher-issue-113" with type "test" omitted
# Unsupported target "responder_lifetime-issue-345" with type "test" omitted

rust_library(
    name = "rocket",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/atty-0.2.11:atty",
        "//server/vendor/base64-0.10.1:base64",
        "//server/vendor/log-0.4.6:log",
        "//server/vendor/memchr-2.2.0:memchr",
        "//server/vendor/num_cpus-1.10.0:num_cpus",
        "//server/vendor/pear-0.1.2:pear",
        "//server/vendor/rocket_codegen-0.4.1:rocket_codegen",
        "//server/vendor/rocket_http-0.4.1:rocket_http",
        "//server/vendor/state-0.4.1:state",
        "//server/vendor/time-0.1.42:time",
        "//server/vendor/toml-0.4.10:toml",
        "//server/vendor/yansi-0.5.0:yansi",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.1",
    crate_features = [
        "default",
        "private-cookies",
        "rocket_http",
    ],
)

# Unsupported target "route_guard" with type "test" omitted
# Unsupported target "segments-issues-41-86" with type "test" omitted
# Unsupported target "simple-routing" with type "bench" omitted
# Unsupported target "strict_and_lenient_forms" with type "test" omitted
# Unsupported target "uri-percent-encoding-issue-808" with type "test" omitted
