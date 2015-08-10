// phi/events.rs

macro_rules! create_event_structs {
	(
		keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },

		// match against a pattern
		else: { $( $e_alias:ident : $e_sdl:pat ),* }
	) => {
		use ::sdl2::event::EventPump;

		pub struct ImmediateEvents {
			// For all keyboard events, we have an Option<bool>
			// Some(true) => Was just pressed
			// Some(false) => Was just released
			// None => Nothing happening _now_
			resize: Option<(u32, u32)>,
			$( pub $k_alias: Option<bool> , )*
			$( pub $e_alias: bool ),*
		}

		impl ImmediateEvents {
			pub fn new() -> ImmediateEvents {
				ImmediateEvents {
					// when reinitialized, nothing has yet happend,
					// so all are set to None
					resize: None,
					$( $k_alias: None , )*
					$( $e_alias: false ),*
				}
			}
		}

		pub struct Events<'p> {
			pump: EventPump<'p>,
			pub now: ImmediateEvents,

			// true => pressed
			// false => not pressed
			$( pub $k_alias: bool ),*
		}

		impl<'p> Events<'p> {
			pub fn new(pump: EventPump<'p>) -> Events<'p> {
				Events {
					pump: pump,
					now: ImmediateEvents::new(),

					// by default, init every key with _not pressed_
					$( $k_alias: false ),*
				}
			}

			pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
				self.now = ImmediateEvents::new();

				for event in self.pump.poll_iter() {
					use ::sdl2::event::Event::*;
					use ::sdl2::keyboard::Keycode::*;

					match event {
						Window { win_event_id: Resized, .. } => {
							self.now.resize = Some(renderer.get_output_size().unwrap());
						},

						KeyDown { keycode, .. } => match keycode {
							// $( ... ),* containing $k_sdl and $k_alias means:
							// 'for every element ($k_alias : $k_sdl ) pair,
							// check whether the keycode is Some($k_sdl). if
							// it is, then set the $k_aliast fields to true.'
							$(
								Some($k_sdl) => {
									// prevent multiple presses when keeping a key down
									// was previously not pressed?
									if !self.$k_alias {
										// key pressed
										self.now.$k_alias = Some(true);
									}

									self.$k_alias = true;
								}
							),*  // and add a comma after every option
							_ => {}
						},

						KeyUp { keycode, .. } => match keycode {
							$(
								Some($k_sdl) => {
									// key released
									self.now.$k_alias = Some(false);
									self.$k_alias = false;
								}
							),*
							_ => {}
						},

						$( $e_sdl => { self.now.$e_alias = true; })*,

						_ => {}
					}
				}
			}
		}
	}
}
