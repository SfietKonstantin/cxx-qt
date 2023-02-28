// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn bridges() -> [(&'static str, &'static str); 13] {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let (qpointf, qrectf, qsizef) = match target_arch.as_str() {
        "x86" | "arm" => (
            "qpointf/qpointf32.rs",
            "qrectf/qrectf32.rs",
            "qsizef/qsizef32.rs",
        ),
        "x86_64" | "aarch64" => (
            "qpointf/qpointf64.rs",
            "qrectf/qrectf64.rs",
            "qsizef/qsizef64.rs",
        ),
        _ => panic!("Unsupported architecture {}", target_arch),
    };

    [
        ("qcolor.rs", "qcolor.cpp"),
        ("qdate.rs", "qdate.cpp"),
        ("qdatetime.rs", "qdatetime.cpp"),
        ("qpoint.rs", "qpoint.cpp"),
        (qpointf, "qpointf.cpp"),
        ("qrect.rs", "qrect.cpp"),
        (qrectf, "qrectf.cpp"),
        ("qsize.rs", "qsize.cpp"),
        (qsizef, "qsizef.cpp"),
        ("qstring.rs", "qstring.cpp"),
        ("qtime.rs", "qtime.cpp"),
        ("qurl.rs", "qurl.cpp"),
        ("qvariant.rs", "qvariant.cpp"),
    ]
}

fn main() {
    let qt_modules = vec!["Core", "Gui"]
        .iter()
        .map(|m| String::from(*m))
        .collect();
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");
    qtbuild.cargo_link_libraries();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let bridges = bridges();
    for bridge in &bridges {
        println!("cargo:rerun-if-changed=src/types/{}", bridge.0);
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(
        bridges
            .iter()
            .map(|bridge| format!("src/types/{}", bridge.0)),
    );
    for bridge in &bridges {
        builder.file(format!("src/types/{}", bridge.1));
        println!("cargo:rerun-if-changed=src/types/{}", bridge.1);
    }
    builder.file("src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/qt_types.cpp");
    println!("cargo:rerun-if-changed=src/types/assertion_utils.h");

    // Write this library's manually written C++ headers to files and add them to include paths
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(&format!("{}/cxx-qt-lib", out_dir));
    builder.include(out_dir);

    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    builder.compile("cxx-qt-lib");
}
