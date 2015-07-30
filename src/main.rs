// main.rs

extern crate sdl2;
mod phi;
mod views;


use ::phi::{Events, ViewAction};
use ::sdl2::timer;


const WINDOW_TITLE: &'static str = "Arc RS";
static WINDOW_SIZE_X: u32 = 800;
static WINDOW_SIZE_Y: u32 = 600;

fn main() {
	// init sdl2
	//
	// what build() returns is a Some(Sdl2) object
	// that contains all the memory and objects
	// needed for this specific context. we use
	// unwrap() because we just want the object,
	// and if we can't get that value out of the Some()
	// then we can't continue and should just puke
	let mut sdl_context = sdl2::init().timer().video()
		.build().unwrap();

	// TODO: get the window size from config db
	let current_x_sz: &u32 = &WINDOW_SIZE_X;
	let current_y_sz: &u32 = &WINDOW_SIZE_Y;
	let window = sdl_context.window(WINDOW_TITLE, *current_x_sz, *current_y_sz)
		.position_centered().opengl()
		.build().unwrap();

	// create context
	let mut context = ::phi::Phi {
		events: Events::new(sdl_context.event_pump()),
		renderer: window.renderer()
			.accelerated()
			.build().unwrap(),
	};

	// create and show the default view
	let mut current_view: Box<::phi::View> = Box::new(::views::DefaultView);
	current_view.resume(&mut context);

	'game_loop: loop {
		context.events.pump();

		match current_view.render(&mut context, 0.01f64) {
			ViewAction::None => context.renderer.present(),
			ViewAction::Quit => {
				current_view.pause(&mut context);
				break 'game_loop;
			},
		}
	}
}
