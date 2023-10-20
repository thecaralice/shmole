use std::{
	collections::BTreeMap,
	env::{args_os, current_dir, current_exe, vars_os},
};

fn main() {
	let args: Vec<_> = args_os().collect();
	let vars: BTreeMap<_, _> = vars_os().collect();
	// TODO: bold text for terminal emulators without ANSI support 
	// (looking at you, Windows)
	eprintln!(
		"\x1b[1mExecuted as\x1b[m {exe:?} in {dir:?}\n",
		exe = current_exe(),
		dir = current_dir(),
	);
	eprintln!("\x1b[1mArguments:\x1b[m {args:#?}\n");
	eprintln!("\x1b[1mEnvironment\x1b[m: {vars:#?}\n");
}
