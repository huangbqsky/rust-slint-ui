slint::slint!{
    import { GroupBox, HorizontalBox, VerticalBox, LineEdit, Button } from "std-widgets.slint";
    
    // 导出全局单单例
    export global Logic {
        // You can collect other global properties here
        in-out property <int> counter: 0;
        // 全局单例中的回调（带参数和返回值）
        pure callback to-upper-case(string) -> string;
        // Do the translation of the first argument, with an array of string as supstitution
        pure callback gettext(string, [string]) -> string;
    }
    
    export component MainWindow inherits Window {
        preferred-width: 500px;
        preferred-height: 300px;

        in-out property <int> counter: 0; //in-out类型的属性
        callback button-pressed <=> button.clicked; // 双向绑定属性的回调

        VerticalBox {
            alignment: start;
            GroupBox {
                title: "Rust Modify global Logic";
                VerticalBox {
                    input := LineEdit {
                        text: "Text to be transformed";
                    }
                    HorizontalBox {
                        alignment: start;
                        Text { text: "Transformed:"; }
                        // Callback invoked in binding expression
                        Text {
                            // 全局回调
                            text: { Logic.to-upper-case(input.text);}
                        }
                    } 
                }
            }

            GroupBox {
                title: "Rust Modify Property";
                button := Button {
                    height:30px;
                    // 属性绑定
                    text: "Button, pressed " + root.counter + " times";
                }
            }
        }
    }
}
    
fn main() {
    let window = MainWindow::new().unwrap();
    // 全局单例中的回调（带参数和返回值）
    window.global::<Logic>().on_to_upper_case(|string| {
        string.as_str().to_uppercase().into()
    });

    let main_window_weak = window.as_weak();
    // on_button_pressed() is a callback that is bound to the Button's text
    window.on_button_pressed(move || {
        let main_window = main_window_weak.unwrap();
        // get_xxx() 函数获取属性
        let mut value = main_window.get_counter();
        value = value + 1;
        // set_xxx() 函数重新设置属性
        main_window.set_counter(value);
    });

    window.run().unwrap();
    // ...
}