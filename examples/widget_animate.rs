fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
    import { CheckBox } from "std-widgets.slint";
    export component MainWindow inherits Window {
        width: 200px;
        height: 100px;

        rect := Rectangle {
            x:0;
            y: 5px;
            width: 40px;
            height: 40px;
            background: blue;
            // 只要属性发生变化，它就会运行：要么是因为回调设置了属性，要么是因为它的绑定值发生了变化。animate x
            animate x {
                duration: 500ms;
                easing: ease-in-out;
            }
            animate y {
                duration: 250ms;
                delay: 500ms;
                easing: ease-in;
            }
        }


        CheckBox {
            y: 25px;
            text: "Align rect bottom right";
            toggled => {
                if (self.checked) {
                    rect.x = parent.width - rect.width;
                    rect.y = parent.height - rect.height;
                } else {
                    rect.x = 0px;
                    rect.y = 0px;
                }
            }
        }
    }
}
