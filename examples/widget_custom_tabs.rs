
fn main() {
    TabsWindow::new().unwrap().run().unwrap();
}

// Slint 语言
slint::slint! {
    import { Button } from "std-widgets.slint";
    // 创建自己的自定义选项卡小部件Tabs
    export component TabsWindow inherits Window {
        preferred-width: 800px;
        preferred-height: 600px;
        in-out property <int> active-tab;
        VerticalLayout {
            tab_bar := HorizontalLayout {
                spacing: 3px;
                Button {
                    text: "Red";
                    clicked => { root.active-tab = 0; }
                }
                Button {
                    text: "Blue";
                    clicked => { root.active-tab = 1; }
                }
                Button {
                    text: "Green";
                    clicked => { root.active-tab = 2; }
                }
            }
            Rectangle {
                clip: true;
                Rectangle {
                    background: red;
                    x: root.active-tab == 0 ? 0 : root.active-tab < 0 ? - self.width - 1px : parent.width + 1px;
                    animate x { duration: 125ms; easing: ease; }
                }
                Rectangle {
                    background: blue;
                    x: root.active-tab == 1 ? 0 : root.active-tab < 1 ? - self.width - 1px : parent.width + 1px;
                    animate x { duration: 125ms; easing: ease; }
                }
                Rectangle {
                    background: green;
                    x: root.active-tab == 2 ? 0 : root.active-tab < 2 ? - self.width - 1px : parent.width + 1px;
                    animate x { duration: 125ms; easing: ease; }
                }
            }
        }
    }

}