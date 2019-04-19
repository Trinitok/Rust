// rustc 0.9 (7613b15 2014-01-08 18:04:43 -0800)

use std::process::Command;

fn main() {
	let x = 0;
	while x < 1000000 {
		Command::new("xdotool")
		.args(&["mousemove", "200", "200"])
		.spawn().ok();
	}

}