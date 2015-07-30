// phi/mod.rs
use ::sdl2::render::Renderer;


// #[macro_use] asks the compiler to import the macros defined
// in the `events` module. needed because macros cannot be namespaced
// as macro expansion happens before concept of namespace even starts
// to _exist_ in the compilation timeline.
#[macro_use] mod events;


create_event_structs!(
	keyboard: {
		key_escape: Escape,
		key_up: Up,
		key_down: Down
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
}

pub trait View {
	// called when 1self` becomes the main, rendered view
	fn resume(&mut self, context: &mut Phi);

	// called when `self` stops being the main, rendered view
	fn pause(&mut self, context: &mut Phi);

	// called on every frame to take care of both
	// logic and rendering of the current view
	//
	// `elapsed` is expressed in seconds
	fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
