fn main() {
    slint_build::compile("src/style.slint").unwrap();
    slint_build::compile("src/bin/c3_superclics/style.slint").unwrap();
}