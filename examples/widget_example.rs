fn main() {
    // Example::new().unwrap().run().unwrap();
    let app = Example::new().unwrap();
    app.global::<Logic>().on_magic_operation(|value| {
        eprintln!("magic operation input: {}", value);
        value * 2 // 返回值为输入值的2倍
    });
    app.global::<Logic>().set_the_value(42); // 设置值

    // 运行窗体
    app.run().unwrap();
}

// slint宏，构建 UI
slint::slint! {

    // 全局单例
    // 声明一个全局单例，使属性和回调在整个项目中可用。使用访问它们。
    // global Name { /* .. properties or callbacks .. */ }Name.property
    export global Logic  {
        in-out property <int> the-value;
        pure callback magic-operation(int) -> int;
    }
    component SomeComponent inherits Text {
        // use the global in any component
        text: "The magic value is:" + Logic.magic-operation(42);
    }

    export component Example inherits Window {
        preferred-width: 600px;
        preferred-height: 300px;

        in property <[{foo: string, col: color}]> model: [
            {foo: "abc", col: #e11 },
            {foo: "def", col: #1a2 },
            {foo: "xyz", col: #23d },
        ];

        VerticalLayout {
            // 使用for-in语法多次创建一个元素： for name[index] in model : id := Element { ... }
            // 在元素中进行查找，索引是可选的，将被设置为该元素在模型中的索引。id也是可选的。
            HorizontalLayout {
                // 整数类型模型：元素将重复
                for data in root.model: my-repeated-text := Text {
                    color: data.col;
                    text: data.foo;
                }
            }

            HorizontalLayout {
                 // 组类类型模型：将为数组或模型中的每个元素实例化元素
                for my-color[index] in [ #e11, #1a2, #23d ]: Rectangle {
                    height: 100px;
                    width: 60px;
                    background: my-color;
                }
            }

            Rectangle {
                // 条件：仅当给定条件为真时，构造if才会实例化元素。语法是if condition : id := Element { ... }
                if area.pressed : foo := Rectangle { background: blue;}
                if !area.pressed : Rectangle { background: red;}
                area := TouchArea {}
            }

            Rectangle {
                background: area1.pressed ? red : blue;
                // 动画: 使用animate这样的关键字声明属性的动画
                animate background {
                    duration: 250ms;
                }
                area1 := TouchArea {}
            }
          
            st :=Rectangle {
                preferred-width: 600px;
                preferred-height: 100px;
                property <bool> active: true;
                // states语句允许一次性声明多个元素的状态和设置属性：
                states [
                    active when active && !ta.has-hover: {
                        label.text: "Active";
                        st.background: blue;
                    }
                    active-hover when active && ta.has-hover: {
                        label.text: "Active\nHover";
                        st.background: green;
                    }
                    inactive when !active: {
                        label.text: "Inactive";
                        st.background: gray;
                    }
                ]
                label := Text { }
                ta := TouchArea {
                    clicked => {
                        active = !active;
                    }
                }
            }
            // 动画过渡
            st1 :=Rectangle {
                preferred-width: 600px;
                preferred-height: 100px;

                text := Text { text: "hello"; }
                ta1 := TouchArea {
                    clicked => {
                        pressed = !pressed;
                        is-enabled = !is-enabled;
                    }
                }

                in-out property<bool> pressed;
                in-out property<bool> is-enabled;
                // 过渡将动画绑定到状态变化。
                // 这个例子定义了两个转换。首先out关键字用于在离开disabled状态时为所有属性设置 800 毫秒的动画。
                // 第二次转换使用in关键字在转换到状态时为背景设置动画down。
                states [
                    disabled when !st1.is-enabled : {
                        background: gray; // same as root.background: gray;
                        text.color: white;
                        out {
                            animate * { duration: 800ms; }
                        }
                    }
                    down when pressed : {
                        background: blue;
                        in {
                            animate background { duration: 300ms; }
                        }
                    }
                ]
            }

            // 全局单例
            Rectangle {
                preferred-width: 600px;
                preferred-height: 100px;
                // re-expose the global properties such that the native code
                // can access or modify them
                in-out property new_the-value <=> Logic.the-value;
                pure callback new_magic-operation <=> Logic.magic-operation;

                SomeComponent {
                    // 覆盖了组件定义中默认text的值
                    text: "The magic value is:" + new_magic-operation(10);
                }
            }
        }
    }
}
