// use std::env;
// use std::fs::File;
// use std::io::Write;
// use std::path::PathBuf;

// FIXME use the memory.x provided by the hci

fn main() {
//     if let Some((flash, mem)) = memory_sizes() {
//         let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

//         let mut file = File::create(out.join("memory.x")).unwrap();

//         write!(
//             file,
//             r#"MEMORY
// {{
// FLASH : ORIGIN = 0x00000000, LENGTH = {}
// RAM : ORIGIN = 0x20000000, LENGTH = {}
// }}
// "#,
//             flash, mem
//         )
//         .unwrap();

//         println!("cargo:rustc-link-search={}", out.display());
//     }

//     println!("cargo:rerun-if-changed=build.rs");
}
