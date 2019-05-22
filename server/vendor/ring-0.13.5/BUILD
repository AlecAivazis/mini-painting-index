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
  "restricted", # "no license"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "aead_tests" with type "test" omitted
# Unsupported target "agreement_tests" with type "test" omitted
# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "digest_tests" with type "test" omitted
# Unsupported target "ecdsa_tests" with type "test" omitted
# Unsupported target "ed25519_tests" with type "test" omitted
# Unsupported target "hkdf_tests" with type "test" omitted
# Unsupported target "hmac_tests" with type "test" omitted
# Unsupported target "pbkdf2_tests" with type "test" omitted

rust_library(
    name = "ring",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//server/vendor/lazy_static-1.3.0:lazy_static",
        "//server/vendor/libc-0.2.55:libc",
        "//server/vendor/untrusted-0.6.2:untrusted",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.13.5",
    crate_features = [
        "default",
        "dev_urandom_fallback",
        "use_heap",
    ],
)

# Unsupported target "rsa_tests" with type "test" omitted
# Unsupported target "signature_tests" with type "test" omitted
