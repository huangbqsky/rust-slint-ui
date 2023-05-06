fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    // Added: tile 数据结构定义并将其粘贴到slint!宏内部的顶部：
    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
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
            background: #193076;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }
    
        TouchArea {
            clicked => {
                // Delegate to the user of this element
                root.clicked();
            }
        }
    }
    
    export component MainWindow inherits Window {
        width: 326px;
        height: 326px;
 
        in property <[TileData]> memory_tiles: [
            { image: @image-url("icons/at.png") },
            { image: @image-url("icons/balance-scale.png") },
            { image: @image-url("icons/bicycle.png") },
            { image: @image-url("icons/bus.png") },
            { image: @image-url("icons/cloud.png") },
            { image: @image-url("icons/cogs.png") },
            { image: @image-url("icons/motorcycle.png") },
            { image: @image-url("icons/video.png") },
        ];
        // 该for tile[i] in memory_tiles:语法声明了一个变量tile，其中包含数组中一个元素的数据memory_tiles，以及一个变量i，该变量是图块的索引。
        // 我们使用i索引根据其行和列计算图块的位置，使用模和整数除法创建 4 x 4 网格。
        // 运行它会为我们提供一个显示 8 个图块的窗口，这些图块可以单独打开。
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px + 20px; // 调整坐标，尽量使整体居中
            y: floor(i / 4) * 74px + 20px; // 调整坐标，尽量使整体居中
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                tile.image_visible = !tile.image_visible;
            }
        }
    }
}