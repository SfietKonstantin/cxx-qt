// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(any(qt_version_major = "5", qt_version_major = "6")))]
compile_error!("qt_version_major must be either \"5\" or \"6\"");
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
)))]
compile_error!("Supported archs are x86, x86_64, arm or aarch64");

mod types;
pub use types::*;
