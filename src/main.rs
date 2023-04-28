fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text { // Text 文本
            text: "hello world";
            color: green;
        }
    }
}