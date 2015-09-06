
extern crate conrod;
extern crate piston_window;
extern crate graphics;

//use conrod::{Canvas, Theme, Widget};
use conrod::color::{Color};
use piston_window::*;

pub enum ManState {
	Free,
	Alive(u8),
	Dead
}

const MAX_STAGES: u8 = 5;

impl ManState {
	pub fn from_mistakes(nr: u32) -> ManState {
		if nr == 0 {
			ManState::Free
		} else if (nr -1) < MAX_STAGES as u32 {
			ManState::Alive((nr - 1) as u8)
		} else {
			ManState::Dead
		}
	}
}


pub struct Hangman {
	color: Color,
	xy: [f64;  2],
	dim: [f64; 2],
	state: ManState,
}


impl Hangman {
	pub fn new(color: Color, xy : [f64;2], dim: [f64;2]) -> Hangman {
		Hangman {
			color: color,
			xy: xy,
			dim: dim,
			state: ManState::Free
		}
	}

	pub fn state(mut self, new_state: ManState) -> Self {
		self.update_state(new_state);
		self
	}

	pub fn update_state(&mut self, new_state: ManState) {
		self.state = new_state;
	}

	pub fn draw_line(&self, dim: [f64; 4], ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {
                Line::new(self.color.to_fsa(), 5.0)
                .draw(dim, default_draw_state(),
                      math::abs_transform(ui.win_w, ui.win_h), g);
	}

	pub fn draw_hang(&self, ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {

        let hang1_dim = [self.xy[0] + self.dim[0]*3.0/4.0,
                                        self.xy[1],
                                        self.xy[0] + self.dim[0]*3.0/4.0,
                                        self.xy[1] + self.dim[1]];

        let hang2_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1],
                                        self.xy[0] + self.dim[0],
                                        self.xy[1] + self.dim[1]];

        let hang3_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1],
                                        self.xy[0] + self.dim[0]*3.0/4.0,
                                        self.xy[1]];

        let hang4_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*1.0/5.0,
                                        self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1]];

        self.draw_line(hang1_dim, ui, g);
        self.draw_line(hang2_dim, ui, g);
        self.draw_line(hang3_dim, ui, g);
        self.draw_line(hang4_dim, ui, g);
	}

	pub fn draw_head_dead(&self, ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {
        let head_prop = 5.0;

        let head_dim = [self.xy[0] + self.dim[0]*2.0/7.0,
                        self.xy[1] + self.dim[1]*1.0/5.0,
                        self.dim[0]/head_prop,
                        self.dim[1]/head_prop];

        Ellipse::new_border(self.color.to_fsa(), 5.0)
                 .draw(head_dim, default_draw_state(),
                      math::abs_transform(ui.win_w, ui.win_h), g);
    }

	pub fn draw_head(&self, ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {
        let head_prop = 5.0;

        let head_dim = [self.xy[0] + self.dim[0]*1.0/2.0 - self.dim[0]/head_prop/2.0,
                        self.xy[1] + self.dim[1]*1.0/5.0,
                        self.dim[0]/head_prop,
                        self.dim[1]/head_prop];

        Ellipse::new_border(self.color.to_fsa(), 5.0)
                 .draw(head_dim, default_draw_state(),
                      math::abs_transform(ui.win_w, ui.win_h), g);
    }
	
	pub fn draw_body(&self, stage: u8, ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {
		use std::cmp::{min};

        let body_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*2.0/5.0,
                                        self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*3.0/4.0];


        let leg1_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*3.0/4.0,
                                        self.xy[0] + self.dim[0]*3.0/5.0,
                                        self.xy[1] + self.dim[1]*7.0/8.0];

        let leg2_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*3.0/4.0,
                                        self.xy[0] + self.dim[0]*2.0/5.0,
                                        self.xy[1] + self.dim[1]*7.0/8.0];


        let hand1_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*2.0/4.0,
                                        self.xy[0] + self.dim[0]*3.0/5.0,
                                        self.xy[1] + self.dim[1]*5.0/8.0];

        let hand2_dim = [self.xy[0] + self.dim[0]*1.0/2.0,
                                        self.xy[1] + self.dim[1]*2.0/4.0,
                                        self.xy[0] + self.dim[0]*2.0/5.0,
                                        self.xy[1] + self.dim[1]*5.0/8.0];

		let pieces_dim = [
            body_dim,
            hand1_dim,
            hand2_dim,
            leg1_dim,
            leg2_dim,
        ];

		let part_max = min(stage, pieces_dim.len() as u8) as usize;
		for part in 0..part_max {
        	self.draw_line(pieces_dim[part], ui, g);
		}
	}

    pub fn draw(&self, ui: &mut conrod::Ui<Glyphs>, g: &mut G2d) {

    	self.draw_hang(ui, g);
    	match self.state {
    		ManState::Dead => {
				self.draw_body(MAX_STAGES, ui, g);
				self.draw_head_dead(ui, g);
    		},

    		ManState::Alive(x) => {
				self.draw_head(ui, g);
				self.draw_body(x, ui, g);
    		},
    		_ => {} ,
    	}
   }
}
