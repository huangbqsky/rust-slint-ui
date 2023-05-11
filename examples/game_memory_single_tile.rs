fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
 
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

    component MemoryTile inherits Rectangle {
        callback clicked;
        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;
    
        height: 64px;
        width: 64px;
        background: solved ? #34CE57 : #3960D5;
        animate background { duration: 800ms; }
    
        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }
    
        // Left curtain
        Rectangle {
            background: #193076;
            x: 0px;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
        }
    
        // Right curtain
        Rectangle {
            background: #193020;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }

        Text {
            text: "hello left & right curtain";
            color: green;
        }
    
        TouchArea {
            clicked => {
                // Delegate to the user of this element
                root.clicked();
            }
        }
    }
    
    export component MainWindow inherits Window {
        MemoryTile {
            background: #3911D5;
            icon: @image-url("icons/bus.png");
            clicked => { // 点击处理
                self.open_curtain = !self.open_curtain;
            }
        }
    }
}