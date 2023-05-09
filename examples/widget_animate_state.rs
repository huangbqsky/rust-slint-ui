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
        animate x { duration: 250ms; easing: ease-in; }
        animate y { duration: 250ms; easing: ease-in-out; }
        animate background { duration: 250ms; }
    }

    export component MainWindow inherits Window {
        // 将属性值与状态相关联
        states [
            left-aligned when b1.pressed: {
                circle1.x: 0px; circle1.y: 40px; circle1.background: green;
                circle2.x: 0px; circle2.y: 0px; circle2.background: blue;
            }
            right-aligned when b2.pressed: {
                circle1.x: 170px; circle1.y: 70px; circle1.background: green;
                circle2.x: 170px; circle2.y: 0px; circle2.background: blue;
            }
        ]

        VerticalBox {
            HorizontalBox {
                max-height: self.min-height;
                b1 := Button {
                    text: "State 1";
                }
                b2 := Button {
                    text: "State 2";
                }
            }
            Rectangle {
                background: root.background.darker(20%);
                width: 200px;
                height: 100px;

                circle1 := Circle { background: green; x: 85px; y: 0px; }
                circle2 := Circle { background: green; x: 85px; y: 40px; }
            }
        }
    }
}