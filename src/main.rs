// main.rs

extern crate sdl2;
mod phi;
mod views;

fn main() {
	::phi::spawn("Arc-rs", |phi| {
		Box::new(::views::ShipView::new(phi))
	});
}
