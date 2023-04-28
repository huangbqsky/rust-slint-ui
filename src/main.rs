fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    // export component MainWindow inherits Window {
    //     Text {
    //         text: "hello world";
    //         color: green;
    //     }
    // }

    component MemoryTile inherits Rectangle {
        width: 64px;
        height: 64px;
        background: #3960D5;
    
        Image {
            source: @image-url("icons/bus.png");
            width: parent.width;
            height: parent.height;
        }
        Text {
            text: "hello world";
            color: red;
        }
    }
    
    export component MainWindow inherits Window {
        MemoryTile {}
    }
}