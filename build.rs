fn main() {
    slint_build::compile("ui/slide_puzzle.slint").unwrap();
    slint_build::compile("ui/slider.slint").unwrap();
    slint_build::compile("ui/main_simple.slint").unwrap();
    slint_build::compile("ui/main.slint").unwrap();
   
}
