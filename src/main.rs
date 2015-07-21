extern crate sdl2;

// #[macro_use] asks the compiler to import the macros defined
// in the `events` module. needed because macros cannot be namespaced
// as macro expansion happens before concept of namespace even starts
// to _exist_ in the compilation timeline.
#[macro_use] mod events;

// we cannot call functions at top-level. however, `struct_events`
// is a macro. which means that you can use a macro to do
// pretty much anything _normal_ code would.
struct_events!(
	keyboard: {
		key_escape: Escape,
		key_up: Up,
		key_down: Down
	},
	else: {
		quit: Quit { .. }
	}
);


use sdl2::pixels::Color;


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
