
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
    import { Button } from "std-widgets.slint";

    export component MainWindow inherits Window {
        width: 300px;
        height: 200px;
        preferred-height: 100px;
        VerticalLayout {
            alignment: start;
            Button {
                text: "press me";
                // 通过调用手动激活元素上的焦点focus()：
                clicked => { input.focus(); }
            }
            // 某些元素例如TextInput不仅接受来自鼠标/手指的输入，还接受来自（虚拟）键盘的键事件。
            // 为了让项目接收这些事件，它必须有焦点。这可以通过has-focus(out) 属性看到。
            input := TextInput {
                text: "I am a text input field";
            }
        }
    }
}