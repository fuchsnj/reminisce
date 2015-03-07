extern crate reminisce;
use reminisce::*;
fn main() {
	let joysticks = scan();
	let mut joysticks:Vec<_> = joysticks.into_iter().map(|v| v.with_state()).collect();
	for js in &joysticks {
		println!("Joystick #{}: {}", js.get_index(), js.get_id());
		println!("\tAxes: {}", js.get_num_axes());
		println!("\tButtons: {}", js.get_num_buttons());
	}
	if joysticks.len() == 0 {
		panic!("No joysticks plugged in")
	}
	loop {
		for js in &mut joysticks {
			if let Some(event) = js.poll() {
				match event {
					Event::ButtonPressed(Button::A) =>
						println!("({:?}, {:?})", js.get_normalised_axis(Axis::LeftX).unwrap(), js.get_normalised_axis(Axis::LeftY).unwrap()),
					_ =>
						println!("{:?}", event)
				}
			}
		}
	}
}