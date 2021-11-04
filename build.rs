extern crate cc;

fn main() {
    cc::Build::new()
    .file("src/add.c")
    .compile("libadd.a");

    cc::Build::new()
    .file("src/callback.c")
    .compile("libcallback.a");
}