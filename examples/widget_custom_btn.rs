
fn main() {
    Recipe::new().unwrap().run().unwrap();
}

// Slint 宏构建 UI : 自定义按钮
slint::slint! {
    component MyButton inherits Rectangle {
        in-out property text <=> txt.text;
        callback clicked <=> touch.clicked;
        border-radius: root.height / 2;
        border-width: 1px;
        border-color: root.background.darker(25%);
        background: touch.pressed ? #6b8282 : touch.has-hover ? #6c616c :  #456;
        height: txt.preferred-height * 1.33;
        min-width: txt.preferred-width + 20px;
        txt := Text {
            x: (parent.width - self.width)/2 + (touch.pressed ? 2px : 0);
            y: (parent.height - self.height)/2 + (touch.pressed ? 1px : 0);
            color: touch.pressed ? #fff : #eee;
        }
        touch := TouchArea { }
    }

    export component Recipe inherits Window {
        VerticalLayout {
            alignment: start;
            MyButton { text: "Button"; }
        }
    }
}