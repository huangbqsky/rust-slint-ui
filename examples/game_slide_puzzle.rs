// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use std::cell::RefCell;
use std::rc::Rc;
use slint::Model;

// slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn shuffle() -> Vec<i8> {
    fn is_solvable(positions: &[i8]) -> bool {
        // Same source as the flutter's slide_puzzle:
        // https://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html
        // This page seems to be no longer available, a copy can be found here:
        // https://horatiuvlad.com/unitbv/inteligenta_artificiala/2015/TilesSolvability.html

        let mut inversions = 0;
        for x in 0..positions.len() - 1 {
            let v = positions[x];
            inversions += positions[x + 1..].iter().filter(|x| **x >= 0 && **x < v).count();
        }
        //((blank on odd row from bottom) == (#inversions even))
        let blank_row = positions.iter().position(|x| *x == -1).unwrap() / 4;
        inversions % 2 != blank_row % 2
    }

    let mut vec = ((-1)..15).into_iter().collect::<Vec<i8>>();
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
    while !is_solvable(&vec) {
        vec.shuffle(&mut rng);
    }
    vec
}

struct AppState {
    pieces: Rc<slint::VecModel<Piece>>,
    main_window: slint::Weak<MainWindow>,
    /// An array of 16 values which represent a 4x4 matrix containing the piece number in that
    /// position. -1 is no piece.
    positions: Vec<i8>,
    auto_play_timer: slint::Timer,
    kick_animation_timer: slint::Timer,
    /// The speed in the x and y direction for the associated tile
    speed_for_kick_animation: [(f32, f32); 15],
    finished: bool,
}

impl AppState {
    fn set_pieces_pos(&self, p: i8, pos: i8) {
        if p >= 0 {
            self.pieces.set_row_data(
                p as usize,
                Piece { pos_y: (pos % 4) as _, pos_x: (pos / 4) as _, offset_x: 0., offset_y: 0. },
            );
        }
    }

    fn randomize(&mut self) {
        self.positions = shuffle();
        for (i, p) in self.positions.iter().enumerate() {
            self.set_pieces_pos(*p, i as _);
        }
        self.main_window.unwrap().set_moves(0);
        self.apply_tiles_left();
    }

    fn apply_tiles_left(&mut self) {
        let left = 15 - self.positions.iter().enumerate().filter(|(i, x)| *i as i8 == **x).count();
        self.main_window.unwrap().set_tiles_left(left as _);
        self.finished = left == 0;
    }

    fn piece_clicked(&mut self, p: i8) -> bool {
        let piece = self.pieces.row_data(p as usize).unwrap_or_default();
        assert_eq!(self.positions[(piece.pos_x * 4 + piece.pos_y) as usize], p);

        // find the coordinate of the hole.
        let hole = self.positions.iter().position(|x| *x == -1).unwrap() as i8;
        let pos = (piece.pos_x * 4 + piece.pos_y) as i8;
        let sign = if pos > hole { -1 } else { 1 };
        if hole % 4 == piece.pos_y as i8 {
            self.slide(pos, sign * 4)
        } else if hole / 4 == piece.pos_x as i8 {
            self.slide(pos, sign)
        } else {
            self.speed_for_kick_animation[p as usize] = (
                if hole % 4 > piece.pos_y as i8 { 10. } else { -10. },
                if hole / 4 > piece.pos_x as i8 { 10. } else { -10. },
            );
            return false;
        };
        self.apply_tiles_left();
        if let Some(x) = self.main_window.upgrade() {
            x.set_moves(x.get_moves() + 1);
        }
        true
    }

    fn slide(&mut self, pos: i8, offset: i8) {
        let mut swap = pos;
        while self.positions[pos as usize] != -1 {
            swap += offset;
            self.positions.swap(pos as usize, swap as usize);
            self.set_pieces_pos(self.positions[swap as usize] as _, swap);
        }
    }

    fn random_move(&mut self) {
        let mut rng = rand::thread_rng();
        let hole = self.positions.iter().position(|x| *x == -1).unwrap() as i8;
        let mut p;
        loop {
            p = rand::Rng::gen_range(&mut rng, 0..16);
            if hole == p {
                continue;
            } else if (hole % 4 == p % 4) || (hole / 4 == p / 4) {
                break;
            }
        }
        let p = self.positions[p as usize];
        self.piece_clicked(p);
    }

    /// Advance the kick animation
    fn kick_animation(&mut self) {
        /// update offset and speed, returns true if the animation is still running
        fn spring_animation(offset: &mut f32, speed: &mut f32) -> bool {
            const C: f32 = 0.3; // Constant = k/m
            const DAMP: f32 = 0.7;
            const EPS: f32 = 0.3;
            let acceleration = -*offset * C;
            *speed += acceleration;
            *speed *= DAMP;
            if *speed != 0. || *offset != 0. {
                *offset += *speed;
                if speed.abs() < EPS && offset.abs() < EPS {
                    *speed = 0.;
                    *offset = 0.;
                }
                true
            } else {
                false
            }
        }

        let mut has_animation = false;
        for idx in 0..15 {
            let mut p = self.pieces.row_data(idx).unwrap_or_default();
            let ax = spring_animation(&mut p.offset_x, &mut self.speed_for_kick_animation[idx].0);
            let ay = spring_animation(&mut p.offset_y, &mut self.speed_for_kick_animation[idx].1);
            if ax || ay {
                self.pieces.set_row_data(idx, p);
                has_animation = true;
            }
        }
        if !has_animation {
            self.kick_animation_timer.stop();
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let main_window = MainWindow::new().unwrap();

    #[cfg(target_arch = "wasm32")]
    handle_resize(main_window.as_weak());

    let state = Rc::new(RefCell::new(AppState {
        pieces: Rc::new(slint::VecModel::<Piece>::from(vec![Piece::default(); 15])),
        main_window: main_window.as_weak(),
        positions: vec![],
        auto_play_timer: Default::default(),
        kick_animation_timer: Default::default(),
        speed_for_kick_animation: Default::default(),
        finished: false,
    }));
    state.borrow_mut().randomize();
    main_window.set_pieces(state.borrow().pieces.clone().into());

    let state_copy = state.clone();
    main_window.on_piece_clicked(move |p| {
        state_copy.borrow().auto_play_timer.stop();
        state_copy.borrow().main_window.unwrap().set_auto_play(false);
        if state_copy.borrow().finished {
            return;
        }
        if !state_copy.borrow_mut().piece_clicked(p as i8) {
            let state_weak = Rc::downgrade(&state_copy);
            state_copy.borrow().kick_animation_timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    if let Some(state) = state_weak.upgrade() {
                        state.borrow_mut().kick_animation();
                    }
                },
            );
        }
    });

    let state_copy = state.clone();
    main_window.on_reset(move || {
        state_copy.borrow().auto_play_timer.stop();
        state_copy.borrow().main_window.unwrap().set_auto_play(false);
        state_copy.borrow_mut().randomize();
    });

    let state_copy = state;
    main_window.on_enable_auto_mode(move |enabled| {
        if enabled {
            let state_weak = Rc::downgrade(&state_copy);
            state_copy.borrow().auto_play_timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(200),
                move || {
                    if let Some(state) = state_weak.upgrade() {
                        state.borrow_mut().random_move();
                    }
                },
            );
        } else {
            state_copy.borrow().auto_play_timer.stop();
        }
    });
    main_window.run().unwrap();
}

#[cfg(target_arch = "wasm32")]
/// winit doesn't handle the resizing of the canvas from CSS well
/// https://github.com/rust-windowing/winit/issues/1661
/// in the mean time, adjust the size manually
fn handle_resize(slint_window: slint::Weak<MainWindow>) -> Option<()> {
    let window = web_sys::window()?;
    let resize_handler = move || {
        let window = web_sys::window()?;
        let doc = window.document()?;

        let container = doc.get_element_by_id("container")?;
        let children = container.children();
        let mut height_sum = 0.;
        for i in 0..children.length() {
            let child = children.item(i).unwrap();
            if child.tag_name().to_lowercase() == "canvas" {
                continue;
            }
            let style = window.get_computed_style(&child).ok()?;
            let get_margin = |m| {
                style.as_ref()?.get_property_value(m).ok()?.strip_suffix("px")?.parse::<f32>().ok()
            };
            height_sum += child.scroll_height() as f32
                + get_margin("margin-top").unwrap_or(0.)
                + get_margin("margin-bottom").unwrap_or(0.)
                + 1.;
        }

        let width = doc.body()?.client_width() as f32;
        let height = doc.body()?.client_height() as f32 - height_sum;
        slint_window.upgrade()?.window().set_size(slint::LogicalSize { width, height });
        Some(())
    };
    resize_handler();
    let closure = Closure::wrap(Box::new(move || drop(resize_handler())) as Box<dyn FnMut()>);
    window.set_onresize(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    Some(())
}

slint::slint! {
    // Copyright © SixtyFPS GmbH <info@slint-ui.com>
    // SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

    struct Piece  {
        // col/row position of the tile in the puzzle
        pos-x: int,
        pos-y: int,
        // offset in pixel from the base position for the kicking animation
        offset-x: length,
        offset-y: length,
    }

    struct Theme  {
        name: string,
        window-background-color: brush,
        game-background-color: brush,
        game-use-background-image: bool,
        game-border: length,
        game-radius: length,
        game-text-color: color,
        game-highlight-color: color,
        piece-border: length,
        piece-background-1: brush,
        piece-background-2: brush,
        piece-border-color-1: brush,
        piece-border-color-2: brush,
        piece-text-color-1: color,
        piece-text-color-2: color,
        piece-text-weight-incorrect-pos: int,
        piece-text-weight-correct-pos: int,
        piece-text-font-family: string,
        piece-radius: length,
        // Ratio of the piece size
        piece-spacing: float,
    }

    component Checkbox inherits Rectangle {
        in-out property <bool> checked;
        callback toggled(bool);
        in property<color> checked-color;
        in property<color> unchecked-color;

        hover-rect := Rectangle {
            background: #f5f5f5;
            x: - parent.width / 4;
            y: - parent.height / 4;
            width: ta.has-hover ? root.width * 1.5 : 0px;
            height: self.width;
            border-radius: self.width;
        }

        checkbox-rect := Rectangle {
            border-width: self.height * 10%;
            border-color: root.unchecked-color;
            border-radius: 2px;

            clip := Rectangle {
                x:0;
                width: 0px;
                clip: true;

                Text {
                    x:0;y:0;
                    width: root.width;
                    height: root.height;

                    text: "✓";
                    font-size: self.height * 80%;
                    color: white;
                    animate color { duration: 200ms; }
                    vertical-alignment: center;
                    horizontal-alignment: center;
                }
            }
            ta := TouchArea {
                clicked => {
                    root.checked = !root.checked;
                    root.toggled(root.checked);
                }
            }

        }

        states [
        /* pressed when ta.pressed : {
                clip.width: root.width;
                root.border-color: checked_color;
                root.border-width: root.width;
            }*/
            checked when root.checked : {
                clip.width: root.width;
                checkbox-rect.border-color: root.checked-color;
                checkbox-rect.border-width: root.width;
                in {
                    animate clip.width { duration: 200ms; easing: ease-in; }
                    animate checkbox-rect.border-width { duration: 100ms; easing: ease-out; }
                }
                out {
                    animate clip.width { duration: 100ms; easing: ease; }
                    animate checkbox-rect.border-width { duration: 200ms; easing: ease-in-out; }
                    animate checkbox-rect.border-color { duration: 200ms; easing: cubic-bezier(1,1,1,0); }
                }
            }
        ]
    }

    import "./plaster-font/Plaster-Regular.ttf";

    export component MainWindow inherits Window {
        title: "Slide Puzzle - Slint Demo";

        callback piece-clicked(int);
        callback reset();
        callback enable-auto-mode(bool);
        in-out property <bool> auto-play;
        in-out property <int> moves;
        in-out property <int> tiles-left;
        in property <[Piece]> pieces: [
            { pos-x: 0, pos-y: 0 },
            { pos-x: 0, pos-y: 1 },
            { pos-x: 0, pos-y: 2 },
            { pos-x: 0, pos-y: 3 },
            { pos-x: 1, pos-y: 0 },
            { pos-x: 1, pos-y: 1 },
            { pos-x: 1, pos-y: 2 },
            { pos-x: 1, pos-y: 3 },
            { pos-x: 2, pos-y: 0 },
            { pos-x: 2, pos-y: 1 },
            { pos-x: 2, pos-y: 2 },
            { pos-x: 2, pos-y: 3 },
            { pos-x: 3, pos-y: 0 },
            { pos-x: 3, pos-y: 1 },
            { pos-x: 3, pos-y: 2 },
        ];

        private property <[Theme]> themes: [
            {
                name: "SIMPLE",
                window-background-color: #ffffff,
                game-background-color: #ffffff,
                game-use-background-image: false,
                game-border: 1px,
                game-radius: 2px,
                game-text-color: #858585,
                game-highlight-color: #1d6aaa,
                piece-border: 1px,
                piece-background-1: #0d579b,
                piece-background-2: #0d579b,
                piece-border-color-1: #0a457b,
                piece-border-color-2: #0a457b,
                piece-text-color-1: #ffffff,
                piece-text-color-2: #ffffff,
                piece-text-weight-incorrect-pos: 400,
                piece-text-weight-correct-pos: 700,
                piece-radius: 5px,
                // Ratio of the piece size
                piece-spacing: 10%,
            },
            {
                name: "BERLIN",
                window-background-color: #ffffff88,
                game-background-color: #ffffffcc,
                game-use-background-image: true,
                game-border: 0px,
                game-radius: 2px,
                game-text-color: #858585,
                game-highlight-color: #1d6aaa,
                piece-border: 0px,
                piece-background-1: #2f689e,
                piece-background-2: #2f2a14,
                piece-border-color-1: #0000,
                piece-border-color-2: #0000,
                piece-text-color-1: #000000,
                piece-text-color-2: #ffffff,
                piece-text-weight-incorrect-pos: 700,
                piece-text-weight-correct-pos: 700,
                piece-radius: 0px,
                // Ratio of the piece size
                piece-spacing: 8%,
            },
            {
                name: "PLASTER",
                window-background-color: #424244,
                game-background-color: #f8f4e9,
                game-use-background-image: false,
                game-border: 5px,
                game-radius: 20px,
                game-text-color: #858585,
                game-highlight-color: #e06b53,
                piece-border: 4px,
                piece-background-1: #e06b53,
                piece-background-2: #f8f4e9,
                piece-border-color-1: #424244,
                piece-border-color-2: #e06b53,
                piece-text-color-1: #f8f4e9,
                piece-text-color-2: #424244,
                piece-text-weight-incorrect-pos: 700,
                piece-text-weight-correct-pos: 700,
                piece-text-font-family: "Plaster",
                piece-radius: 5px,
                // Ratio of the piece size
                piece-spacing: 10%,
            },
        ];

        out property<int> current-theme-index;
        private property <Theme> current-theme: root.themes[root.current-theme-index];

        private property<length> pieces-size: min(root.width, root.height) / 6;
        private property<length> pieces-spacing: root.current-theme.game-use-background-image && root.tiles-left == 0 ?
            2px : (root.pieces-size * root.current-theme.piece-spacing);

        animate pieces-spacing { duration: 500ms; easing: ease-out; }

        Image {
            // For the wasm build we want the puzzle to resize with the browser viewport, as per CSS in index.html.
            // Our winit backend preserves the CSS set size if there's no preferred size set on the Slint window.
            // This image propagates its preferred size and that means the window won't scale. By positioning it
            // manually, the preferred size is ignored.
            x: 0; y: 0;
            height: 100%; width: 100%;
            // https://commons.wikimedia.org/wiki/File:Berlin_potsdamer_platz.jpg Belappetit, CC BY-SA 3.0
            source: @image-url("./images/berlin.jpg");
            image-fit: cover;
        }

        Rectangle {
            background: root.current-theme.window-background-color;
            animate background { duration: 500ms; easing: ease-out; }
        }

        Rectangle {
            background: root.current-theme.game-background-color;
            border-color: root.current-theme.game-text-color;
            border-width: root.current-theme.game-border;
            border-radius: root.current-theme.game-radius;
            width: root.pieces-size * 4.6;
            height: root.pieces-size * 5.4;
            x: (parent.width - self.width)/2;
            y: (parent.height - self.height)/2;
            animate background, border-color, border-width, border-radius { duration: 500ms; easing: ease-out; }

            Rectangle {
                y:0;
                width: parent.width * 90%;
                height: root.pieces-size/2;
                x: (parent.width - self.width) / 2;
                HorizontalLayout {
                    spacing: 0px;
                    for theme[idx] in root.themes: TouchArea {
                        t := Text {
                            width: 100%; height: 100%;
                            text: theme.name;
                            color: idx == root.current-theme-index ? root.current-theme.game-highlight-color : root.current-theme.game-text-color;
                            vertical-alignment: center;
                            horizontal-alignment: center;
                        }
                        Rectangle {
                            background: t.color;
                            height: idx == root.current-theme-index ? 2px: 1px;
                            y: parent.height - self.height;
                        }
                        clicked => {
                            root.current-theme = theme;
                            root.current-theme-index = idx;
                        }
                    }
                }
            }


            for p[i] in root.pieces : Rectangle {
                x: self.py * (root.pieces-size + root.pieces-spacing) + p.offset-x
                    + (parent.width - (4*root.pieces-size + 3*root.pieces-spacing))/2;
                y: self.px * (root.pieces-size + root.pieces-spacing) + p.offset-y
                    + (parent.height - (4*root.pieces-size + 3*root.pieces-spacing))/2;
                width: root.pieces-size;
                height: root.pieces-size;
                property <bool> is-correct: i == p.pos-x * 4 + p.pos-y;

                drop-shadow-offset-x: 1px;
                drop-shadow-offset-y: 1px;
                drop-shadow-blur: 3px;
                drop-shadow-color: #00000040;
                border-radius: root.current-theme.piece-radius;
                clip: true;

                property<float> px: p.pos-x;
                property<float> py: p.pos-y;
                animate px , py { duration: 170ms; easing: cubic-bezier(0.17,0.76,0.4,1.75); }

                if (root.current-theme.game-use-background-image) : Image {
                    height: 100%; width: 100%;
                    // https://commons.wikimedia.org/wiki/File:Berlin_potsdamer_platz.jpg Belappetit, CC BY-SA 3.0
                    source: @image-url("./images/berlin.jpg");
                    source-clip-x: mod(i, 4) * self.source.width / 4;
                    source-clip-y: floor(i / 4) * self.source.height / 4;
                    source-clip-width: self.source.width / 4;
                    source-clip-height: self.source.height / 4;

                    if (root.tiles-left != 0) : Rectangle {
                        width: 60%;
                        height: 60%;
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        border-radius: self.width;
                        background: is-correct ? #0008 : #fff8;
                    }
                }

                if (!root.current-theme.game-use-background-image) : Rectangle {
                    background: i >= 8 ? root.current-theme.piece-background-2 : root.current-theme.piece-background-1;
                    border-color: i >= 8 ? root.current-theme.piece-border-color-2 : root.current-theme.piece-border-color-1;
                    border-width: root.current-theme.piece-border;
                    border-radius: root.current-theme.piece-radius;
                    animate border-width, border-radius { duration: 500ms; easing: ease-out; }
                }

                if (!root.current-theme.game-use-background-image || root.tiles-left > 0) : Text {
                    text: i+1;
                    color: ((!root.current-theme.game-use-background-image && i >= 8) || (root.current-theme.game-use-background-image && is-correct)) ? root.current-theme.piece-text-color-2 : root.current-theme.piece-text-color-1;
                    font-size: root.pieces-size / 3;
                    font-weight: is-correct ? root.current-theme.piece-text-weight-correct-pos : root.current-theme.piece-text-weight-incorrect-pos;
                    font-family: root.current-theme.piece-text-font-family;
                    vertical-alignment: center;
                    horizontal-alignment: center;
                    width: 100%;
                    height: 100%;
                }

                touch := TouchArea {
                    clicked => { root.piece-clicked(i); }
                }

                shadow := Rectangle {
                    circle := Rectangle {
                        height: self.width;
                        border-radius: self.width/2;
                        background: #0002;
                        x: touch.pressed-x - self.width/2;
                        y: touch.pressed-y - self.width/2;
                    }
                }

                states [
                    pressed when touch.pressed : {
                        shadow.color: #0002;
                        circle.width: shadow.width * 2 * 1.4142;
                    in  {
                        animate shadow.color { duration: 50ms; }
                        animate circle.width { duration: 2s; easing: ease-out; }
                    }
                    out  {
                        animate shadow.color { duration: 50ms; }
                    }
                    }
                    hover when touch.has-hover: {
                        shadow.color: #0000000d;
                    }

                ]

            }

            if (root.tiles-left == 0) : Text {
                width: root.pieces-size;
                height: root.pieces-size;
                x: 3 * (root.pieces-size + root.pieces-spacing)
                    + (parent.width - (4*root.pieces-size + 3*root.pieces-spacing))/2;
                y: 3 * (root.pieces-size + root.pieces-spacing)
                    + (parent.height - (4*root.pieces-size + 3*root.pieces-spacing))/2;

                color: root.current-theme.game-highlight-color;
                font-size: root.pieces-size / 2;
                vertical-alignment: center;
                horizontal-alignment: center;
                text: "🖒";

                if (root.current-theme.game-use-background-image) : Image {
                    height: 100%; width: 100%;
                    // https://commons.wikimedia.org/wiki/File:Berlin_potsdamer_platz.jpg Belappetit, CC BY-SA 3.0
                    source: @image-url("./images/berlin.jpg");
                    source-clip-x: 3 * self.source.width / 4;
                    source-clip-y: 3 * self.source.height / 4;
                    source-clip-width: self.source.width / 4;
                    source-clip-height: self.source.height / 4;
                }
            }

            Rectangle {
                width: parent.width;
                height: 1px;
                background: root.current-theme.game-text-color;
                y: parent.height - root.pieces-size / 2;
            }

            HorizontalLayout {
                height: root.pieces-size / 2;
                y: parent.height - root.pieces-size / 2;
                width: parent.width;
                padding: self.height * 25%;
                Text {
                    text: " ↻ ";
                    font-size: parent.height * 40%;
                    color: root.current-theme.game-highlight-color;
                    vertical-alignment: center;
                    TouchArea {
                        clicked => { root.reset(); }
                    }
                }
                Checkbox {
                    width: parent.height - 2 * parent.padding;
                    checked <=> root.auto-play;
                    toggled(checked) => { root.enable-auto-mode(self.checked) }
                    checked-color: root.current-theme.game-highlight-color;
                    unchecked-color: root.current-theme.game-text-color;
                }
                Rectangle {} // stretch
                Text {
                    text: root.moves;
                    color: root.current-theme.game-highlight-color;
                    vertical-alignment: center;
                }
                Text {
                    text: "Moves ";
                    color: root.current-theme.game-text-color;
                    vertical-alignment: center;
                }
                Text {
                    text: root.tiles-left;
                    color: root.current-theme.game-highlight-color;
                    vertical-alignment: center;
                }
                Text {
                    text: "Tiles left";
                    color: root.current-theme.game-text-color;
                    vertical-alignment: center;
                }
            }
        }
    }
}