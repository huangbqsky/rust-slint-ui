slint::slint!{
    import { HorizontalBox, VerticalBox, LineEdit, Button } from "std-widgets.slint";
    
    export global Logic {
        pure callback to-upper-case(string) -> string;
        // You can collect other global properties here

        // Do the translation of the first argument, with an array of string as supstitution
       pure callback gettext(string, [string]) -> string;
    }
    
    export component MainWindow inherits Window {
        property <int> count;

        VerticalBox {
            input := LineEdit {
                text: "Text to be transformed";
            }
            HorizontalBox {
                Text { text: "Transformed:"; }
                // Callback invoked in binding expression
                Text {
                    text: {
                        Logic.to-upper-case(input.text);
                    }
                }
            }

            Button {
                text: Logic.gettext("Button pressed {0} times", [count]);
            }
        }
    }
}
    
fn main() {
    let window = MainWindow::new().unwrap();
    
    window.global::<Logic>().on_to_upper_case(|string| {
        string.as_str().to_uppercase().into()
    });

    window.global::<Logic>().on_gettext(|string, model| {
        use slint::Model;
        let mut str = String::from(string.as_str());

        for (idx, to) in model.iter().enumerate() {
            str = str.replace(&format!("%{}", idx + 1), to.as_str());
        }
        str.into()
    });
    window.run().unwrap();
    // ...
}