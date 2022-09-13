<!--
SPDX-FileCopyrightText: 2021-2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt

CXX-Qt is a set of Rust crates for creating bidirectional Rust ⇄ C++ bindings with [Qt](https://www.qt.io/).
It can be used to integrate Rust into C++ applications using CMake or used to build Rust applications with Cargo.
CXX-Qt provides tools for implementing [QObject](https://doc.qt.io/qt-6/object.html) subclasses in Rust which can
be used from C++, QML, and JavaScript. It consists of two parts:

  * cxx-qt-lib, a library of Rust bindings to common QtCore and QtGui classes made with [CXX](https://cxx.rs/)

  * cxx-qt & cxx-qt-build, a pair of Rust & C++ code generators which are a superset of CXX plus additional attributes
    to interface with Qt's [signals & slots](https://doc.qt.io/qt-6/signalsandslots.html) and [property system](https://doc.qt.io/qt-6/properties.html).
    The cxx-qt crate implements a macro for Rust code generation. cxx-qt-build is used in [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
    to generate and compile the corresponding C++ code.

CXX-Qt is in early development and the API changes frequently.

## Getting Started

If you want to use CXX-Qt and see it in action visit our Getting Started guide in our Rust book [https://kdab.github.io/cxx-qt/book/getting-started/index.html](https://kdab.github.io/cxx-qt/book/getting-started/index.html).

Here we go through the steps of creating a Rust project and exposing to QML.

For more complex examples navigate to the [examples folder](./examples) where there are demonstrations of using threading, QQmlExtensionPlugin, and various other features.

Below is an example of Rust code that exposes a QObject with two properties and four invokable methods to Qt.

```rust
use serde::{Deserialize, Serialize};

// Represent the data within the QObject below with serde friendly types, so we can (de)serialize it
#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&MyObject> for DataSerde {
    fn from(value: &MyObject) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

const DEFAULT_STR: &str = r#"{"number": 1, "string": "Hello World!"}"#;

#[cxx_qt::bridge(namespace = "core")]
mod ffi {
    use super::{DataSerde, DEFAULT_STR};

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        pub number: i32,
        #[qproperty(cxx_type = "QString")]
        pub string: UniquePtr<QString>,
    }

    impl Default for MyObject {
        fn default() -> Self {
            let data_serde: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for MyObject {
        fn from(value: DataSerde) -> MyObject {
            MyObject {
                number: value.number,
                string: QString::from_str(&value.string),
            }
        }
    }

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let new_number = self.get_number() + 1;
            self.set_number(new_number);
        }

        #[qinvokable]
        pub fn reset(mut self: Pin<&mut Self>) {
            let data: DataSerde = serde_json::from_str(DEFAULT_STR).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(QString::from_str(&data.string));
        }

        #[qinvokable(return_cxx_type = "QString")]
        pub fn serialize(&self) -> UniquePtr<QString> {
            let data_serde = DataSerde::from(self.rust());
            let data_string = serde_json::to_string(&data_serde).unwrap();
            QString::from_str(&data_string)
        }

        #[qinvokable]
        pub fn grab_values(mut self: Pin<&mut Self>) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data: DataSerde = serde_json::from_str(string).unwrap();
            self.as_mut().set_number(data.number);
            self.as_mut().set_string(QString::from_str(&data.string));
        }
    }
}
```

## Comparison to other Rust Qt bindings

| Project | Integrate into C++ codebase  | Safe Rust | QML | QWidgets | Maintained<sup>1</sup> | Binding mechanism |
|-------- | ---------------------------- | --------- | --- | -------- | ---------------------- | ----------------- |
| CXX-Qt  |  ✔                           | ✔         | ✔ | limited<sup>2</sup> | ✔       | [cxx](https://cxx.rs) plus additional code generation to implement QObject subclasses in Rust and bind them to C++ |
| [qmetaobject](https://github.com/woboq/qmetaobject-rs/) | ✗ | ✔ | ✔ | ✗ | ✔ | [cpp](https://github.com/mystor/rust-cpp) macro to write C++ inline in Rust, plus Rust macros to create QObject subclasses from Rust structs |
| [Rust Qt Binding Generator](https://invent.kde.org/sdk/rust-qt-binding-generator) | ✔ | ✔ | ✔ | limited<sup>2</sup> | ✗ | generates Rust traits and C++ bindings from JSON description of QObject subclass |
| [rust-qt](https://rust-qt.github.io/) | ✗ | ✗ | ✔ | ✔ | ✗ | [ritual](https://rust-qt.github.io/ritual/) to generate unsafe Rust bindings from C++ headers |
| [qml-rust](https://github.com/White-Oak/qml-rust) | ✗ | ✔ | ✔ | ✗ | ✗ | [DOtherSide](https://github.com/filcuc/DOtherSide) C wrapper for QML C++ classes |
| [qmlrs](https://github.com/flanfly/qmlrs) | ✗ | ✔ | ✔ | ✗ | ✗ | own C++ library to bind QQmlApplicationEngine |
| [qmlrsng](https://github.com/nbigaouette/qmlrsng) | ✗ | ✔ | ✔ | ✗ | ✗ | [libqmlbind](https://github.com/seanchas116/libqmlbind) with [bindgen](https://rust-lang.github.io/rust-bindgen/) |
| [rust-qml](https://github.com/florianjacob/rust-qml) | ✗ | ✔ | ✔ | ✗ | ✗ | [libqmlbind](https://github.com/seanchas116/libqmlbind) |

<sup>1</sup>: maintained: supports Qt6 and repository has had nontrivial commits within last year as of August 2022

<sup>2</sup>: CXX-Qt and Rust Qt Binding Generator can be used to implement custom QObjects subclasses in Rust. C++
bindings for these QObject subclasses can be used in QWidgets applications, but these projects do not provide Rust
bindings for QWidgets APIs.

## Contributing to CXX-Qt

### Building

Ensure that you have the following installed

  * C++ compiler
  * [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
  * [CMake v3.16+](https://cmake.org/)
  * [Qt 5 or Qt 6 (experimental)](https://www.qt.io/)
  * [Rust](https://www.rust-lang.org/)
  * Linux 64-bit x86 - currently we only support Linux, but we plan on adding arm 64-bit, macOS, and Windows support in the future

### Compiling
In a CXX-Qt project, the build system is based on CMake, which uses Cargo under the hood.
Therefore, unlike a typical Rust project, CMake must be used to build CXX-Qt.

On Windows and macOS, CXX-Qt defaults to installing Qt from vcpkg. Prebuilt packages are
automatically downloaded from GitHub Packages (this will take several minutes the first time
you run CMake). If you already have Qt installed, you can disable this by adding
`-D VCPKG=OFF` to the CMake configure step (the first call to `cmake`).

CXX-Qt defaults to building with Qt6. If you want to build with Qt5 when both are installed,
or you want to tell vcpkg to use Qt5, add `-D QT_DEFAULT_MAJOR_VERSION=5` to the CMake
configure step.

```bash
mkdir build/
cd build/
cmake ../
cmake --build . -j$(nproc)
```

### Run the basic QML example

```bash
./build/examples/qml_minimal/example_qml_minimal
```

### Book

You can build the book using `mdbook serve` from the `book` folder, you should install `mdbook` and `mdbook-linkcheck` from cargo.

### Testing
Testing assumes that `cargo clippy` and `cargo fmt` are available, you may need to install these with `rustup component add clippy rustfmt`.

For testing the book, it assumes that `mdbook` and `mdbook-linkcheck` from cargo have been installed.

For license and memory testing, it assumes that you have [`reuse`](https://reuse.software/) installed (eg via `pip3 install reuse`) and [`valgrind`](https://valgrind.org/).

```bash
cd build/
ctest -j$(nproc)
```

## Licensing

CXX-Qt is Copyright (C) 2021, Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/Apache-2.0.txt)
licenses.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

CXX-Qt includes these source files, also available under the terms of the MIT license:

* [doctest.h](https://github.com/onqtam/doctest) - the lightest feature-rich C++ single-header testing framework for unit tests and TDD (C) 2016-2021 Viktor Kirilov <vik.kirilov@gmail.com>

The following CMake source files are available under the BSD-3-Clause

* [cmake/CompilerCaching.cmake](./cmake/CompilerCaching.cmake) - a helper for using sccache

# About KDAB

CXX-Qt is supported and maintained by Klarälvdalens Datakonsult AB (KDAB).

The KDAB Group is the global No.1 software consultancy for Qt, C++ and
OpenGL applications across desktop, embedded and mobile platforms.

The KDAB Group provides consulting and mentoring for developing Qt applications
from scratch and in porting from all popular and legacy frameworks to Qt.
We continue to help develop parts of Qt and are one of the major contributors
to the Qt Project. We can give advanced or standard trainings anywhere
around the globe on Qt as well as C++, OpenGL, 3D and more.

Please visit https://www.kdab.com to meet the people who write code like this.
