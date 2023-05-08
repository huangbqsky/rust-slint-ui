
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
    import { VerticalBox, Button} from "std-widgets.slint";

    export component MainWindow inherits Window {
        width: 300px;
        height: 200px;
        preferred-height: 100px;
        in-out property <int> counter: 0;
        callback button-pressed <=> button.clicked;
        VerticalBox {
            width: 300px;
            height: 50px;
            alignment:start;
            button := Button { // 一个可点击的按钮
                text: "Button, pressed " + root.counter + " times";
                clicked => {
                    root.counter += 1;
                }
            }
        } 
    }
}