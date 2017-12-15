

#[ macro_use ]
extern crate rust_scheme;


use rust_scheme::exports::*;
use rust_scheme::runtime::exports::*;

use std::io;
use std::io::Read;
use std::io::Write;




fn main () -> () {
	main_0 () .expect ("f2d87179");
}


fn main_0 () -> (Outcome<()>) {
	
	let mut transcript = io::stdout ();
	let mut source_stream = io::stdin ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	
	let mut source = StdString::new ();
	match source_stream.read_to_string (&mut source) {
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0xb498c8f0);
			fail! (0x68c9854b);
		},
	}
	
	let expressions = match parse_script (&source) {
		Ok (expressions) =>
			expressions,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! parse !! => {:#?}\n", &error), 0x4b546a75);
			return Err (error);
		},
	};
	
	let expressions = match compile_script (&context, &expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! compile !! => {:#?}\n", &error), 0xeaf9b7f2);
			return Err (error);
		},
	};
	
	let expressions = match optimize_script (expressions) {
		Ok (expression) =>
			expression,
		Err (error) => {
			try_or_fail! (write! (transcript, "!! optimize !! => {:#?}\n", &error), 0x89f48a5b);
			return Err (error);
		},
	};
	
	for expression in expressions.into_iter () {
		try_or_fail! (write! (transcript, "\n--------------------------------------------------------------------------------\n"), 0x25f931a1);
		try_or_fail! (write! (transcript, "{:#?}\n", &expression), 0x829a2b78);
		try_or_fail! (write! (transcript, "--------------------------------------------------------------------------------\n\n"), 0xbfaa9836);
	}
	
	return Ok (());
}
