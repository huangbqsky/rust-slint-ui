fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 宏构建 UI
slint::slint! {
    import { AboutSlint } from "std-widgets.slint";
    export component MainWindow inherits Window {
 
        AboutSlint {}
    }
}