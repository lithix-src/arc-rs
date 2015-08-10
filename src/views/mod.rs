use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::phi::data::Rectangle;

// constants
const PLAYER_SPEED: f64 = 180.0;

// data types
struct Ship {
	rect: Rectangle,
}

// view definition
pub struct ShipView {
	player: Ship,
}

impl ShipView {
	pub fn new(phi: &mut Phi) -> ShipView {
		ShipView {
			player: Ship {
				rect: Rectangle {
					x: 64.0,
					y: 64.0,
					w: 32.0,
					h: 32.0
				}
			}
		}
	}
}

impl View for ShipView {
	fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
		let events = &phi.events;

		if events.now.quit || events.now.key_escape == Some(true) {
			return ViewAction::Quit;
		}

		let diagonal = (events.key_up ^ events.key_down) && (events.key_left ^ events.key_right);
		let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;

		let dx = match (events.key_left, events.key_right) {
			(true, true) | (false, false) => 0.0,
			(true, false) => -moved,
			(false, true) => moved,
		};

		let dy = match(events.key_up, events.key_down) {
			(true, true) | (false, false) => 0.0,
			(true, false) => -moved,
			(false, true) => moved,
		};

		self.player.rect.x += dx;
		self.player.rect.y += dy;

		// movable region spans entire height of window and 70%
		// of its width. this way player cannot get to far right
		// of screen, where we spawn asteroids
		let movable_region = Rectangle {
			x: 0.0,
			y: 0.0,
			w: phi.output_size().0 as f64 * 0.70,
			h: phi.output_size().1 as f64,
		};

		// still need to do the resize logic
		self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

		phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
		phi.renderer.clear();

		phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
		phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

		ViewAction::None
	}
}