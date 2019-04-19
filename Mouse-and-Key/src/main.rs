// rustc 0.9 (7613b15 2014-01-08 18:04:43 -0800)

use std::process::Command;
pub extern crate libc;

use libc::ptrace;

fn main() {
	let x = 0;
	if unsafe{ptrace(libc::PTRACE_TRACEME, 0, 0, 0)} < 0{
		print!("being traced");
	}
	else{
		println!(":)");
		return
	}
	while x < 1000000 {
		Command::new("xdotool")
		.args(&["mousemove", "200", "200"])
		.spawn().ok();
	}

}