extern crate env_logger;
extern crate glob;
extern crate log;

extern crate protoc;
extern crate protoc_rust;

extern crate protobuf_test_common;

use std::fs;
use std::io::Write;

use protobuf_test_common::build::*;

fn gen_in_dir(dir: &str, include_dir: &str) {
    gen_in_dir_impl(
        dir,
        |GenInDirArgs {
             out_dir,
             input,
             customize,
         }| {
            protoc_rust::Codegen::new()
                .out_dir(out_dir)
                .inputs(input)
                .includes(&["../proto", include_dir])
                .customize(customize)
                .run()
        },
    );
}

fn generate_in_common() {
    let v3 = protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3();

    gen_in_dir("src/common/v2", "src/common/v2");

    if v3 {
        copy_tests_v2_v3("src/common/v2", "src/common/v3");
        gen_in_dir("src/common/v3", "src/common/v3");
    } else {
        let mut mod_rs = fs::File::create("src/common/v3/mod.rs").expect("create");
        writeln!(mod_rs, "// @generated").expect("write");
        writeln!(mod_rs, "// no tests because protoc is not v3").expect("write");
        mod_rs.flush().expect("flush");
    }
}

fn generate_in_v2_v3() {
    gen_in_dir("src/v2", "src/v2");

    assert!(protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3());

    gen_in_dir("src/v3", "src/v3");

    gen_in_dir("src/google/protobuf", "src");
}

fn generate_interop() {
    protoc_rust::Codegen::new()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .input("../interop/cxx/interop_pb.proto")
        .run()
        .unwrap();
}

fn generate_pb_rs() {
    generate_in_common();
    generate_in_v2_v3();
    generate_interop();
}

fn main() {
    env_logger::init();

    cfg_serde();

    clean_old_files();

    generate_pb_rs();

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        println!("cargo:rustc-cfg=proto3");
    }
}
