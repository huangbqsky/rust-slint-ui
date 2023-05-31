use std::rc::Rc;

use slint::{StandardListViewItem, VecModel};

slint::slint!{
    import { GroupBox, HorizontalBox, VerticalBox, StandardListView, LineEdit, Button } from "std-widgets.slint";
    
    // 导出全局单单例
    export global Logic {
        // You can collect other global properties here
        in-out property <int> counter: 0;
        // 全局单例中的回调（带参数和返回值）
        pure callback to-upper-case(string) -> string;
    }

    // 对外导出全局单例（可与 Rust 代码直接交互）
    export global ListViewPageAdapter  {
        // StandardListView model (in StandardListViewItem): The model
        in property <[StandardListViewItem]> list_data: [
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
        ];
    }
    
    export component MainWindow inherits Window {
        preferred-width: 500px;
        preferred-height: 300px;

        in-out property <int> counter: 0; //in-out类型的属性
        callback button-pressed <=> button.clicked; // 双向绑定属性的回调

        VerticalBox {
            alignment: start;
            GroupBox {
                title: "Rust Global Callbacks";
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
                title: "Rust Modify Propertys";
                button := Button {
                    height:30px;
                    // 属性绑定
                    text: "Button, pressed " + root.counter + " times";
                }
            }

            GroupBox {  
                title: "Rust Modify Models";
                vertical-stretch: 0;
                height: 300px;
                StandardListView {
                    // model方案1: 直接写死数组
                    // model: [
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    //     {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                    // ];
                    // model方案2: 可与 Rust 代码动态交互的数据
                    model: ListViewPageAdapter.list-data;
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

    // 获取 StandardListView Model数据，这里用到 VecModel，才能修改值
    let tiles_model: Rc<VecModel<StandardListViewItem>> = Rc::new(VecModel::default());
    // 重新组织填充数据： 100列的数据
    for r in 1..=100 { // 100行
        tiles_model.push(slint::format!("Item {r}").into());
    }
    // 通过全局单例重新设置list_data的值， set_XXXX()是重新设置Model数据
    window.global::<ListViewPageAdapter>().set_list_data(tiles_model.clone().into());

    // 运行窗体
    window.run().unwrap();
}