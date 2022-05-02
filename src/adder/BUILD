package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "adder",
    srcs = [
        "adder.rs",
        "lib.rs",
    ],
)

rust_test(
    name = "adder_test",
    crate = ":adder",
)