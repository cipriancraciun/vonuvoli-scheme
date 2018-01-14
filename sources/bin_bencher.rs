

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;




fn main () -> () {
	main_0 () .expect ("ce1cf1fd");
}


#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
fn main_0 () -> (Outcome<()>) {
	
	let arguments = env::args () .collect::<StdVec<_>> ();
	let (identifier, source_path) = match arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			("<stdin>", None),
		2 =>
			(arguments[1].as_str (), Some (&arguments[1])),
		_ =>
			fail! (0x351c9060),
	};
	
	let transcript_backend = io::stdout ();
	let mut transcript = transcript_backend.lock ();
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all_with_prefix (try! (language_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0xe5edc9fc);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			try_or_fail! (write! (transcript, "!! input !! => {:#?}\n", &error), 0x8968ee53);
			fail! (0xd63850f0);
		},
	}
	
	return benchmark_tests_main (identifier, &source, Some (context), None, Some (&mut transcript), None, None);
}
