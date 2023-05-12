
fn main() {
    Example::new().unwrap().run().unwrap();
}

// slint宏，构建 UI
slint::slint!{
  
    // 使用for-in语法多次创建一个元素： for name[index] in model : id := Element { ... }
    // 在元素中进行查找，索引是可选的，将被设置为该元素在模型中的索引。id也是可选的。
    
    export component Example inherits Window {
        preferred-width: 600px;
        preferred-height: 300px;

        in property <[{foo: string, col: color}]> model: [
            {foo: "abc", col: #e11 },
            {foo: "def", col: #1a2 },
            {foo: "xyz", col: #23d },
        ];

        VerticalLayout {
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
        } 
    }
}