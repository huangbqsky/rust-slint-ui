
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
// Slint 语言
slint::slint! {
    export component MainWindow inherits Window {
        width: 300px;
        height: 600px;
        // VerticalLayout 和 HorizontalLayout 布局
        VerticalLayout {
            x:0px;
            y:0px;
            width: 300px;
            height: 200px;
            // Same stretch factor (1 by default): the size is divided equally
            HorizontalLayout {
                Rectangle { background: blue; }
                Rectangle { background: yellow;}
                Rectangle { background: green;}
            }
            // Elements with a bigger min-width are given a bigger size before they expand
            HorizontalLayout {
                Rectangle { background: cyan; min-width: 100px;}
                Rectangle { background: magenta; min-width: 50px;}
                Rectangle { background: gold;}
            }
            // Stretch factor twice as big:  grows twice as much
            HorizontalLayout {
                Rectangle { background: navy; horizontal-stretch: 2;}
                Rectangle { background: gray; }
            }
            // All elements not having a maximum width have a stretch factor of 0 so they grow
            HorizontalLayout {
                Rectangle { background: red; max-width: 20px; }
                Rectangle { background: orange; horizontal-stretch: 0; }
                Rectangle { background: pink; horizontal-stretch: 0; }
            }

            for t in [ "Hello", "World", "!" ] : Text {
                text: t;
            }
        }

        // 网格布局
        GridLayout {
            width: 200px;
            height: 200px;
            x:0px;
            y:300px;
            spacing: 5px;
            Row {
                Rectangle { background: red; }
                Rectangle { background: blue; }
            }
            Row {
                Rectangle { background: yellow; }
                Rectangle { background: green; }
            }
        }
    }
}
