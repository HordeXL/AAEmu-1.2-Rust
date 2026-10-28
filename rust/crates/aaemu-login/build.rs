//! Compiles `resources/icon_login.ico` into the executable as its Windows icon.
//!
//! The C# project set `<ApplicationIcon>`; the Rust port needs the resource compiled into the
//! binary, otherwise Explorer and the console host show the generic executable icon.
fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    println!("cargo:rerun-if-changed=resources/icon_login.ico");
    let mut res = winresource::WindowsResource::new();
    res.set_icon("resources/icon_login.ico");
    res.compile()
        .expect("compiling the Windows icon resource (rc.exe from the Windows SDK is required)");
}
