// phi/mod.rs
use ::sdl2::render::Renderer;
use ::sdl2::timer;


// #[macro_use] asks the compiler to import the macros defined
// in the `events` module. needed because macros cannot be namespaced
// as macro expansion happens before concept of namespace even starts
// to _exist_ in the compilation timeline.
#[macro_use] mod events;


create_event_structs!(
	keyboard: {
		key_escape: Escape,
		key_up: Up,
		key_down: Down,
		key_space: Space
	},
	else: {
		quit: Quit { .. }
	}
);

// bundles the Phi abstractions in a single structure
// which can be passed easily beteen functions
pub struct Phi<'a> {
    pub events: Events<'a>,
    pub renderer: Renderer<'a>,
}

// a `ViewAction` is a way for the executed view to
// communicate with the game loop. it specifies which
// action should be executed before the next rendering
pub enum  ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
	// called when 1self` becomes the main, rendered view
	fn resume(&mut self, _context: &mut Phi) {}

	// called when `self` stops being the main, rendered view
	fn pause(&mut self, _context: &mut Phi) {}

	// called on every frame to take care of both
	// logic and rendering of the current view
	//
	// `elapsed` is expressed in seconds
	fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

/// create a window with name `title`, initialize
/// the underlying libraries and start game with the `View`
/// returned by `init()`
pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
	// init SDL2
	let mut sdl_context = ::sdl2::init().timer().video()
		.build().unwrap();

	let window = sdl_context.window(title, 800, 600)
		.position_centered().opengl()
		.build().unwrap();

	let mut context = Phi {
		events: Events::new(sdl_context.event_pump()),
		renderer: window.renderer()
		.accelerated()
		.build().unwrap(),
	};

	// create and show default view
	let mut current_view = init(&mut context);
	current_view.resume(&mut context);

	// frame timing
	let interval = 1_000 / 60;
	let mut before = timer::get_ticks();
	let mut last_second = timer::get_ticks();
	let mut fps = 0u16;

	loop {
		// frame timing (bis)
		let now = timer::get_ticks();
		let dt = now - before;
		let elapsed = dt as f64 / 1_000.0;

		if dt < interval {
			timer::delay(interval - dt);
			continue;
		}

		before = now;
		fps += 1;

		if now - last_second > 1_000 {
			println!("FPS: {}", fps);
			last_second = now;
			fps = 0;
		}

		// logic & rendering
		context.events.pump();

		match current_view.render(&mut context, elapsed) {
			ViewAction::None => context.renderer.present(),
			ViewAction::Quit => {
				current_view.pause(&mut context);
				break;
			},
			ViewAction::ChangeView(new_view) => {
				current_view.pause(&mut context);
				current_view = new_view;
				current_view.resume(&mut context);
			}
		}
	}
}
