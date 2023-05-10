fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
   import { HorizontalBox, VerticalBox, Button } from "std-widgets.slint";

    component Circle inherits Rectangle {
        width: 30px;
        height: 30px;
        border-radius: root.width / 2;
    }

    export component MainWindow inherits Window {
        states [
            // 摁下状态动画
            left-aligned when b1.pressed: {
                circle1.x: 0px; circle1.y: 40px;
                circle2.x: 0px; circle2.y: 0px;
                in {
                    animate circle1.x, circle2.x { duration: 250ms; }
                }
                out {
                    animate circle1.x, circle2.x { duration: 500ms; }
                }
            }
            // 恢复默认状态
            right-aligned when !b1.pressed: {
                circle1.x: 170px; circle1.y: 70px;
                circle2.x: 170px; circle2.y: 00px;
            }
        ]

        VerticalBox {
            HorizontalBox {
                max-height: self.min-height;
                b1 := Button {
                    text: "Press and hold to change state";
                }
            }
            Rectangle {
                background: root.background.darker(20%);
                width: 250px;
                height: 100px;

                circle1 := Circle { background: green; x: 170px; y: 70px;}
                circle2 := Circle { background: blue; x: 170px; y: 0px; }
            }
        }
    } 
}