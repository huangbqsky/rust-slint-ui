
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {

     // 容器组件(类似Andorid的自定义组件)
    component BoxWithLabel inherits GridLayout {
        Row {
            Text { text: "label text here"; col: 0; row: 0;}
        }
        Row {
            // @children在组件的元素层次结构中使用表达式来更改默认的子位置：
            @children
        }
    }

    export component MainWindow inherits Window {
        width: 300px;
        height: 200px;
        preferred-height: 100px;
        BoxWithLabel {
            spacing: 0px;
            // Text{ text: "text1"; horizontal-stretch:1;}
            Rectangle { background: blue;  row: 1; col: 0;}
            // Text{ text: "text2"; horizontal-stretch:1;}
            Rectangle { background: yellow; row: 1; col: 1;}
            // Text{ text: "text3"; horizontal-stretch:1;}
        }
    }
}