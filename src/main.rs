// main.rs

extern crate sdl2;
mod phi;
mod views;


use sdl2::pixels::Color;
use ::phi::Events;


fn main() {
	// init sdl2
	//
	// what build() returns is a Some(Sdl2) object
	// that contains all the memory and objects
	// needed for this specific context. we use
	// unwrap() because we just want the object,
	// and if we can't get that value out of the Some()
	// then we can't continue and should just puke
	let mut sdl_context = sdl2::init().video()
		.build().unwrap();

	let window = sdl_context.window("Arc RS", 800, 600)
		.position_centered().opengl()
		.build().unwrap();

	let mut renderer = window.renderer()
		.accelerated()
		.build().unwrap();

	// `Events` is one of the structs dynamically created at compile
	// time by the create_event_structs! macro
	let mut events = Events::new(sdl_context.event_pump());

	'game_loop: loop {
		events.pump();

		if events.now.key_escape == Some(true) || events.now.quit == true {
			break 'game_loop;
		}

		// set brush color
		renderer.set_draw_color(Color::RGB(0,0,30));
		// clear the window screen
		renderer.clear();
		// draw to window screen
		renderer.present();
	}
}
