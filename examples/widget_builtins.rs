
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

// Slint宏 构建 UI
slint::slint! {
    component MyButton inherits Rectangle {
        in-out property <string> text: "Initial";
        // 每个元素都隐式声明了一个init回调
        init => {
            // If `text` is queried here, it will have the value "Hello".
            debug("first");
        }
    }

    component MyCheckBox inherits Rectangle {
        init => { debug("second"); }
    }
    // Initial Example
    export component InitExample inherits Rectangle {
        MyButton {
            text: "Hello world！";
            init => { debug("third"); }
        }
        MyCheckBox {

        }
    }

    import { StandardButton, Button } from "std-widgets.slint";
    export component MyDialog inherits Dialog {
        Text {
           text: "This is a dialog box";
        }
        StandardButton { kind: ok; }
        StandardButton { kind: cancel; }
        Button {
            text: "More Info";
            dialog-button-role: action;
        }
    }

    // Flickable可滚动元素，不会创建滚动条
    export component FlickComponent inherits Rectangle {
        width: 270px;
        height: 100px;

        Flickable {
            viewport-height: 300px; // 可滚动元素的总大小（子元素宽高大于父元素宽高时，该元素变为可滚动）
            Text {
                x:0;
                y: 150px;
                text: "This is some text that you have to scroll to see";
            }
        }
    }

    // 焦点拦截
    export component FocusScopeComponent inherits Rectangle {
        background: green;
        width: 100px;
        height: 100px;
        forward-focus: my-key-handler;
        my-key-handler := FocusScope {
            key-pressed(event) => {
                debug("key-pressed");
                debug(event.text);
                if (event.modifiers.control) {
                    debug("control was pressed during this event");
                }
                if (event.text == Key.Escape) {
                    debug("Esc key was pressed")
                }
                accept
            }
        }
    }
    
    // 示例演示了opacity带有子项的属性。不透明度应用于红色矩形。由于绿色矩形是红色矩形的子矩形，
    // 因此您可以看到它下方的渐变，但您无法透过绿色矩形看到红色矩形
    export component OpacityComponent inherits Rectangle {
        width: 100px;
        height: 100px;
        background: @radial-gradient(circle, black, white, black, white);

        Rectangle {
            opacity: 0.5;
            background: red;
            border-color: #822;
            border-width: 5px;
            width: 50px; height: 50px;
            x: 10px; y: 10px;
            Rectangle {
                background: green;
                border-color: #050;
                border-width: 5px;
                width: 50px; height: 50px;
                x: 25px; y: 25px;
            }
        }
    }

    // Path元素允许呈现由不同几何命令组成的通用形状。可以填充和勾勒路径形状。
    // Path：使用 SVG 命令的路径
    export component SvgPathComponent inherits Path {
        width: 100px;
        height: 100px;
        commands: "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z"; // 使用 SVG 命令的路径
        stroke: red;
        stroke-width: 1px;
    }
    // Path：使用路径命令元素
    export component PathComponent inherits Path {
        width: 100px;
        height: 100px;
        stroke: blue;
        stroke-width: 1px;

        MoveTo {
            x: 0;
            y: 0;
        }
        LineTo {
            x: 0;
            y: 100;
        }
        ArcTo {
            radius-x: 1;
            radius-y: 1;
            x: 100;
            y: 100;
        }
        LineTo {
            x: 100;
            y: 0;
        }
        Close {
        }
    }
    // animation-tick() -> duration 该函数返回一个单调递增的时间，可用于动画
    export component TickComponent inherits Rectangle {
        preferred-width: 100px;
        preferred-height: 100px;

        Rectangle {
            y:0;
            background: red;
            height: 50px;
            width: parent.width * mod(animation-tick(), 2s) / 2s;
        }

        Rectangle {
            background: blue;
            height: 50px;
            y: 50px;
            width: parent.width * abs(sin(360deg * animation-tick() / 3s));
        }
    }

    export component MainWindow inherits Window {
        // 此元素显示弹出窗口，如工具提示或弹出菜单。
        popup := PopupWindow {
            Rectangle { height:100%; width: 100%; background: yellow; }
            x: 20px; y: 20px; height: 50px; width: 50px;
        }
        dialog := MyDialog{
            x: 50px; y: 50px;
        }

        VerticalLayout {
            HorizontalLayout {
                OpacityComponent{}
                FocusScopeComponent{}
            }
            FlickComponent{}
            HorizontalLayout {
                PathComponent{}
                SvgPathComponent{}
            }
            TickComponent{}
        }
        TouchArea {
            height:100%; width: 100%;
            // show()在屏幕上显示弹出窗口。
            clicked => { popup.show(); }
        }
    }
}