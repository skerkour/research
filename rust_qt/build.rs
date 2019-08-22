use std::process::Command;
use std::env;

fn qmake_query(qmake_bin: &str, var: &str) -> String {
    String::from_utf8(
        Command::new(qmake_bin)
            .args(&["-query", var])
            .output()
            .expect("Failed to execute qmake. Make sure 'qmake' is in your path")
            .stdout,
    ).expect("UTF-8 conversion failed")
}

fn main() {
    let qmake_bin = match env::var("QMAKE") {
        Ok(val) => val,
        Err(_) => "qmake".to_string(),
    };

    let qt_include_path = qmake_query(&qmake_bin, "QT_INSTALL_HEADERS").trim().to_string();
        cpp_build::Config::new()
        .include(qt_include_path.clone())
        .include(qt_include_path.clone() + "/QtQuick")
        .include(qt_include_path.clone() + "/QtCore")
        .include(qt_include_path.clone() + "/QuickControls2")
        .build("src/main.rs");

    println!("cargo:rustc-link-search=target/cpp");

    let qt_library_path = qmake_query(&qmake_bin, "QT_INSTALL_LIBS").trim().to_string();

    if env::var("QT_STATIC").is_ok() {
        // link Qt statically
        println!("cargo:rustc-link-search={}", qt_library_path);
        println!( // for -lqmacstyle
            "cargo:rustc-link-search={}/styles",
            qmake_query(&qmake_bin, "QT_INSTALL_PLUGINS").trim().to_string()
        );
    } else {
        // link Qt dynamically
        let macos_lib_framework = if cfg!(target_os = "macos") { "" } else { "5" };
        let macos_lib_search = if cfg!(target_os = "macos") { "=framework" } else { "" };

        println!("cargo:rustc-link-search{}={}", macos_lib_search, qt_library_path);
        println!("cargo:rustc-link-lib{}=Qt{}Widgets", macos_lib_search, macos_lib_framework);
        println!("cargo:rustc-link-lib{}=Qt{}Gui", macos_lib_search, macos_lib_framework);
        println!("cargo:rustc-link-lib{}=Qt{}Core", macos_lib_search, macos_lib_framework);
        println!("cargo:rustc-link-lib{}=Qt{}Quick", macos_lib_search, macos_lib_framework);
        println!("cargo:rustc-link-lib{}=Qt{}Qml", macos_lib_search, macos_lib_framework);
        println!("cargo:rustc-link-lib{}=Qt{}QuickControls2", macos_lib_search, macos_lib_framework);
    }
}