// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_i64 = crate::Qt5List<i64>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_i64, pos: i32) -> &'a i64;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_i64, _: &i64);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_i64);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_i64, _: &i64) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_i64, _: &i64, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_i64, _: i32, _: &i64);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_i64) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_i64, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_i64"]
        fn construct(_: &Qt5List_i64) -> Qt5List_i64;
        #[rust_name = "qt5list_default_i64"]
        fn construct() -> Qt5List_i64;
        #[rust_name = "qt5list_drop_i64"]
        fn drop(_: &mut Qt5List_i64);
    }
}

pub(crate) fn clone(v: &ffi::Qt5List_i64) -> ffi::Qt5List_i64 {
    ffi::qt5list_clone_i64(v)
}

pub(crate) fn default() -> ffi::Qt5List_i64 {
    ffi::qt5list_default_i64()
}

pub(crate) fn drop(v: &mut ffi::Qt5List_i64) {
    ffi::qt5list_drop_i64(v);
}
