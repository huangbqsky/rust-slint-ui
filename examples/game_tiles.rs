fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model(从 Model中获取图块列表)
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // 复制一份图块数据，确保整体上是成对的，因为游戏就要上翻牌来配对图块
    tiles.extend(tiles.clone());

    // 随机混合一下，就是打乱所有图标
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // 将打乱后的 Vec 分配给模型属性
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // 游戏规则：游戏规则应强制要求最多两块牌的图块打开，
    // 如果图块匹配，那么我们认为它们已解决并且它们保持打开状态。
    // 否则等一会儿，这样玩家就可以记住图标的位置，然后再次关闭它们。
    let main_window_weak = main_window.as_weak();
    // 在 Rust 方面，我们现在可以向回调添加一个处理程序check_if_pair_solved，它将检查是否打开了两个图块
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles =
            tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved { // 如果它们2个匹配，则该solved属性在模型中设置为 true
                t1.solved = true; 
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {    
                // 如果它们不匹配，则启动一个计时器，该计时器将在一秒钟后关闭它们。当计时器运行时，禁用每个图块，因此在此期间不能单击任何内容。
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true); // 禁止单击

                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run().unwrap();

}

slint::slint! {
    // Added: tile 数据结构定义并将其粘贴到slint!宏内部的顶部：
    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
    }

    component GameOverTile inherits Rectangle {
        background: #3960D5;
        in property <bool> show;
    
        Text {
            width: show ? 326px : 0px;
            height: show ? 326px : 0px;
            text: "Game Over";
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
        // 分成左右两半的原因是点击后图块的关闭动画
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

        GameOverTile {
            background: #3911D5;
            show : true;
        }

        callback check_if_pair_solved(); // Added 回调函数：检查两个图块是否打开已配对
        in property <bool> disable_tiles; // Added
 
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
        // 该for tile[i] in memory_tiles:语法声明了一个变量 tile，
        // 其中包含数组中一个元素的数据memory_tiles，以及一个变量i，该变量是图块的索引。
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
                // old: tile.image_visible = !tile.image_visible;
                // new:
                if (!root.disable_tiles) { // 当计时器运行时，禁用每个图块，因此在此期间不能单击任何内容。
                    tile.image_visible = !tile.image_visible;
                    root.check_if_pair_solved();
                }
            }
        }
    }
}