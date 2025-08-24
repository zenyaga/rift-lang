use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-python", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-python");
    println!("cargo:rerun-if-changed=tree-sitter-python/src/parser.c");

    let dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-javascript");
    println!("cargo:rerun-if-changed=tree-sitter-javascript/src/parser.c");

    let dir: PathBuf = ["tree-sitter-go", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-go");
    println!("cargo:rerun-if-changed=tree-sitter-go/src/parser.c");

    let dir: PathBuf = ["tree-sitter-cpp", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-cpp");
    println!("cargo:rerun-if-changed=tree-sitter-cpp/src/parser.c");

    let dir: PathBuf = ["tree-sitter-java", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-java");
    println!("cargo:rerun-if-changed=tree-sitter-java/src/parser.c");

    let dir: PathBuf = ["tree-sitter-php", "src"].iter().collect();
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-php");
    println!("cargo:rerun-if-changed=tree-sitter-php/src/parser.c");
}