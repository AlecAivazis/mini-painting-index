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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "hyper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/base64-0.9.3:base64",
        "//server/vendor/httparse-1.3.3:httparse",
        "//server/vendor/language-tags-0.2.2:language_tags",
        "//server/vendor/log-0.3.9:log",
        "//server/vendor/mime-0.2.6:mime",
        "//server/vendor/num_cpus-1.10.0:num_cpus",
        "//server/vendor/time-0.1.42:time",
        "//server/vendor/traitobject-0.1.0:traitobject",
        "//server/vendor/typeable-0.1.2:typeable",
        "//server/vendor/unicase-1.4.2:unicase",
        "//server/vendor/url-1.7.2:url",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.10.16",
    crate_features = [
    ],
)

