use std::cell::RefCell;
use std::rc::Rc;

fn main() {
   // 创建窗体应用程序
   let app =  App::new().unwrap();
   let weak = app.as_weak();// as_weak避免内存泄露
   let state: Rc<RefCell<CalcState>> = Rc::new(RefCell::new(CalcState::default()));
   // 处理.slint中 CalcLogic 全局单例的回调
   app.global::<CalcLogic>().on_button_pressed(move |value| {
      let app = weak.unwrap();
      let mut state = state.borrow_mut();
      println!("input value: {}", value);
      // 只处理输入的数字，保存到 state 中
      if let Ok(val) = value.parse::<i32>() {
        state.current_value *=10;
        state.current_value += val;
        app.set_value(state.current_value);
        return;
      }
      // 处理等号"="逻辑
      if value.as_str() == "=" {
        let reslut = match state.operator.as_str() {
            "+" => state.prev_value + state.current_value,
            "-" => state.prev_value - state.current_value,
            "*" => state.prev_value * state.current_value,
            "/" => state.prev_value / state.current_value,
            _=> state.current_value,
        };
        // 输出结果
        app.set_value(reslut);
        println!("{} {} {} = {}", state.prev_value, state.operator, state.current_value, reslut);
        // 重置 state
        state.current_value = 0;
        state.prev_value = reslut;
        state.operator = Default::default();
      } else {
        state.operator = value.clone();
        state.prev_value = state.current_value;
        state.current_value = 0;
      }
   });
   // 运行窗体程序
   app.run().unwrap();
   println!("Hello, world! Hello, Slint!");

}

// 保存输入数据的结构体
#[derive(Default)]
struct CalcState {
   prev_value: i32,
   current_value: i32,
   operator:slint::SharedString,
}

// Slint 宏构建 UI
slint::slint! {
    import { VerticalBox } from "std-widgets.slint";

    // 导出全局单例：Rust 代码可以操作
    export global CalcLogic {
        callback button-pressed(string);
    }
    // 自定义 Button 组件
    component Button {
        in property <string> text;
        min-height:30px;
        min-width: 30px;
        in property <brush> background: @linear-gradient(-20deg, #a0a3e4, #3c58e3);
        Rectangle {
            background: ta.pressed ? red :ta.has-hover? background.darker(10%) : background;
            animate background {duration: 100ms; }
            border-radius: 4px;
            border-width: 2px;
            border-color: self.background.darker(20%);
            ta := TouchArea {
                // Button初始化
                init => { debug("Button init"); }
                // Button点击事件，回传给 Rust 处理
                clicked => {
                    debug("Button clicked");
                    CalcLogic.button-pressed(root.text)
                }
            }
            Text {text: root.text;}
        }
    }
    export component App inherits Window {
        title: "Slint Calculator";
        in property <int> value: 0 ;
        // Slint 内嵌的网格组件
        GridLayout {
            padding: 10px;
            spacing: 5px;
            Text {text: value; colspan: 3;}
            Row {
                Button {text: "1";}
                Button {text: "2";}
                Button {text: "3";}
                Button {text: "+"; background: yellow;}
            }
            Row {
                Button {text: "4";}
                Button {text: "5";}
                Button {text: "6";}
                Button {text: "-"; background: yellow;}
            }
            Row {
                Button {text: "7";}
                Button {text: "8";}
                Button {text: "9";}
                Button {text: "*"; background: yellow;}
            }
            Row {
                Button {text: "0";}
                Button {text: "="; col: 2; background: green;}
                Button {text: "/"; background: yellow;}
            }
        }
    }
}