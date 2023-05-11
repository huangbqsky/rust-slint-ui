
fn main() {
    SideBarWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
    import { Button, StyleMetrics } from "std-widgets.slint";

    // 使用配方实现响应式侧边栏，当父宽度小于给定的断点时折叠。单击按钮时，侧边栏再次展开。
    export component SideBar inherits Rectangle {
        private property <bool> collapsed: root.reference-width < root.break-point;

        // Defines the reference width to check `break-point`.
        in-out property <length> reference-width;

        // If `reference-width` is less `break-point` the `SideBar` collapses.
        in-out property <length> break-point: 600px;

        // Set the text of the expand button.
        in-out property <string> expand-button-text;

        width: 160px;

        container := Rectangle {
            private property <bool> expaned;

            width: parent.width;
            background: StyleMetrics.window-background.darker(0.2);

            VerticalLayout {
                padding: 2px;
                alignment: start;

                HorizontalLayout {
                    alignment: start;

                    if (root.collapsed) : Button {
                        checked: container.expaned;
                        text: root.expand-button-text;

                        clicked => {
                            container.expaned = !container.expaned;
                        }
                    }
                }

                @children
            }

            states [
                expaned when container.expaned && root.collapsed : {
                    width: 160px;

                    in {
                        animate width { duration: 200ms; }
                    }

                    out {
                        animate width { duration: 200ms; }
                    }
                in {
                        animate width { duration: 200ms; }
                    }
                out {
                        animate width { duration: 200ms; }
                    }
                }
            ]
        }

        states [
            collapsed when root.collapsed : {
                width: 62px;
            }
        ]
    }

    export component SideBarWindow inherits Window {
        preferred-width: 700px;
        min-height: 400px;

        GridLayout {
            Rectangle {
                height: 100%;
                col: 1;
                background: white;

                HorizontalLayout {
                    padding: 8px;

                    Text {
                        color: black;
                        text: "Content";
                    }
                }
            }
            SideBar {
                col: 0;
                reference-width: parent.width;
                expand-button-text: "E";
            }
        }
    }

}