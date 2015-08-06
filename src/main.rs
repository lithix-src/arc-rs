// main.rs

extern crate sdl2;
mod phi;
mod views;

fn main() {
	::phi::spawn("Arc-rs", |_| {
		Box::new(::views::DefaultView)
	});
}
