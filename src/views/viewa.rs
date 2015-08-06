use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;

use ::views::viewb;

pub struct ViewA;

impl View for ViewA {
	fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
		let renderer = &mut context.renderer;
		let events = &context.events;

		if events.now.quit || Some(true) == events.now.key_escape {
			return ViewAction::Quit;
		}

		if Some(true) == events.now.key_space {
			return ViewAction::ChangeView(Box::new(viewb::ViewB));
		}

		renderer.set_draw_color(Color::RGB(255, 0, 0));
		renderer.clear();

		ViewAction::None
	}
}