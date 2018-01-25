

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::ports::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
		
		is_port_input_open, is_port_output_open,
		
		port_close, port_input_close, port_output_close,
		
	};
	
	pub use super::{
		
		port_input_byte_peek, port_input_byte_read, port_input_byte_ready,
		port_input_character_peek, port_input_character_read, port_input_character_ready,
		
		port_input_bytes_read_copy_range,
		
		port_input_bytes_read_collect, port_input_bytes_read_extend,
		port_input_string_read_collect, port_input_string_read_extend,
		
		port_input_bytes_read_collect_until, port_input_bytes_read_extend_until,
		port_input_string_read_collect_until, port_input_string_read_extend_until,
		
		port_input_read_line,
		
	};
	
	pub use super::{
		
		port_output_byte_write, port_output_bytes_write,
		port_output_character_write, port_output_string_write,
		port_output_flush,
		
	};
	
	pub use super::{
		
		port_call_and_close,
		
	};
	
	pub use super::{
		
		port_bytes_reader_new, port_bytes_writer_new, port_bytes_writer_finalize,
		port_string_reader_new, port_string_writer_new, port_string_writer_finalize,
		
	};
	
	pub use super::{
		
		port_native_reader_new,
		port_native_writer_new,
		
		port_file_reader_open, port_file_reader_open_with_options,
		port_file_writer_open, port_file_writer_open_with_options,
		
		port_file_exists,
		port_file_delete,
		
	};
	
	pub use super::{
		
		port_output_value_display, port_output_value_display_0, port_output_value_display_0_slice, port_output_value_display_0_iterable,
		port_output_value_write, port_output_value_write_0, port_output_value_write_0_slice, port_output_value_write_0_iterable,
		port_output_newline, port_output_newline_0,
		
	};
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_input_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_input_open ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn is_port_output_open (port : &Value) -> (Outcome<bool>) {
	succeed! (try_as_port_ref! (port) .is_output_open ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.close ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.input_close ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_close (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_close ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_peek ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (byte) = try! (port.byte_read ()) {
		succeed! (byte.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_byte_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.byte_ready ();
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_peek (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_peek ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_read (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	if let Some (char) = try! (port.char_read ()) {
		succeed! (char.into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_character_ready (port : &Value) -> (Outcome<bool>) {
	let port = try_as_port_ref! (port);
	return port.char_ready ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_copy_range (port : &Value, bytes : &Value, range_start : Option<&Value>, range_end : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let mut buffer = bytes.bytes_ref_mut ();
	let full = range_start.is_some () || range_end.is_some ();
	let (range_start, range_end) = try! (range_coerce (range_start, range_end, buffer.len ()));
	let buffer = try_some! (buffer.get_mut (range_start .. range_end), 0xb8c1be42);
	if let Some (count) = try! (port.byte_read_slice (buffer, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect (port : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let mut buffer = StdVec::with_capacity (count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.byte_read_extend (&mut buffer, count, full)) {
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend (port : &Value, bytes : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let count = try! (count_coerce (count));
	let mut buffer = bytes.bytes_ref_mut ();
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (count) = try! (port.byte_read_extend (&mut buffer, count, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect (port : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let mut buffer = StdString::with_capacity (count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.char_read_string (&mut buffer, count, full)) {
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend (port : &Value, string : &Value, count : Option<&Value>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let string = try_as_string_mutable_ref! (string);
	let count = try! (count_coerce (count));
	let mut buffer = string.string_ref_mut ();
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (count) = try! (port.char_read_string (&mut buffer, count, full)) {
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_collect_until (port : &Value, delimiter : Option<&Value>, count : Option<&Value>, include_delimiter : Option<bool>) -> (Outcome<Value>) {
	// FIXME:  Verify how `count` and `full` fit for `read_*_until` family of functions!
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { '\n' as u8 };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdVec::with_capacity (count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0x87f51301);
			}
		}
		succeed! (bytes_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_bytes_read_extend_until (port : &Value, bytes : &Value, delimiter : Option<&Value>, count : Option<&Value>, include_delimiter : Option<bool>) -> (Outcome<Value>) {
	// FIXME:  Verify how `count` and `full` fit for `read_*_until` family of functions!
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_mutable_ref! (bytes);
	let count = try! (count_coerce (count));
	let delimiter = if let Some (delimiter) = delimiter { try! (try_as_number_integer_ref! (delimiter) .try_to_u8 ()) } else { '\n' as u8 };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = bytes.bytes_ref_mut ();
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (count) = try! (port.byte_read_extend_until (&mut buffer, delimiter, count, full)) {
		let count = if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
					count
				} else {
					count - 1
				}
			} else {
				fail_panic! (0x1ccb568e);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_collect_until (port : &Value, delimiter : Option<&Value>, count : Option<&Value>, include_delimiter : Option<bool>) -> (Outcome<Value>) {
	// FIXME:  Verify how `count` and `full` fit for `read_*_until` family of functions!
	let port = try_as_port_ref! (port);
	let count = try! (count_coerce (count));
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { '\n' };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = StdString::with_capacity (count.unwrap_or (DEFAULT_PORT_BUFFER_SIZE));
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (_) = try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) {
		if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
				}
			} else {
				fail_panic! (0xec6380c4);
			}
		}
		succeed! (string_new (buffer) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_string_read_extend_until (port : &Value, string : &Value, delimiter : Option<&Value>, count : Option<&Value>, include_delimiter : Option<bool>) -> (Outcome<Value>) {
	// FIXME:  Verify how `count` and `full` fit for `read_*_until` family of functions!
	let port = try_as_port_ref! (port);
	let string = try_as_string_mutable_ref! (string);
	let count = try! (count_coerce (count));
	let delimiter = if let Some (delimiter) = delimiter { try_as_character_ref! (delimiter) .value () } else { '\n' };
	let include_delimiter = include_delimiter.unwrap_or (false);
	let mut buffer = string.string_ref_mut ();
	let (count, full) = (Some (count.unwrap_or (buffer.capacity ())), count.is_some ());
	if let Some (count) = try! (port.char_read_string_until (&mut buffer, delimiter, count, full)) {
		let count = if ! include_delimiter {
			if let Some (last) = buffer.pop () {
				if last != delimiter {
					buffer.push (last);
					count
				} else {
					count - 1
				}
			} else {
				fail_panic! (0xd2798fc4);
			}
		} else {
			count
		};
		succeed! (try! (NumberInteger::try_from (count)) .into ());
	} else {
		succeed! (PORT_EOF.into ());
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_input_read_line (port : &Value, include_delimiter : Option<bool>) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let delimiter = '\n';
	let include_delimiter = include_delimiter.unwrap_or (false);
	// FIXME:  Decide if we should use the `char` or `byte` port interfaces!
	if false {
		let mut buffer = StdString::with_capacity (DEFAULT_PORT_BUFFER_SIZE);
		if let Some (_) = try! (port.char_read_string_until (&mut buffer, delimiter, None, true)) {
			if ! include_delimiter {
				if let Some (last) = buffer.pop () {
					if last != delimiter {
						buffer.push (last);
					}
				} else {
					fail_panic! (0xca581872);
				}
			}
			succeed! (string_new (buffer) .into ());
		} else {
			succeed! (PORT_EOF.into ());
		}
	} else {
		let delimiter = delimiter as u8;
		let mut buffer = StdVec::with_capacity (DEFAULT_PORT_BUFFER_SIZE);
		if let Some (_) = try! (port.byte_read_extend_until (&mut buffer, delimiter, None, true)) {
			if ! include_delimiter {
				if let Some (last) = buffer.pop () {
					if last != delimiter {
						buffer.push (last);
					}
				} else {
					fail_panic! (0xba2e4baa);
				}
			}
			if let Ok (buffer) = StdString::from_utf8 (buffer) {
				succeed! (string_new (buffer) .into ());
			} else {
				fail! (0x0c3a0397);
			}
		} else {
			succeed! (PORT_EOF.into ());
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_byte_write (port : &Value, byte : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let byte = try_as_number_integer_ref! (byte);
	let byte = try! (byte.try_to_u8 ());
	return port.byte_write (byte);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_bytes_write (port : &Value, bytes : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let bytes = try_as_bytes_ref! (bytes);
	let bytes = bytes.bytes_as_slice ();
	try! (port.byte_write_slice (bytes, true));
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_character_write (port : &Value, char : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let char = try_as_character_ref! (char);
	let char = char.value ();
	return port.char_write (char);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_string_write (port : &Value, string : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let string = try_as_string_ref! (string);
	let string = string.string_as_str ();
	try! (port.char_write_string (string, true));
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_flush (port : &Value) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	return port.output_flush ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_call_and_close (port : &Value, callable : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	try_as_port_ref! (port);
	let outcome = evaluator.evaluate_procedure_call_1 (callable, port);
	let port = try_as_port_ref! (port);
	try! (port.close ());
	return outcome;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_reader_new (bytes : &Value) -> (Outcome<Value>) {
	let bytes = try_as_bytes_ref! (bytes);
	let port = match bytes {
		BytesRef::Immutable (ref bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_immutable (bytes, 0, None))
		},
		BytesRef::Mutable (ref bytes, _) => {
			let bytes = bytes.bytes_rc_clone ();
			try! (Port::new_bytes_reader_from_bytes_mutable (bytes, 0, None))
		},
	};
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_reader_new (string : &Value) -> (Outcome<Value>) {
	let string = try_as_string_ref! (string);
	let port = match string {
		StringRef::Immutable (ref string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_immutable (string, 0, None))
		},
		StringRef::Mutable (ref string, _) => {
			let string = string.string_rc_clone ();
			try! (Port::new_bytes_reader_from_string_mutable (string, 0, None))
		},
	};
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_new (buffer : Option<usize>) -> (Outcome<Value>) {
	let port = try! (Port::new_bytes_writer (buffer));
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_bytes_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = port.internals_ref_mut ();
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			succeed! (bytes_new (buffer) .into ());
		},
		_ =>
			fail! (0x2c8a3119),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_string_writer_finalize (port : &Value) -> (Outcome<Value>) {
	let port = try_as_port_ref! (port);
	let mut port = port.internals_ref_mut ();
	let port = port.backend_ref_mut ();
	match *port {
		PortBackend::BytesWriter (ref mut backend) => {
			let buffer = try! (backend.finalize ());
			if let Ok (string) = StdString::from_utf8 (buffer) {
				succeed! (string_new (string) .into ());
			} else {
				fail! (0xfa7d2f1a);
			}
		},
		_ =>
			fail! (0xac1839d4),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_reader_new (reader : StdBox<io::Read>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_reader_from_unbuffered (reader, buffer, descriptor));
	succeed! (port.into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_native_writer_new (writer : StdBox<io::Write>, buffer : Option<usize>, descriptor : Option<PortDescriptor>) -> (Outcome<Value>) {
	let port = try! (Port::new_native_writer_from_unbuffered (writer, buffer, descriptor));
	succeed! (port.into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_reader_open (path : &Value, buffer : Option<usize>) -> (Outcome<Value>) {
	let mut options = fs::OpenOptions::new ();
	options.read (true);
	return port_file_reader_open_with_options (path, &options, buffer);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_open (path : &Value, buffer : Option<usize>) -> (Outcome<Value>) {
	let mut options = fs::OpenOptions::new ();
	options.write (true);
	options.create (true);
	options.truncate (true);
	// FIXME:  A safer default would be to make sure we are creating the file!
	// options.create_new (true);
	return port_file_writer_open_with_options (path, &options, buffer);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_reader_open_with_options (path : &Value, options : &fs::OpenOptions, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = try! (port_file_open_with_options (path, options));
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_reader_new (file, buffer, descriptor);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_writer_open_with_options (path : &Value, options : &fs::OpenOptions, buffer : Option<usize>) -> (Outcome<Value>) {
	let file = try! (port_file_open_with_options (path, options));
	let file = StdBox::new (file);
	let descriptor = PortDescriptor::for_file (&file);
	return port_native_writer_new (file, buffer, descriptor);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub(crate) fn port_file_open_with_options (path : &Value, options : &fs::OpenOptions) -> (Outcome<fs::File>) {
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	// FIXME:  Clearly indicate why the operation failed!
	let file = try_or_fail! (options.open (path), 0xbe1989bd);
	succeed! (file);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_exists (path : &Value) -> (Outcome<bool>) {
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	succeed! (path.exists ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_file_delete (path : &Value) -> (Outcome<()>) {
	let path = try_as_string_ref! (path);
	let path = fs_path::Path::new (path.string_as_str ());
	// FIXME:  Clearly indicate why the operation failed!
	succeed_or_fail! (fs::remove_file (path), 0xa1653696);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	return port_output_value_display_0 (port, value, flatten, separator, flush);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0 (port : &mut PortBackendWriter, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	match value.class_match_as_ref () {
		
		ValueClassMatchAsRef::Null => {
			try! (port.char_write_string ("null", true));
		},
		
		ValueClassMatchAsRef::Void => {
			try! (port.char_write_string ("void", true));
		},
		
		ValueClassMatchAsRef::Undefined => {
			try! (port.char_write_string ("undefined", true));
		},
		
		ValueClassMatchAsRef::Singleton (value) => {
			let formatted = match value {
				ValueSingleton::Null =>
					"null",
				ValueSingleton::Void =>
					"void",
				ValueSingleton::Undefined =>
					"undefined",
				ValueSingleton::PortEof =>
					"end-of-file",
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Boolean (value) => {
			let value = value.value ();
			let formatted = if value {
				"true"
			} else {
				"false"
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Number (class) => {
			let formatted = match class {
				NumberMatchAsRef::Integer (value) =>
					format! ("{}", value.value ()),
				NumberMatchAsRef::Real (value) =>
					format! ("{}", value.value ()),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Character (value) => {
			let value = value.value ();
			try! (port.char_write (value));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			let string = value.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		ValueClassMatchAsRef::String (class) => {
			let string = class.string_ref ();
			let string = string.string_as_str ();
			try! (port.char_write_string (string, true));
		},
		
		ValueClassMatchAsRef::Bytes (class) => {
			let bytes = class.bytes_ref ();
			let bytes = bytes.bytes_as_slice ();
			try! (port.byte_write_slice (bytes, true));
		},
		
		ValueClassMatchAsRef::Pair (_) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let mut iterator = try! (ListIterator::new (value, true));
				try! (port_output_value_display_0_iterable (port, &mut iterator, Some (true), separator, Some (false)));
				if let Some (dotted) = iterator.dotted () {
					let dotted = dotted.as_ref ();
					try! (port_output_value_display_0 (port, dotted, Some (true), separator, Some (false)));
				}
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let array = class.array_ref ();
				let values = array.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Values (values) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLATTEN) {
				let values = values.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				return port_output_value_write_0 (port, value, Some (false), separator, flush);
			}
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::Error (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) |
		ValueClassMatchAsRef::Opaque (_) => {
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
	}
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_slice (port : &mut PortBackendWriter, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR);
	let mut first = true;
	
	for value in values {
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_display_0_iterable <'a, Iterator> (port : &mut PortBackendWriter, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>
{
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_DISPLAY_SEPARATOR);
	let mut first = true;
	
	for value in values {
		let value = try! (value);
		let value = value.as_ref ();
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_display_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write (port : &Value, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	return port_output_value_write_0 (port, value, flatten, separator, flush);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0 (port : &mut PortBackendWriter, value : &Value, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	match value.class_match_as_ref () {
		
		ValueClassMatchAsRef::Null => {
			try! (port.char_write_string ("#null", true));
		},
		
		ValueClassMatchAsRef::Void => {
			try! (port.char_write_string ("#void", true));
		},
		
		ValueClassMatchAsRef::Undefined => {
			try! (port.char_write_string ("#undefined", true));
		},
		
		ValueClassMatchAsRef::Singleton (value) => {
			let formatted = match value {
				ValueSingleton::Null =>
					"#null",
				ValueSingleton::Void =>
					"#void",
				ValueSingleton::Undefined =>
					"#undefined",
				ValueSingleton::PortEof =>
					"#end-of-file",
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Boolean (value) => {
			let value = value.value ();
			let formatted = if value {
				"#t"
			} else {
				"#f"
			};
			try! (port.char_write_string (formatted, true));
		},
		
		ValueClassMatchAsRef::Number (class) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				NumberMatchAsRef::Integer (value) =>
					format! ("{}", value),
				NumberMatchAsRef::Real (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Character (value) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Symbol (value) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::String (class) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				StringMatchAsRef::Immutable (value) =>
					format! ("{}", value),
				StringMatchAsRef::Mutable (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Bytes (class) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = match class {
				BytesMatchAsRef::Immutable (value) =>
					format! ("{}", value),
				BytesMatchAsRef::Mutable (value) =>
					format! ("{}", value),
			};
			try! (port.char_write_string (&formatted, true));
		},
		
		ValueClassMatchAsRef::Pair (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let mut iterator = try! (ListIterator::new (value, true));
				try! (port_output_value_write_0_iterable (port, &mut iterator, Some (true), separator, Some (false)));
				if let Some (dotted) = iterator.dotted () {
					let dotted = dotted.as_ref ();
					try! (port_output_value_write_0 (port, dotted, Some (true), separator, Some (false)));
				}
			} else {
				// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = match class {
					PairMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					PairMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Array (class) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let array = class.array_ref ();
				let values = array.values_as_slice ();
				try! (port_output_value_write_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = match class {
					ArrayMatchAsRef::Immutable (value) =>
						format! ("{}", value),
					ArrayMatchAsRef::Mutable (value) =>
						format! ("{}", value),
				};
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Values (value) => {
			if flatten.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLATTEN) {
				let values = value.values_as_slice ();
				try! (port_output_value_display_0_slice (port, values, Some (true), separator, Some (false)));
			} else {
				// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
				let formatted = format! ("{}", value);
				try! (port.char_write_string (&formatted, true));
			}
		},
		
		ValueClassMatchAsRef::Procedure (_) |
		ValueClassMatchAsRef::Syntax (_) |
		ValueClassMatchAsRef::Error (_) |
		ValueClassMatchAsRef::Port (_) |
		ValueClassMatchAsRef::Resource (_) |
		ValueClassMatchAsRef::Internal (_) |
		ValueClassMatchAsRef::Opaque (_) => {
			// FIXME:  Implement this efficiently without delegating to `fmt::Display` and without allocating an extra buffer!
			let formatted = format! ("{}", value);
			try! (port.char_write_string (&formatted, true));
		},
		
	}
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_slice (port : &mut PortBackendWriter, values : &[Value], flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR);
	let mut first = true;
	
	for value in values {
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_value_write_0_iterable <'a, Iterator> (port : &mut PortBackendWriter, values : &mut Iterator, flatten : Option<bool>, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>)
		where Iterator : iter::Iterator<Item = Outcome<ValueRef<'a>>>
{
	
	let separator_actual = separator.unwrap_or (DEFAULT_PORT_OUTPUT_VALUE_WRITE_SEPARATOR);
	let mut first = true;
	
	for value in values {
		let value = try! (value);
		let value = value.as_ref ();
		if ! first {
			try! (port.char_write (separator_actual));
		} else {
			first = false;
		}
		try! (port_output_value_write_0 (port, value, flatten, separator, Some (false)));
	}
	
	if flush.unwrap_or (false) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline (port : &Value, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	let port = try_as_port_ref! (port);
	let mut port = try! (port.backend_ref_mut_check_open ());
	let port = port.deref_mut ();
	return port_output_newline_0 (port, separator, flush);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_output_newline_0 (port : &mut PortBackendWriter, separator : Option<char>, flush : Option<bool>) -> (Outcome<()>) {
	
	let separator = separator.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_SEPARATOR);
	
	try! (port.char_write (separator));
	
	if flush.unwrap_or (DEFAULT_PORT_OUTPUT_NEWLINE_FLUSH) {
		try! (port.output_flush ());
	}
	
	succeed! (());
}

