//! 
//! A simple demonstration of how to construct and use Canvasses by splitting up the window.
//!
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate graphics;

use conrod::{Canvas, Theme, Widget};
use piston_window::*;
use conrod::color::{Color};

type Ui = conrod::Ui<Glyphs>;
const APP_TITLE: &'static str = "Hangende";
const KEYBOARD_MAP:[&'static str;3] = ["qwertyuiop[", "asdfghjkl;'","zxcvbnm,./"];
const LETTER_MAP: [&'static str; 3] = ["qwertyuiopü", "asdfghjklöä", "zxcvbnmẞ  "];

struct HungmanApp {
	keys_matrix: Vec<Vec<bool>>
}

impl HungmanApp {
	fn new() -> HungmanApp {
		HungmanApp {
			keys_matrix : vec![vec![true; 30];3]
		}
	}
}

fn main() {
    // Construct the window.
    let mut hm = HungmanApp::new();
 	let opengl = OpenGL::V3_2;
    let window: PistonWindow =
        WindowSettings::new(
            "Hello Conrod".to_string(),
            Size { width: 1100, height: 550 }
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    // construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    // Poll events from the window.
    for event in window {
        event.text(|text| {
        		if let Some(chr) = text.chars().nth(0) {
        			for (l, s) in KEYBOARD_MAP.iter().enumerate() {
        				if let Some(c) = s.chars().position(|s: char| s == chr) {
        					hm.keys_matrix[l][c] = false;
        				}
        			}
        		}
        		println!("{:?}", text);
        	});
        ui.handle_event(&event);
        event.draw_2d(|c, g| draw_ui(&mut ui, c, g, &mut hm));
    }

}


// Draw the Ui.
fn draw_ui(ui: &mut Ui, c: Context, g: &mut G2d, hm: &mut HungmanApp) {
    use conrod::color::{blue, light_orange, orange, dark_orange, red, white};
    use conrod::{Button, Toggle, Colorable, Label, Labelable, Positionable, Sizeable, Split, Tabs,
                 WidgetMatrix};

    // Construct our Canvas tree.
    Split::new(MASTER).flow_down(&[
        Split::new(HEADER).length(90.0).color(blue()),
        Split::new(BODY).flow_right(&[
            Split::new(MIDDLE_COLUMN).length(200.0).color(orange()),
            Split::new(GAME).flow_down(&[
                    Split::new(LEFT_COLUMN).color(light_orange()).pad(20.0),
                    Split::new(RIGHT_COLUMN).color(dark_orange()).pad(20.0),
            ]),
        ]),
    ]).set(ui);


    Canvas::new()
        .show_title_bar(true)
        .floating(false)
        .label("Orange")
        .mid_left_of(LEFT_COLUMN)
        .color(light_orange())
        .label_color(white())
        .dimensions(ui.widget_size(LEFT_COLUMN)[0]/3.0,ui.widget_size(LEFT_COLUMN)[1])
        .set(FLOATING_B, ui);

    Tabs::new(&[(TAB_FOO, "FOO"),
                (TAB_BAR, "BAR"),
                (TAB_BAZ, "BAZ")])
        .dim(ui.widget_size(MIDDLE_COLUMN))
        .color(blue())
        .label_color(white())
        .middle_of(MIDDLE_COLUMN)
        .set(TABS, ui);

    Label::new(APP_TITLE).color(light_orange()).font_size(48).middle_of(HEADER).set(TITLE, ui);
    Label::new("Subtitle").color(blue().complement()).mid_bottom_of(HEADER).set(SUBTITLE, ui);

    Label::new("Top Left")
        .color(light_orange().complement())
        .top_left_of(LEFT_COLUMN)
        .set(TOP_LEFT, ui);

    Label::new("Bottom Right")
        .color(dark_orange().complement())
        .bottom_right_of(RIGHT_COLUMN)
        .set(BOTTOM_RIGHT, ui);

    Label::new("Foo!").color(white()).font_size(36).middle_of(TAB_FOO).set(FOO_LABEL, ui);
    Label::new("Bar!").color(white()).font_size(36).middle_of(TAB_BAR).set(BAR_LABEL, ui);
    Label::new("BAZ!").color(white()).font_size(36).middle_of(TAB_BAZ).set(BAZ_LABEL, ui);


    Button::new().color(red()).dimensions(30.0, 30.0).middle_of(FLOATING_B)
        .react(|| println!("Bong!"))
        .set(BONG, ui);

    WidgetMatrix::new(COLS, ROWS)
        .dimensions(550.0, 150.0)
        .mid_bottom_of(RIGHT_COLUMN)
        .each_widget(ui, |ui, n, _col, _row, xy, dim| {

        	let dim2 = [dim[0]*0.9, dim[1]*0.9];
        	let xy2 =  [xy[0] + (_row as f64)*10.0, xy[1]];
        	let (r, g, b, a) = (
                0.5 + (_col as f32 / COLS as f32) / 2.0,
                0.75,
                1.0 - (_row as f32 / ROWS as f32) / 2.0,
                1.0
            );

			let mut label = String::from("");
        	if let Some(chr) = LETTER_MAP[_row].chars().nth(_col) {
 				label.push(chr);
           	}

            Toggle::new(hm.keys_matrix[_row][_col])
				.rgba(r,g,b,a)
                .dim(dim2)
                .point(xy2)
                .react(|new_val:bool| {
                		 println!("Hey! {:?}", n);
        				hm.keys_matrix[_row][_col] = false;
                		 })
                .label(&label)
                .set(BUTTON + n, ui);
        });


    ui.draw_if_changed(c, g);
    draw_hangman(ui, g, blue(), [330.0, 100.0], [200.0, 200.0] );
}

fn draw_hangman(ui: &mut Ui, g: &mut G2d, color: Color,
	xy:[f64; 2], dim:[f64;2]) {
	use graphics::*;
	let head_prop = 5.0;

	let hang1_dim = [xy[0] + dim[0]*3.0/4.0,
					xy[1],
					xy[0] + dim[0]*3.0/4.0,
					xy[1] + dim[1]];

	let hang2_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1],
					xy[0] + dim[0],
					xy[1] + dim[1]];

	let hang3_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1],
					xy[0] + dim[0]*3.0/4.0,
					xy[1]];

	let hang4_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*1.0/5.0,
					xy[0] + dim[0]*1.0/2.0,
					xy[1]];

	let head_dim = [xy[0] + dim[0]*2.0/7.0,
					xy[1] + dim[1]*1.0/5.0,
	 				dim[0]/head_prop,
	 				dim[1]/head_prop];

    Ellipse::new_border(color.to_fsa(), 5.0)
        .draw(head_dim,
              default_draw_state(),
              math::abs_transform(ui.win_w, ui.win_h),
              g);

	let mut draw_line = |dim| {
		Line::new(color.to_fsa(), 5.0)
        	.draw(dim, default_draw_state(),
                      math::abs_transform(ui.win_w, ui.win_h), g);
        };

	draw_line(hang1_dim);
	draw_line(hang2_dim);
	draw_line(hang3_dim);
	draw_line(hang4_dim);

	let body_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*2.0/5.0,
					xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*3.0/4.0];

	draw_line(body_dim);

	let leg1_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*3.0/4.0,
					xy[0] + dim[0]*3.0/5.0,
					xy[1] + dim[1]*7.0/8.0];

	let leg2_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*3.0/4.0,
					xy[0] + dim[0]*2.0/5.0,
					xy[1] + dim[1]*7.0/8.0];

	draw_line(leg1_dim);
	draw_line(leg2_dim);

	let hand1_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*2.0/4.0,
					xy[0] + dim[0]*3.0/5.0,
					xy[1] + dim[1]*5.0/8.0];

	let hand2_dim = [xy[0] + dim[0]*1.0/2.0,
					xy[1] + dim[1]*2.0/4.0,
					xy[0] + dim[0]*2.0/5.0,
					xy[1] + dim[1]*5.0/8.0];

	draw_line(hand1_dim);
	draw_line(hand2_dim);
}

// Button matrix dimensions.
const ROWS: usize = 3;
const COLS: usize = 11;

// Generate a unique `WidgetId` for each widget.
widget_ids! {

    // Canvas IDs.
    MASTER,
    HEADER,
    BODY,
    GAME,
    LEFT_COLUMN,
    MIDDLE_COLUMN,
    RIGHT_COLUMN,
    FLOATING_A,
    FLOATING_B,
    TABS,
    TAB_FOO,
    TAB_BAR,
    TAB_BAZ,

    // Widget IDs.
    TITLE,
    SUBTITLE,
    TOP_LEFT,
    BOTTOM_RIGHT,
    FOO_LABEL,
    BAR_LABEL,
    BAZ_LABEL,
    BING,
    BONG,
	BUTTON with COLS * ROWS, 
}