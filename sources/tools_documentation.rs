

use super::documentation::exports::*;
use super::documentation::exports::documentation_model::*;
use super::errors::exports::*;
use super::tools::exports::*;

use super::externals::serde_json as json;
use super::externals::cpio::newc as cpio;

use super::prelude::*;

use super::documentation::exports::documentation_model::Entity;
use super::values::exports::Value as SchemeValue;
use super::values::exports::ValueKind as SchemeValueKind;




pub mod exports {
	
	pub use super::main;
	
	pub use super::{
			dump_json,
			dump_json_0,
		};
	
	pub use super::{
			dump_html,
			dump_html_0,
			dump_html_cpio,
			dump_html_cpio_0,
		};
	
	pub use super::{
			dump_cmark,
			dump_cmark_0,
			dump_cmark_cpio,
			dump_cmark_cpio_0,
		};
	
	pub use super::{
			DumpCmarkCallbacks,
			DumpCpioWriter,
		};
	
	pub use super::{
			dump_cmark_configure,
			DumpCmarkLibrariesConfiguration,
			DumpCmarkLibraryConfiguration,
			DumpCmarkCategoriesConfiguration,
			DumpCmarkCategoryConfiguration,
			DumpCmarkExportsConfiguration,
			DumpCmarkExportConfiguration,
			DumpCmarkDefinitionsConfiguration,
			DumpCmarkDefinitionConfiguration,
			DumpCmarkValueKindsConfiguration,
			DumpCmarkValueKindConfiguration,
			DumpCmarkAppendicesConfiguration,
			DumpCmarkAppendixConfiguration,
			DumpCmarkHierarchyConfiguration,
			DumpCmarkLinkedCategoriesConfiguration,
			DumpCmarkLinkedExportsConfiguration,
			DumpCmarkLinkedDefinitionsConfiguration,
			DumpCmarkLinkedValueKindsConfiguration,
			DumpCmarkGenericConfiguration,
		};
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
pub fn main (inputs : ToolInputs) -> (Outcome<u32>) {
	
	let stream = io::stdout ();
	let mut stream = stream.lock ();
	
	if ! inputs.tool_arguments.is_empty () {
		fail! (0x2f6cb42b);
	}
	
	let tool_commands = vec_map! (inputs.tool_commands.iter (), command, command.as_str ());
	let tool_commands = tool_commands.as_slice ();
	let dump_function = match tool_commands {
		["dump-html"] =>
			dump_html,
		["dump-html-cpio"] =>
			dump_html_cpio,
		["dump-cmark"] =>
			dump_cmark,
		["dump-cmark-cpio"] =>
			dump_cmark_cpio,
		["dump-json"] =>
			dump_json,
		_ =>
			fail! (0x3b57eb47),
	};
	
	let source = match inputs.rest_arguments.as_slice () {
		[] =>
			None,
		[ref source] =>
			Some (fs_path::Path::new (source)),
		_ =>
			fail! (0xf4938c6d),
	};
	
	let libraries = if let Some (source_path) = source {
		let mut source_buffer = StdString::new ();
		let mut source_stream = try_or_fail! (fs::File::open (source_path), 0x463c39f9);
		try_or_fail! (source_stream.read_to_string (&mut source_buffer), 0x4025f07b);
		try! (parse_library_specifications (&source_buffer))
	} else {
		#[ cfg ( not ( feature = "vonuvoli_documentation_sources" ) ) ]
		fail! (0x834807b9);
		#[ cfg ( feature = "vonuvoli_documentation_sources" ) ]
		try! (parse_library_specifications_for_builtins ())
	};
	
	try! (dump_function (&libraries, &mut stream));
	
	succeed! (0);
}




const BUFFER_SIZE_LARGE : usize = 8 * 1024 * 1024;
const BUFFER_SIZE_SMALL : usize = 128 * 1024;




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	
	let mut buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
	
	try! (dump_json_0 (libraries, &mut buffer));
	
	try_or_fail! (stream.write_all (&buffer), 0xa1639c14);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_json_0 (libraries : &Libraries, stream : &mut impl io::Write) -> (Outcome<()>) {
	
	let libraries_json = json::Map::from_iter (vec_map! (libraries.libraries (), library, (library.identifier_clone (), dump_json_library (library))));
	
	try_or_fail! (json::to_writer_pretty (stream, &libraries_json), 0x200f6e78);
	
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_library (library : &Library) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			"features" : if let Some (features) = library.features () { dump_json_features (features) } else { json::Value::Null },
			
			"categories" : json::Map::from_iter (vec_map! (library.categories (), category, (category.identifier_clone (), dump_json_category (category)))),
			
			"exports" : json::Map::from_iter (vec_map! (library.exports (), export, (export.identifier_clone (), dump_json_export (export)))),
			
			"types" : json::Map::from_iter (vec_map! (library.value_kinds (), value_kind, (value_kind.identifier_clone (), dump_json_value_kind (value_kind)))),
			"types_all" : json::Map::from_iter (vec_map! (library.value_kinds_all (), (alias, value_kind), (StdString::from (alias), json::Value::String (value_kind.identifier_clone ())))),
			
			"definitions" : json::Map::from_iter (vec_map! (library.definitions (), definition, (definition.identifier_clone (), dump_json_definition (definition)))),
			"definitions_all" : json::Map::from_iter (vec_map! (library.definitions_all (), (alias, definition), (StdString::from (alias), json::Value::String (definition.identifier_clone ())))),
			
			"title" : if let Some (title) = library.title () { json::Value::String (StdString::from (title)) } else { json::Value::Null },
			"description" : if let Some (description) = library.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = library.links () { dump_json_links (links) } else { json::Value::Null },
			
			"appendices" : json::Map::from_iter (vec_map! (library.appendices (), appendix, (appendix.identifier_clone (), dump_json_appendix (appendix)))),
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_category (category : &Category) -> (json::Value) {
	json! ({
			
			"identifier" : category.identifier_clone (),
			
			"super_categories" : dump_json_identifiers_perhaps_for_entities (category.parents ()),
			"super_categories_recursive" : dump_json_identifiers_perhaps_for_entities (category.parents_recursive ()),
			"sub_categories" : dump_json_identifiers_perhaps_for_entities (category.children ()),
			"sub_categories_recursive" : dump_json_identifiers_perhaps_for_entities (category.children_recursive ()),
			
			"exports" : dump_json_identifiers_perhaps_for_entities (category.exports ()),
			"exports_recursive" : dump_json_identifiers_perhaps_for_entities (category.exports_recursive ()),
			
			"types" : dump_json_identifiers_perhaps_for_entities (category.value_kinds ()),
			"types_recursive" : dump_json_identifiers_perhaps_for_entities (category.value_kinds_recursive ()),
			
			"definitions" : dump_json_identifiers_perhaps_for_entities (category.definitions ()),
			"definitions_recursive" : dump_json_identifiers_perhaps_for_entities (category.definitions_recursive ()),
			
			"description" : if let Some (description) = category.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = category.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_export (export : &Export) -> (json::Value) {
	json! ({
			
			"identifier" : export.identifier_clone (),
			"features" : if let Some (features) = export.features () { dump_json_features (features) } else { json::Value::Null },
			
			"super_exports" : dump_json_identifiers_perhaps_for_entities (export.parents ()),
			"super_exports_recursive" : dump_json_identifiers_perhaps_for_entities (export.parents_recursive ()),
			"sub_exports" : dump_json_identifiers_perhaps_for_entities (export.children ()),
			"sub_exports_recursive" : dump_json_identifiers_perhaps_for_entities (export.children_recursive ()),
			
			"categories" : dump_json_identifiers_perhaps_for_entities (export.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_entities (export.categories_recursive ()),
			
			"description" : if let Some (description) = export.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = export.links () { dump_json_links (links) } else { json::Value::Null },
			
			"definitions" : dump_json_identifiers_perhaps_for_entities (export.definitions ()),
			"definitions_recursive" : dump_json_identifiers_perhaps_for_entities (export.definitions_recursive ()),
			
			"descriptor" : dump_json_value (&export.descriptor_format ()),
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_value_kind (value_kind : &ValueKind) -> (json::Value) {
	json! ({
			
			"identifier" : value_kind.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (value_kind.aliases ()),
			"features" : if let Some (features) = value_kind.features () { dump_json_features (features) } else { json::Value::Null },
			
			"super_types" : dump_json_identifiers_perhaps_for_entities (value_kind.parents ()),
			"super_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.parents_recursive ()),
			"sub_types" : dump_json_identifiers_perhaps_for_entities (value_kind.children ()),
			"sub_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.children_recursive ()),
			
			"categories" : dump_json_identifiers_perhaps_for_entities (value_kind.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.categories_recursive ()),
			
			"covariant_types" : dump_json_identifiers_perhaps_for_entities (value_kind.covariants ()),
			"covariant_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.covariants_recursive ()),
			"contravariant_types" : dump_json_identifiers_perhaps_for_entities (value_kind.contravariants ()),
			"contravariant_types_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.contravariants_recursive ()),
			
			"definitions_input" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input ()),
			"definitions_input_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input_recursive ()),
			"definitions_input_contravariant" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_input_contravariant_recursive ()),
			"definitions_output" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output ()),
			"definitions_output_recursive" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output_recursive ()),
			"definitions_output_covariant" : dump_json_identifiers_perhaps_for_entities (value_kind.definitions_output_covariant_recursive ()),
			
			"description" : if let Some (description) = value_kind.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = value_kind.links () { dump_json_links (links) } else { json::Value::Null },
			
			"predicate" : if let Some (predicate) = value_kind.predicate () { dump_json_value (&predicate.format ()) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_definition (definition : &Definition) -> (json::Value) {
	json! ({
			
			"identifier" : definition.identifier_clone (),
			"aliases" : dump_json_identifiers_perhaps (definition.aliases ()),
			"features" : if let Some (features) = definition.features () { dump_json_features (features) } else { json::Value::Null },
			
			"categories" : dump_json_identifiers_perhaps_for_entities (definition.categories ()),
			"categories_recursive" : dump_json_identifiers_perhaps_for_entities (definition.categories_recursive ()),
			
			"exports" : dump_json_identifiers_perhaps_for_entities (definition.exports ()),
			"exports_recursive" : dump_json_identifiers_perhaps_for_entities (definition.exports_recursive ()),
			
			"kind" : json::Value::String (StdString::from (definition.kind () .identifier ())),
			
			"description" : if let Some (description) = definition.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = definition.links () { dump_json_links (links) } else { json::Value::Null },
			
			"procedure_signature" : if let Some (procedure_signature) = definition.procedure_signature () { dump_json_procedure_signature (procedure_signature) } else { json::Value::Null },
			"syntax_signature" : if let Some (syntax_signature) = definition.syntax_signature () { dump_json_syntax_signature (syntax_signature) } else { json::Value::Null },
			
			"referenced_types" : dump_json_identifiers_perhaps_for_entities (definition.referenced_value_kinds ()),
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature (signature : &ProcedureSignature) -> (json::Value) {
	json! ({
			"variants" : json::Value::Array (vec_map! (signature.variants.iter (), variant, dump_json_procedure_signature_variant (variant))),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_variant (variant : &ProcedureSignatureVariant) -> (json::Value) {
	json! ({
			"inputs" : dump_json_procedure_signature_values (&variant.inputs),
			"outputs" : dump_json_procedure_signature_values (&variant.outputs),
			"features" : if let Some (features) = &variant.features { dump_json_features (features) } else { json::Value::Null },
			"source" : dump_json_value (& variant.format ()),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_values (values : &ProcedureSignatureValues) -> (json::Value) {
	json! ({
			"values" : json::Value::Array (vec_map! (values.values.iter (), value, dump_json_procedure_signature_value (value))),
			"variadic" : values.variadic,
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_procedure_signature_value (value : &ProcedureSignatureValue) -> (json::Value) {
	json! ({
			"identifier" : if let Some (identifier) = &value.identifier { json::Value::String (StdString::from (identifier.deref () .deref ())) } else { json::Value::Null },
			"type" : value.kind.identifier_clone (),
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature (signature : &SyntaxSignature) -> (json::Value) {
	json! ({
			"keywords" : json::Map::from_iter (vec_map! (signature.keywords.iter (), keyword, (keyword.identifier_clone (), dump_json_syntax_signature_keyword (keyword)))),
			"variants" : json::Value::Array (vec_map! (signature.variants.iter (), variant, dump_json_syntax_signature_variant (variant))),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_keyword (keyword : &SyntaxSignatureKeyword) -> (json::Value) {
	let identifier = keyword.identifier_clone ();
	match keyword {
		SyntaxSignatureKeyword::Literal (_) =>
			json! ({
					"kind" : "literal",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Identifier (_) =>
			json! ({
					"kind" : "identifier",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Expression (_) =>
			json! ({
					"kind" : "expression",
					"identifier" : identifier,
				}),
		SyntaxSignatureKeyword::Constant { value, .. } =>
			json! ({
					"kind" : "constant",
					"identifier" : identifier,
					"value" : format! ("{}", value),
				}),
		SyntaxSignatureKeyword::Value { kind, .. } =>
			json! ({
					"kind" : "value",
					"identifier" : identifier,
					"type" : dump_json_identifier_perhaps_for_entity (kind.as_ref ()),
				}),
		SyntaxSignatureKeyword::Pattern { patterns, .. } =>
			json! ({
					"kind" : "pattern",
					"identifier" : identifier,
					"patterns" : json::Value::Array (vec_map! (patterns.iter (), pattern,
							json! ({
								"pattern" : dump_json_syntax_signature_pattern (pattern),
								"source" : dump_json_value (& pattern.format ()),
							}))),
				}),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_variant (variant : &SyntaxSignatureVariant) -> (json::Value) {
	json! ({
			"pattern" : dump_json_syntax_signature_pattern (&variant.pattern),
			"source" : dump_json_value (& variant.pattern.format ()),
		})
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_syntax_signature_pattern (pattern : &SyntaxSignaturePattern) -> (json::Value) {
	match pattern {
		SyntaxSignaturePattern::List (patterns, pattern_dotted) =>
			if let Some (pattern_dotted) = pattern_dotted {
				json! ([
						".",
						json::Value::Array (vec_map! (patterns.iter (), pattern, dump_json_syntax_signature_pattern (pattern))),
						dump_json_syntax_signature_pattern (pattern_dotted),
					])
			} else {
				json::Value::Array (vec_map! (patterns.iter (), pattern, dump_json_syntax_signature_pattern (pattern)))
			},
		SyntaxSignaturePattern::Keyword (keyword) =>
			json::Value::String (keyword.identifier_clone ()),
		SyntaxSignaturePattern::Variadic (pattern) =>
			json! (["...", dump_json_syntax_signature_pattern (pattern)]),
		SyntaxSignaturePattern::SyntaxIdentifier =>
			json::Value::String (StdString::from ("_")),
		SyntaxSignaturePattern::SyntaxRules =>
			json::Value::String (StdString::from ("@syntax-rules")),
		SyntaxSignaturePattern::SyntaxTransformer =>
			json::Value::String (StdString::from ("@syntax-transformer")),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_features (features : &Features) -> (json::Value) {
	return dump_json_value (&features.condition);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_appendix (library : &Appendix) -> (json::Value) {
	json! ({
			
			"identifier" : library.identifier_clone (),
			
			"title" : if let Some (title) = library.title () { json::Value::String (StdString::from (title)) } else { json::Value::Null },
			"description" : if let Some (description) = library.description () { dump_json_description (description) } else { json::Value::Null },
			"links" : if let Some (links) = library.links () { dump_json_links (links) } else { json::Value::Null },
			
		})
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifier_perhaps_for_entity (entity : Option<&impl Entity>) -> (json::Value) {
	return dump_json_identifier_perhaps (entity.map (Entity::identifier));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifier_perhaps (identifier : Option<&str>) -> (json::Value) {
	if let Some (identifier) = identifier {
		return json::Value::String (StdString::from (identifier));
	} else {
		return json::Value::Null;
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifiers_perhaps_for_entities <'a, E : Entity + 'a> (entities : impl iter::Iterator<Item = &'a E>) -> (json::Value) {
	return dump_json_identifiers_perhaps (entities.map (Entity::identifier));
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_identifiers_perhaps <'a> (identifiers : impl iter::Iterator<Item = &'a str>) -> (json::Value) {
	let identifiers = vec_map! (identifiers, identifier, json::Value::String (StdString::from (identifier)));
	if identifiers.is_empty () {
		return json::Value::Null;
	} else {
		return json::Value::Array (identifiers);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_description (description : &Description) -> (json::Value) {
	json::Value::Array (vec_map_into! (description.lines_clone (), line, json::Value::String (line)))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_links (_links : &Links) -> (json::Value) {
	unimplemented_0! (0xb7740aad);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_json_value (value : &SchemeValue) -> (json::Value) {
	match value.kind () {
		SchemeValueKind::Null =>
			json::Value::String (StdString::from ("()")),
		_ => {
			FIXME! ("better handle `#null` case");
			let buffer = format! ("{}", value);
			let buffer = buffer.replace ("#null", "()");
			json::Value::String (buffer)
		}
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (true, true));
	let stream_buffer = {
		let mut stream_buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
		try! (dump_html_header_write ("Scheme Libraries", &mut stream_buffer));
		let mut callbacks = DumpCmarkCallbacksSingleFile {
				buffer : stream_buffer,
			};
		try! (dump_html_0 (libraries, &configuration, &mut callbacks));
		let mut stream_buffer = callbacks.buffer;
		try! (dump_html_trailer_write (&mut stream_buffer));
		stream_buffer
	};
	try_or_fail! (stream.write_all (&stream_buffer), 0x4aed615a);
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (true, false));
	let stream_buffer = {
		let mut stream_buffer = StdVec::with_capacity (BUFFER_SIZE_LARGE);
		let mut callbacks = DumpCmarkCallbacksSingleFile {
				buffer : stream_buffer,
			};
		try! (dump_cmark_0 (libraries, &configuration, &mut callbacks));
		callbacks.buffer
	};
	try_or_fail! (stream.write_all (&stream_buffer), 0x27d8ded6);
	succeed! (());
}


struct DumpCmarkCallbacksSingleFile {
	buffer : StdVec<u8>,
}

impl DumpCmarkCallbacks for DumpCmarkCallbacksSingleFile {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return StdVec::with_capacity (BUFFER_SIZE_SMALL);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write (&mut self, _kind : Option<&str>, _library : Option<&str>, _entity : Option<&str>, buffer : StdVec<u8>) -> (Outcome<()>) {
		try_or_fail! (self.buffer.write_all (&buffer), 0x67f5c369);
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, _source : &str) -> (Outcome<StdString>) {
		let mut anchor_full = StdString::new ();
		let anchor_hash = try! (dump_cmark_anchor_generate (kind, library, entity));
		anchor_full.push ('#');
		anchor_full.push_str (&anchor_hash);
		succeed! (anchor_full);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_anchor_write (kind, library, entity, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_title_write (kind, library, entity, title, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write (&mut self, library : &Library, configuration : &DumpCmarkGenericConfiguration, anchor_source : &str, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_break_write (library, configuration, self, anchor_source, buffer);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_cpio (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (false, true));
	let mut writer = try! (DumpCpioWriter::open (stream));
	try! (dump_html_cpio_0 (libraries, &configuration, &mut writer));
	try! (writer.close ());
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_cpio (libraries : &Libraries, stream : &mut dyn io::Write) -> (Outcome<()>) {
	let configuration = try! (dump_cmark_configure (false, false));
	let mut writer = try! (DumpCpioWriter::open (stream));
	try! (dump_cmark_cpio_0 (libraries, &configuration, &mut writer));
	try! (writer.close ());
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_cpio_0 <'a, Writer : io::Write + 'a> (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, writer : &'a mut DumpCpioWriter<Writer>) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksCpioFile {
			writer : writer,
			path_prefix : "./",
			path_suffix : ".html",
		};
	return dump_html_0 (libraries, configuration, &mut callbacks);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_cpio_0 <'a, Writer : io::Write + 'a> (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, writer : &'a mut DumpCpioWriter<Writer>) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksCpioFile {
			writer : writer,
			path_prefix : "./",
			path_suffix : ".md",
		};
	return dump_cmark_0 (libraries, configuration, &mut callbacks);
}


struct DumpCmarkCallbacksCpioFile <'a, Writer : io::Write + 'a> {
	writer : &'a mut DumpCpioWriter<Writer>,
	path_prefix : &'a str,
	path_suffix : &'a str,
}

impl <'a, Writer : io::Write> DumpCmarkCallbacks for DumpCmarkCallbacksCpioFile<'a, Writer> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return StdVec::with_capacity (BUFFER_SIZE_SMALL);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, buffer : StdVec<u8>) -> (Outcome<()>) {
		let path = try! (dump_cmark_path_generate (kind, library, entity, self.path_prefix, self.path_suffix));
		let path = fs_path::PathBuf::from (path);
		try! (self.writer.write (&path, &buffer));
		succeed! (());
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, source : &str) -> (Outcome<StdString>) {
		let mut anchor_full = StdString::new ();
		let anchor_path_prefix = match source {
			"library" =>
				"../",
			"category" | "categories" |
			"export" | "exports" |
			"definition" | "definitions" |
			"value_kind" | "value_kinds" |
			"appendix" | "appendices" =>
				"../../",
			_ =>
				fail! (0x8fa488c7),
		};
		let anchor_path = try! (dump_cmark_path_generate (kind, library, entity, anchor_path_prefix, self.path_suffix));
		let anchor_hash = try! (dump_cmark_anchor_generate (kind, library, entity));
		anchor_full.push_str (&anchor_path);
		anchor_full.push ('#');
		anchor_full.push_str (&anchor_hash);
		succeed! (anchor_full);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_anchor_write (kind, library, entity, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_title_write (kind, library, entity, title, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write (&mut self, library : &Library, configuration : &DumpCmarkGenericConfiguration, anchor_source : &str, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return dump_cmark_break_write (library, configuration, self, anchor_source, buffer);
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_html_0 (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	let mut callbacks = DumpCmarkCallbacksToHtml {
			callbacks : callbacks,
			embedded : configuration.generic.embedded,
		};
	return dump_cmark_execute (libraries, configuration, &mut callbacks);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_0 (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	return dump_cmark_execute (libraries, configuration, callbacks);
}


struct DumpCmarkCallbacksToHtml <'a, Callbacks : DumpCmarkCallbacks + 'a> {
	callbacks : &'a mut Callbacks,
	embedded : bool,
}

impl <'a, Callbacks : DumpCmarkCallbacks + 'a> DumpCmarkCallbacks for DumpCmarkCallbacksToHtml<'a, Callbacks> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_build (&mut self) -> (StdVec<u8>) {
		return self.callbacks.buffer_build ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn buffer_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, cmark_buffer : StdVec<u8>) -> (Outcome<()>) {
		
		let cmark_buffer = try_or_fail! (StdString::from_utf8 (cmark_buffer), 0xb06a2a9f);
		
		let mut html_buffer = self.callbacks.buffer_build ();
		
		if !self.embedded {
			let title = try! (dump_cmark_title_generate (kind, library, entity, None));
			try! (dump_html_header_write (&title, &mut html_buffer));
		}
		
		let mut html_buffer = try_or_fail! (StdString::from_utf8 (html_buffer), 0x20a8754d);
		
		let parser = ext::pulldown_cmark::Parser::new (&cmark_buffer);
		ext::pulldown_cmark::html::push_html (&mut html_buffer, parser);
		
		let mut html_buffer = StdVec::from (html_buffer);
		
		if !self.embedded {
			try! (dump_html_trailer_write (&mut html_buffer));
		}
		
		return self.callbacks.buffer_write (kind, library, entity, html_buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_generate (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, source : &str) -> (Outcome<StdString>) {
		return self.callbacks.anchor_generate (kind, library, entity, source);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn anchor_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.anchor_write (kind, library, entity, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn title_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.title_write (kind, library, entity, title, configuration, buffer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn break_write (&mut self, library : &Library, configuration : &DumpCmarkGenericConfiguration, anchor_source : &str, buffer : &mut StdVec<u8>) -> (Outcome<()>) {
		return self.callbacks.break_write (library, configuration, anchor_source, buffer);
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_html_header_write (title : &str, stream : &mut impl io::Write) -> (Outcome<()>) {
	let title = title.replace ("<", "&lt;");
	let title = title.replace (">", "&gt;");
	let title = title.replace ("&", "&amp;");
	let title = title.replace ("\"", "&quot;");
	let prefix = DUMP_HTML_PREFIX.replace ("@{title}", &title);
	try_or_fail! (stream.write_all (prefix.as_bytes ()), 0xd730a725);
	try_or_fail! (stream.write_all (b"<style type='text/css'>\n"), 0x64138904);
	try_or_fail! (stream.write_all (DUMP_HTML_CSS.as_bytes ()), 0xe8153313);
	try_or_fail! (stream.write_all (b"</style>\n"), 0x108d0302);
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_html_trailer_write (stream : &mut impl io::Write) -> (Outcome<()>) {
	try_or_fail! (stream.write_all (DUMP_HTML_SUFFIX.as_bytes ()), 0x17a2e8ae);
	succeed! (());
}




#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLibrariesConfiguration {
	pub enabled : bool,
	pub configuration : DumpCmarkLibraryConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLibraryConfiguration {
	pub toc : bool,
	pub toc_compact : bool,
	pub features : bool,
	pub description : bool,
	pub links : bool,
	pub categories : DumpCmarkCategoriesConfiguration,
	pub exports : DumpCmarkExportsConfiguration,
	pub definitions : DumpCmarkDefinitionsConfiguration,
	pub value_kinds : DumpCmarkValueKindsConfiguration,
	pub appendices : DumpCmarkAppendicesConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkCategoriesConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkCategoryConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkCategoryConfiguration {
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	pub exports : DumpCmarkLinkedExportsConfiguration,
	pub definitions : DumpCmarkLinkedDefinitionsConfiguration,
	pub value_kinds : DumpCmarkLinkedValueKindsConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkExportsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkExportConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkExportConfiguration {
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	pub definitions : DumpCmarkLinkedDefinitionsConfiguration,
	pub descriptor : bool,
	pub features : bool,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkDefinitionsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub configuration : DumpCmarkDefinitionConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkDefinitionConfiguration {
	pub kind : bool,
	pub exports : DumpCmarkLinkedExportsConfiguration,
	pub signature : bool,
	pub aliases : bool,
	pub aliases_compact : bool,
	pub features : bool,
	pub value_kinds : DumpCmarkLinkedValueKindsConfiguration,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkValueKindsConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub toc_complete : bool,
	pub toc_depth : usize,
	pub configuration : DumpCmarkValueKindConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkValueKindConfiguration {
	pub tree : bool,
	pub tree_complete : bool,
	pub tree_depth : usize,
	pub hierarchy : DumpCmarkHierarchyConfiguration,
	pub covariants : DumpCmarkLinkedValueKindsConfiguration,
	pub contravariants : DumpCmarkLinkedValueKindsConfiguration,
	pub definitions_all : DumpCmarkLinkedDefinitionsConfiguration,
	pub definitions_all_variant : bool,
	pub definitions_all_variant_complete : bool,
	pub definitions_all_variant_compact : bool,
	pub definitions_input : DumpCmarkLinkedDefinitionsConfiguration,
	pub definitions_input_contravariant : bool,
	pub definitions_input_contravariant_complete : bool,
	pub definitions_input_contravariant_compact : bool,
	pub definitions_output : DumpCmarkLinkedDefinitionsConfiguration,
	pub definitions_output_covariant : bool,
	pub definitions_output_covariant_complete : bool,
	pub definitions_output_covariant_compact : bool,
	pub predicate : bool,
	pub aliases : bool,
	pub aliases_compact : bool,
	pub features : bool,
	pub categories : DumpCmarkLinkedCategoriesConfiguration,
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkAppendicesConfiguration {
	pub enabled : bool,
	pub toc : bool,
	pub configuration : DumpCmarkAppendixConfiguration,
	pub generic : DumpCmarkGenericConfiguration,
}

#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkAppendixConfiguration {
	pub description : bool,
	pub links : bool,
	pub generic : DumpCmarkGenericConfiguration,
}




#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkGenericConfiguration {
	pub notes : bool,
	pub fixme : bool,
	pub lints : bool,
	pub navigator : bool,
	pub navigator_categories : bool,
	pub navigator_exports : bool,
	pub navigator_definitions : bool,
	pub navigator_value_kinds : bool,
	pub navigator_appendices : bool,
	pub navigator_library : bool,
	pub navigator_libraries : bool,
	pub anchors : bool,
	pub embedded : bool,
	pub html : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkHierarchyConfiguration {
	pub super_direct : bool,
	pub super_direct_complete : bool,
	pub super_direct_compact : bool,
	pub super_recursive : bool,
	pub super_recursive_complete : bool,
	pub super_recursive_compact : bool,
	pub sub_direct : bool,
	pub sub_direct_complete : bool,
	pub sub_direct_compact : bool,
	pub sub_recursive : bool,
	pub sub_recursive_complete : bool,
	pub sub_recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedCategoriesConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedExportsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedDefinitionsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}


#[ derive ( Copy, Clone ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub struct DumpCmarkLinkedValueKindsConfiguration {
	pub direct : bool,
	pub direct_complete : bool,
	pub direct_compact : bool,
	pub recursive : bool,
	pub recursive_complete : bool,
	pub recursive_compact : bool,
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn dump_cmark_configure (embedded : bool, html : bool) -> (Outcome<DumpCmarkLibrariesConfiguration>) {
	
	
	const ALL : bool = false;
	const DEBUG : bool = false;
	
	const NO_SUPER : bool = false;
	const NO_SUB : bool = false;
	const NO_TREE : bool = false;
	const NO_LIBRARIES : bool = false;
	const NO_CATEGORIES : bool = false;
	const NO_EXPORTS : bool = false;
	const NO_VALUE_KINDS : bool = false;
	const NO_VARIANTS : bool = true;
	const NO_DEFINITIONS : bool = false;
	const NO_APPENDICES : bool = false;
	const NO_RECURSIVE : bool = false;
	const NO_DETAILS : bool = false;
	const NO_TOC : bool = false;
	const NO_NOTES : bool = false;
	const NO_FIXME : bool = true;
	
	
	const LIBRARIES : bool = ALL || !NO_LIBRARIES;
	const LIBRARIES_TOC : bool = ALL || !NO_TOC;
	
	const CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const CATEGORIES_TOC : bool = ALL || !NO_TOC;
	const CATEGORIES_SUPER : bool = ALL || !NO_SUPER;
	const CATEGORIES_SUPER_RECURSIVE : bool = CATEGORIES_SUPER && (ALL || !NO_RECURSIVE);
	const CATEGORIES_SUB : bool = ALL || !NO_SUB;
	const CATEGORIES_SUB_RECURSIVE : bool = CATEGORIES_SUB && (ALL || !NO_RECURSIVE);
	const CATEGORIES_EXPORTS : bool = ALL || !NO_EXPORTS;
	const CATEGORIES_EXPORTS_RECURSIVE : bool = CATEGORIES_EXPORTS && (ALL || !NO_RECURSIVE);
	const CATEGORIES_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const CATEGORIES_DEFINITIONS_RECURSIVE : bool = CATEGORIES_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const CATEGORIES_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const CATEGORIES_VALUE_KINDS_RECURSIVE : bool = CATEGORIES_VALUE_KINDS && (ALL || !NO_RECURSIVE);
	
	const EXPORTS : bool = ALL || !NO_EXPORTS;
	const EXPORTS_TOC : bool = ALL || !NO_TOC;
	const EXPORTS_DESCRIPTOR : bool = ALL || !NO_DETAILS;
	const EXPORTS_SUPER : bool = ALL || !NO_SUPER;
	const EXPORTS_SUPER_RECURSIVE : bool = EXPORTS_SUPER && (ALL || !NO_RECURSIVE);
	const EXPORTS_SUB : bool = ALL || !NO_SUB;
	const EXPORTS_SUB_RECURSIVE : bool = EXPORTS_SUB && (ALL || !NO_RECURSIVE);
	const EXPORTS_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const EXPORTS_DEFINITIONS_RECURSIVE : bool = EXPORTS_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const EXPORTS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const EXPORTS_CATEGORIES_RECURSIVE : bool = EXPORTS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const DEFINITIONS_TOC : bool = ALL || !NO_TOC;
	const DEFINITIONS_KIND : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_EXPORTS : bool = ALL || !NO_EXPORTS;
	const DEFINITIONS_EXPORTS_RECURSIVE : bool = DEFINITIONS_EXPORTS && (ALL || !NO_RECURSIVE);
	const DEFINITIONS_SIGNATURE : bool = ALL || !NO_DETAILS;
	const DEFINITIONS_VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const DEFINITIONS_VALUE_KINDS_RECURSIVE : bool = DEFINITIONS_SIGNATURE && (ALL || !NO_RECURSIVE);
	const DEFINITIONS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const DEFINITIONS_CATEGORIES_RECURSIVE : bool = DEFINITIONS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const VALUE_KINDS : bool = ALL || !NO_VALUE_KINDS;
	const VALUE_KINDS_TOC : bool = ALL || !NO_TOC;
	const VALUE_KINDS_TREE : bool = ALL || !NO_TREE;
	const VALUE_KINDS_SUPER : bool = ALL || !NO_SUPER;
	const VALUE_KINDS_SUPER_RECURSIVE : bool = VALUE_KINDS_SUPER && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_SUB : bool = ALL || !NO_SUB;
	const VALUE_KINDS_SUB_RECURSIVE : bool = VALUE_KINDS_SUB && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_VARIANTS : bool = ALL || !NO_VARIANTS;
	const VALUE_KINDS_VARIANTS_RECURSIVE : bool = VALUE_KINDS_VARIANTS && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_COVARIANTS : bool = VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_COVARIANTS_RECURSIVE : bool = VALUE_KINDS_COVARIANTS && VALUE_KINDS_VARIANTS_RECURSIVE;
	const VALUE_KINDS_CONTRAVARIANTS : bool = VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_CONTRAVARIANTS_RECURSIVE : bool = VALUE_KINDS_CONTRAVARIANTS && VALUE_KINDS_VARIANTS_RECURSIVE;
	const VALUE_KINDS_DEFINITIONS : bool = ALL || !NO_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_INPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_INPUT && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT : bool = VALUE_KINDS_DEFINITIONS_INPUT && VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_DEFINITIONS_OUTPUT : bool = VALUE_KINDS_DEFINITIONS;
	const VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && (ALL || !NO_RECURSIVE);
	const VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT : bool = VALUE_KINDS_DEFINITIONS_OUTPUT && VALUE_KINDS_VARIANTS;
	const VALUE_KINDS_PREDICATE : bool = ALL || !NO_DETAILS;
	const VALUE_KINDS_CATEGORIES : bool = ALL || !NO_CATEGORIES;
	const VALUE_KINDS_CATEGORIES_RECURSIVE : bool = VALUE_KINDS_CATEGORIES && (ALL || !NO_RECURSIVE);
	
	const APPENDICES : bool = ALL || !NO_APPENDICES;
	const APPENDICES_TOC : bool = ALL || !NO_TOC;
	
	const COMPLETE : bool = false;
	const COMPACT : bool = true;
	const ALIASES : bool = ALL || !NO_DETAILS;
	const FEATURES : bool = ALL || !NO_DETAILS;
	const DESCRIPTIONS : bool = ALL || !NO_DETAILS;
	const LINKS : bool = ALL || !NO_DETAILS;
	
	const NOTES : bool = ALL || !NO_NOTES;
	const FIXME : bool = ALL || !NO_FIXME;
	const LINTS : bool = DEBUG && !NO_FIXME;
	const NAVIGATOR : bool = true;
	const ANCHORS : bool = true;
	
	const TOC_COMPLETE : bool = true;
	const TOC_DEPTH : usize = 2;
	const TREE_COMPLETE : bool = true;
	const TREE_DEPTH : usize = 2;
	
	
	let generic = DumpCmarkGenericConfiguration {
			notes : NOTES,
			fixme : FIXME,
			lints : LINTS,
			navigator : NAVIGATOR,
			navigator_categories : CATEGORIES,
			navigator_exports : EXPORTS,
			navigator_definitions : DEFINITIONS,
			navigator_value_kinds : VALUE_KINDS,
			navigator_appendices : APPENDICES,
			navigator_library : LIBRARIES,
			navigator_libraries : LIBRARIES,
			anchors : ANCHORS,
			embedded : embedded,
			html : html,
		};
	
	
	let category = DumpCmarkCategoryConfiguration {
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : CATEGORIES_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : CATEGORIES_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : CATEGORIES_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : CATEGORIES_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			exports : DumpCmarkLinkedExportsConfiguration {
					direct : CATEGORIES_EXPORTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_EXPORTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions : DumpCmarkLinkedDefinitionsConfiguration {
					direct : CATEGORIES_DEFINITIONS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_DEFINITIONS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			value_kinds : DumpCmarkLinkedValueKindsConfiguration {
					direct : CATEGORIES_VALUE_KINDS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : CATEGORIES_VALUE_KINDS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let categories = DumpCmarkCategoriesConfiguration {
			enabled : CATEGORIES,
			toc : CATEGORIES_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : category,
			generic : generic,
		};
	
	
	let export = DumpCmarkExportConfiguration {
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : EXPORTS_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : EXPORTS_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : EXPORTS_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : EXPORTS_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			definitions : DumpCmarkLinkedDefinitionsConfiguration {
					direct : EXPORTS_DEFINITIONS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : EXPORTS_DEFINITIONS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			descriptor : EXPORTS_DESCRIPTOR,
			features : FEATURES,
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : EXPORTS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : EXPORTS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let exports = DumpCmarkExportsConfiguration {
			enabled : EXPORTS,
			toc : EXPORTS_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : export,
			generic : generic,
		};
	
	
	let definition = DumpCmarkDefinitionConfiguration {
			kind : DEFINITIONS_KIND,
			exports : DumpCmarkLinkedExportsConfiguration {
					direct : DEFINITIONS_EXPORTS,
					direct_compact : COMPACT,
					direct_complete : COMPLETE,
					recursive : DEFINITIONS_EXPORTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			signature : DEFINITIONS_SIGNATURE,
			aliases : ALIASES,
			aliases_compact : COMPACT,
			features : FEATURES,
			value_kinds : DumpCmarkLinkedValueKindsConfiguration {
					direct : DEFINITIONS_VALUE_KINDS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : DEFINITIONS_VALUE_KINDS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : DEFINITIONS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : DEFINITIONS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let definitions = DumpCmarkDefinitionsConfiguration {
			enabled : DEFINITIONS,
			toc : DEFINITIONS_TOC,
			configuration : definition,
			generic : generic,
		};
	
	
	let value_kind = DumpCmarkValueKindConfiguration {
			tree : VALUE_KINDS_TREE,
			tree_complete : TREE_COMPLETE,
			tree_depth : TREE_DEPTH,
			hierarchy : DumpCmarkHierarchyConfiguration {
					super_direct : VALUE_KINDS_SUPER,
					super_direct_complete : COMPLETE,
					super_direct_compact : COMPACT,
					super_recursive : VALUE_KINDS_SUPER_RECURSIVE,
					super_recursive_complete : COMPLETE,
					super_recursive_compact : COMPACT,
					sub_direct : VALUE_KINDS_SUB,
					sub_direct_complete : COMPLETE,
					sub_direct_compact : COMPACT,
					sub_recursive : VALUE_KINDS_SUB_RECURSIVE,
					sub_recursive_complete : COMPLETE,
					sub_recursive_compact : COMPACT,
				},
			covariants : DumpCmarkLinkedValueKindsConfiguration {
					direct : VALUE_KINDS_COVARIANTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_COVARIANTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			contravariants : DumpCmarkLinkedValueKindsConfiguration {
					direct : VALUE_KINDS_CONTRAVARIANTS,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_CONTRAVARIANTS_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_all : DumpCmarkLinkedDefinitionsConfiguration {
					direct : false,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : false,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_all_variant : false,
			definitions_all_variant_complete : COMPLETE,
			definitions_all_variant_compact : COMPACT,
			definitions_input : DumpCmarkLinkedDefinitionsConfiguration {
					direct : VALUE_KINDS_DEFINITIONS_INPUT,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_DEFINITIONS_INPUT_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_input_contravariant : VALUE_KINDS_DEFINITIONS_INPUT_CONTRAVARIANT,
			definitions_input_contravariant_complete : COMPLETE,
			definitions_input_contravariant_compact : COMPACT,
			definitions_output : DumpCmarkLinkedDefinitionsConfiguration {
					direct : VALUE_KINDS_DEFINITIONS_OUTPUT,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_DEFINITIONS_OUTPUT_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			definitions_output_covariant : VALUE_KINDS_DEFINITIONS_OUTPUT_COVARIANT,
			definitions_output_covariant_complete : COMPLETE,
			definitions_output_covariant_compact : COMPACT,
			predicate : VALUE_KINDS_PREDICATE,
			aliases : ALIASES,
			aliases_compact : COMPACT,
			features : FEATURES,
			categories : DumpCmarkLinkedCategoriesConfiguration {
					direct : VALUE_KINDS_CATEGORIES,
					direct_complete : COMPLETE,
					direct_compact : COMPACT,
					recursive : VALUE_KINDS_CATEGORIES_RECURSIVE,
					recursive_complete : COMPLETE,
					recursive_compact : COMPACT,
				},
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let value_kinds = DumpCmarkValueKindsConfiguration {
			enabled : VALUE_KINDS,
			toc : VALUE_KINDS_TOC,
			toc_complete : TOC_COMPLETE,
			toc_depth : TOC_DEPTH,
			configuration : value_kind,
			generic : generic,
		};
	
	
	let appendix = DumpCmarkAppendixConfiguration {
			description : DESCRIPTIONS,
			links : LINKS,
			generic : generic,
		};
	
	let appendices = DumpCmarkAppendicesConfiguration {
			enabled : APPENDICES,
			toc : APPENDICES_TOC,
			configuration : appendix,
			generic : generic,
		};
	
	
	let library = DumpCmarkLibraryConfiguration {
			toc : LIBRARIES_TOC,
			toc_compact : false,
			features : FEATURES,
			description : DESCRIPTIONS,
			links : LINKS,
			categories : categories,
			exports : exports,
			definitions : definitions,
			value_kinds : value_kinds,
			appendices : appendices,
			generic : generic,
		};
	
	let libraries = DumpCmarkLibrariesConfiguration {
			enabled : LIBRARIES,
			configuration : library,
			generic : generic,
		};
	
	
	succeed! (libraries);
}




pub trait DumpCmarkCallbacks {
	fn buffer_build (&mut self) -> (StdVec<u8>);
	fn buffer_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, buffer : StdVec<u8>) -> (Outcome<()>);
	fn anchor_generate (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, source : &str) -> (Outcome<StdString>);
	fn anchor_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
	fn title_write (&mut self, kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>, configuration : &DumpCmarkGenericConfiguration, buffer : &mut StdVec<u8>) -> (Outcome<()>);
	fn break_write (&mut self, library : &Library, configuration : &DumpCmarkGenericConfiguration, anchor_source : &str, buffer : &mut StdVec<u8>) -> (Outcome<()>);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_execute (libraries : &Libraries, configuration : &DumpCmarkLibrariesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	for library in libraries.libraries () {
		
		if configuration.enabled {
			let configuration = &configuration.configuration;
			try! (dump_cmark_library (library, configuration, callbacks));
		}
		
		let configuration = &configuration.configuration;
		
		
		if configuration.categories.enabled && library.has_categories () {
			let configuration = &configuration.categories;
			try! (dump_cmark_categories (library, library.categories (), configuration, callbacks));
			
			for category in library.categories () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_category (library, category, configuration, callbacks));
			}
		}
		
		
		if configuration.exports.enabled && library.has_exports () {
			let configuration = &configuration.exports;
			try! (dump_cmark_exports (library, library.exports (), configuration, callbacks));
			
			for export in library.exports () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_export (library, export, configuration, callbacks));
			}
		}
		
		
		if configuration.definitions.enabled && library.has_definitions () {
			let configuration = &configuration.definitions;
			try! (dump_cmark_definitions (library, library.definitions (), configuration, callbacks));
			
			for definition in library.definitions () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_definition (library, definition, configuration, callbacks));
			}
		}
		
		
		if configuration.value_kinds.enabled && library.has_value_kinds () {
			let configuration = &configuration.value_kinds;
			try! (dump_cmark_value_kinds (library, library.value_kinds (), configuration, callbacks));
			
			for value_kind in library.value_kinds () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_value_kind (library, value_kind, configuration, callbacks));
			}
		}
		
		
		if configuration.appendices.enabled && library.has_appendices () {
			let configuration = &configuration.appendices;
			try! (dump_cmark_appendices (library, library.appendices (), configuration, callbacks));
			
			for appendix in library.appendices () {
				let configuration = &configuration.configuration;
				try! (dump_cmark_appendix (library, appendix, configuration, callbacks));
			}
		}
	}
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (needless_pass_by_value) ) ]
fn dump_cmark_libraries <'a> (_libraries : impl iter::ExactSizeIterator<Item = &'a Library>, _configuration : &DumpCmarkLibrariesConfiguration, _callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_library (library : &Library, configuration : &DumpCmarkLibraryConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("library"), Some (library.identifier ()), None, &configuration.generic, stream));
	try! (callbacks.title_write (Some ("library"), Some (library.identifier ()), None, library.title (), &configuration.generic, stream));
	
	if configuration.toc {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		let mut empty = true;
		if configuration.categories.enabled && library.has_categories () {
			let categories_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("categories"), "library"));
			if configuration.toc_compact {
				try_writeln! (stream, "[categories]({});", &categories_anchor);
			} else {
				try_writeln! (stream, " * [categories]({});", &categories_anchor);
			}
			empty = false;
		}
		if configuration.exports.enabled && library.has_exports () {
			let exports_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("exports"), "library"));
			if configuration.toc_compact {
				try_writeln! (stream, "[exports]({});", &exports_anchor);
			} else {
				try_writeln! (stream, " * [exports]({});", &exports_anchor);
			}
			empty = false;
		}
		if configuration.definitions.enabled && library.has_definitions () {
			let definitions_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("definitions"), "library"));
			if configuration.toc_compact {
				try_writeln! (stream, "[definitions]({});", &definitions_anchor);
			} else {
				try_writeln! (stream, " * [definitions]({});", &definitions_anchor);
			}
			empty = false;
		}
		if configuration.value_kinds.enabled && library.has_value_kinds () {
			let value_kinds_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), "library"));
			if configuration.toc_compact {
				try_writeln! (stream, "[types]({});", &value_kinds_anchor);
			} else {
				try_writeln! (stream, " * [types]({});", &value_kinds_anchor);
			}
			empty = false;
		}
		if configuration.appendices.enabled && library.has_appendices () {
			let appendices_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("appendices"), "library"));
			if configuration.toc_compact {
				try_writeln! (stream, "[appendices]({});", &appendices_anchor);
			} else {
				try_writeln! (stream, " * [appendices]({});", &appendices_anchor);
			}
			empty = false;
		}
		if empty {
			if configuration.toc_compact {
				try_writeln! (stream, "(nothing);");
			} else {
				try_writeln! (stream, " * (nothing);");
			}
		}
	}
	
	if configuration.features {
		if let Some (features) = library.features () {
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Features");
			try_writeln! (stream);
			try_writeln! (stream, "````");
			try_writeln! (stream, "{}", dump_cmark_value_format (& features.format ()));
			try_writeln! (stream, "````");
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, library.description (), library.links (), &configuration.generic, callbacks, "library", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, library.links (), &configuration.generic, callbacks, "library", stream));
	}
	
	try! (callbacks.break_write (library, &configuration.generic, "library", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("library"), Some (library.identifier ()), None, stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_categories <'a> (library : &'a Library, categories : impl iter::ExactSizeIterator<Item = &'a Category>, configuration : &DumpCmarkCategoriesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("toc"), Some (library.identifier ()), Some ("categories"), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("toc"), Some (library.identifier ()), Some ("categories"), None, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		
		for category in categories {
			if category.has_parents () {
				continue;
			}
			let mut stack = StdVec::new ();
			stack.push ((category, true, category.children ()));
			while let Some ((category, emit, sub_categories)) = stack.pop () {
				if emit {
					let padding = "  " .repeat (stack.len ());
					let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), "categories"));
					if category.has_children () {
						try_writeln! (stream, "{}* [`{}`]({}):", padding, category.identifier (), category_anchor);
					} else {
						try_writeln! (stream, "{}* [`{}`]({});", padding, category.identifier (), category_anchor);
					}
					stack.push ((category, false, sub_categories));
				} else {
					let mut sub_categories = sub_categories;
					if let Some (sub_category) = sub_categories.next () {
						stack.push ((category, false, sub_categories));
						stack.push ((sub_category, true, sub_category.children ()));
					}
				}
			}
		}
		
		try! (callbacks.break_write (library, &configuration.generic, "categories", stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("toc"), Some (library.identifier ()), Some ("categories"), stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_category (library : &Library, category : &Category, configuration : &DumpCmarkCategoryConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), None, &configuration.generic, stream));
	
	if configuration.exports.direct && category.has_exports () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Exports");
		try_writeln! (stream);
		for export in category.exports () {
			let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "category"));
			if configuration.exports.direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
	}
	
	if configuration.definitions.direct && category.has_definitions () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Definitions");
		try_writeln! (stream);
		for definition in category.definitions () {
			let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "category"));
			if configuration.definitions.direct_compact {
				try_writeln! (stream, "[`{}`]({});", definition.identifier (), definition_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", definition.identifier (), definition_anchor);
			}
		}
	}
	
	if configuration.value_kinds.direct && category.has_value_kinds () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Types");
		try_writeln! (stream);
		for value_kind in category.value_kinds () {
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "category"));
			if configuration.value_kinds.direct_compact {
				try_writeln! (stream, "[`{}`]({});", value_kind.identifier (), value_kind_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", value_kind.identifier (), value_kind_anchor);
			}
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, category.description (), category.links (), &configuration.generic, callbacks, "category", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, category.links (), &configuration.generic, callbacks, "category", stream));
	}
	
	if configuration.hierarchy.super_direct && category.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-category");
		try_writeln! (stream);
		for category in category.parents () {
			let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), "category"));
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "[`{}`]({});", category.identifier (), category_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", category.identifier (), category_anchor);
			}
		}
		if configuration.hierarchy.super_recursive
				&& category.parents () .count () != category.parents_recursive () .count ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Super-categories recursive");
			try_writeln! (stream);
			for category in category.parents_recursive () {
				let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), "category"));
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", category.identifier (), category_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", category.identifier (), category_anchor);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-category");
		try_writeln! (stream);
		let categories_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("categories"), "category"));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &categories_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &categories_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && category.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Sub-categories");
		try_writeln! (stream);
		for category in category.children () {
			let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), "category"));
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "[`{}`]({});", category.identifier (), category_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", category.identifier (), category_anchor);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& category.children () .count () != category.children_recursive () .count ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Sub-categories recursive");
			try_writeln! (stream);
			for category in category.children_recursive () {
				let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), "category"));
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", category.identifier (), category_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", category.identifier (), category_anchor);
				}
			}
		}
	}
	
	if configuration.exports.recursive
			&& category.exports () .count () != category.exports_recursive () .count ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Exports recursive");
		try_writeln! (stream);
		for export in category.exports_recursive () {
			let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "category"));
			if configuration.exports.recursive_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
	}
	
	if configuration.definitions.recursive
			&& category.definitions () .count () != category.definitions_recursive () .count ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Definitions recursive");
		try_writeln! (stream);
		for definition in category.definitions_recursive () {
			let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "category"));
			if configuration.definitions.recursive_compact {
				try_writeln! (stream, "[`{}`]({});", definition.identifier (), definition_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", definition.identifier (), definition_anchor);
			}
		}
	}
	
	if configuration.value_kinds.recursive
			&& category.value_kinds () .count () != category.value_kinds_recursive () .count ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Types recursive");
		try_writeln! (stream);
		for value_kind in category.value_kinds_recursive () {
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "category"));
			if configuration.value_kinds.recursive_compact {
				try_writeln! (stream, "[`{}`]({});", value_kind.identifier (), value_kind_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", value_kind.identifier (), value_kind_anchor);
			}
		}
	}
	
	try! (callbacks.break_write (library, &configuration.generic, "category", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_exports <'a> (library : &'a Library, exports : impl iter::ExactSizeIterator<Item = &'a Export>, configuration : &DumpCmarkExportsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("toc"), Some (library.identifier ()), Some ("exports"), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("toc"), Some (library.identifier ()), Some ("exports"), None, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		
		for export in exports {
			if export.has_parents () {
				continue;
			}
			let mut stack = StdVec::new ();
			stack.push ((export, true, export.children ()));
			while let Some ((export, emit, sub_exports)) = stack.pop () {
				if emit {
					let padding = "  " .repeat (stack.len ());
					let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "exports"));
					if export.has_children () {
						try_writeln! (stream, "{}* [`{}`]({}):", padding, export.identifier (), export_anchor);
					} else {
						try_writeln! (stream, "{}* [`{}`]({});", padding, export.identifier (), export_anchor);
					}
					stack.push ((export, false, sub_exports));
				} else {
					let mut sub_exports = sub_exports;
					if let Some (sub_export) = sub_exports.next () {
						stack.push ((export, false, sub_exports));
						stack.push ((sub_export, true, sub_export.children ()));
					}
				}
			}
		}
		
		try! (callbacks.break_write (library, &configuration.generic, "exports", stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("toc"), Some (library.identifier ()), Some ("exports"), stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_export (library : &Library, export : &Export, configuration : &DumpCmarkExportConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), None, &configuration.generic, stream));
	
	if configuration.descriptor {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Descriptor");
		try_writeln! (stream);
		try_writeln! (stream, "````");
		try_writeln! (stream, "{}", dump_cmark_value_format (& export.descriptor_format ()));
		try_writeln! (stream, "````");
	}
	
	if configuration.definitions.direct && export.has_definitions () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Definitions");
		try_writeln! (stream);
		for definition in export.definitions () {
			let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "export"));
			if configuration.definitions.direct_compact {
				try_writeln! (stream, "[`{}`]({});", definition.identifier (), definition_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", definition.identifier (), definition_anchor);
			}
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, export.description (), export.links (), &configuration.generic, callbacks, "export", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, export.links (), &configuration.generic, callbacks, "export", stream));
	}
	
	if configuration.hierarchy.super_direct && export.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-export");
		try_writeln! (stream);
		for export in export.parents () {
			let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "export"));
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
		if configuration.hierarchy.super_recursive
				&& export.parents () .count () != export.parents_recursive () .count ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Super-exports recursive");
			try_writeln! (stream);
			for export in export.parents_recursive () {
				let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "export"));
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-export");
		try_writeln! (stream);
		let exports_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("exports"), "export"));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &exports_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &exports_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && export.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Sub-exports");
		try_writeln! (stream);
		for export in export.children () {
			let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "export"));
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& export.children () .count () != export.children_recursive () .count ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Sub-exports recursive");
			try_writeln! (stream);
			for export in export.children_recursive () {
				let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "export"));
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
				} else {
					try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
				}
			}
		}
	}
	
	if configuration.definitions.recursive
			&& export.definitions () .count () != export.definitions_recursive () .count ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Definitions recursive");
		try_writeln! (stream);
		for definition in export.definitions_recursive () {
			let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "export"));
			if configuration.definitions.recursive_compact {
				try_writeln! (stream, "[`{}`]({});", definition.identifier (), definition_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", definition.identifier (), definition_anchor);
			}
		}
	}
	
	if configuration.features {
		if let Some (features) = export.features () {
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Features");
			try_writeln! (stream);
			try_writeln! (stream, "````");
			try_writeln! (stream, "{}", dump_cmark_value_format (& features.format ()));
			try_writeln! (stream, "````");
		}
	}
	
	try! (dump_cmark_entity_categories_write (library, export.categories (), export.categories_recursive (), &configuration.categories, &configuration.generic, callbacks, "export", stream));
	
	try! (callbacks.break_write (library, &configuration.generic, "export", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_definitions <'a> (library : &'a Library, definitions : impl iter::ExactSizeIterator<Item = &'a Definition>, configuration : &DumpCmarkDefinitionsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("toc"), Some (library.identifier ()), Some ("definitions"), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("toc"), Some (library.identifier ()), Some ("definitions"), None, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		
		for definition in definitions {
			let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "definitions"));
			try_writeln! (stream, "* [`{}`]({});", definition.identifier (), definition_anchor);
		}
		
		try! (callbacks.break_write (library, &configuration.generic, "definitions", stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("toc"), Some (library.identifier ()), Some ("definitions"), stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_definition (library : &Library, definition : &Definition, configuration : &DumpCmarkDefinitionConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), None, &configuration.generic, stream));
	
	if configuration.kind {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Kind");
		try_writeln! (stream);
		try_writeln! (stream, "`{}`;", definition.kind () .identifier ());
	}
	
	if configuration.exports.direct && definition.has_exports () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Exports");
		try_writeln! (stream);
		for export in definition.exports () {
			let export_anchor = try! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), "definition"));
			if configuration.exports.direct_compact {
				try_writeln! (stream, "[`{}`]({});", export.identifier (), export_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", export.identifier (), export_anchor);
			}
		}
	}
	
	if let Some (procedure_signature) = if configuration.signature { definition.procedure_signature () } else { None } {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Procedure signature");
		if ! procedure_signature.variants.is_empty () {
			try_writeln! (stream);
			try_writeln! (stream, "Procedure variants:");
			for procedure_signature_variant in procedure_signature.variants.iter () {
				#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
				fn write_procedure_signature_value (library : &Library, value : &ProcedureSignatureValue, prefix : &str, callbacks : &mut impl DumpCmarkCallbacks, stream : &mut impl io::Write) -> (Outcome<()>) {
					let value_kind = try! (value.kind.entity_resolve ());
					let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "definition"));
					if let Some (identifier) = value.identifier.as_ref () {
						try_writeln! (stream, "{}`{}` of type [`{}`]({});", prefix, identifier, value_kind.identifier (), value_kind_anchor);
					} else {
						try_writeln! (stream, "{}a value of type [`{}`]({});", prefix, value_kind.identifier (), value_kind_anchor);
					}
					succeed! (());
				}
				try_writeln! (stream, " * `{}`", dump_cmark_value_format (& procedure_signature_variant.format ()));
				if ! procedure_signature_variant.inputs.values.is_empty () {
					let procedure_signature_variant_inputs = &procedure_signature_variant.inputs;
					if procedure_signature_variant_inputs.values.len () > 1 || procedure_signature_variant_inputs.variadic {
						try_writeln! (stream, "   * inputs:");
						for procedure_signature_value in procedure_signature_variant_inputs.values.iter () {
							try! (write_procedure_signature_value (library, procedure_signature_value, "     * ", callbacks, stream));
						}
						if procedure_signature_variant_inputs.variadic {
							try_writeln! (stream, "     * `...` (i.e. variadic);");
						}
					} else {
						try! (write_procedure_signature_value (library, &procedure_signature_variant_inputs.values[0], "   * input: ", callbacks, stream));
					}
				} else {
					try_writeln! (stream, "   * inputs: none;");
				}
				if ! procedure_signature_variant.outputs.values.is_empty () {
					let procedure_signature_variant_outputs = &procedure_signature_variant.outputs;
					if procedure_signature_variant_outputs.values.len () > 1 || procedure_signature_variant_outputs.variadic {
						try_writeln! (stream, "   * outputs:");
						for procedure_signature_value in procedure_signature_variant_outputs.values.iter () {
							try! (write_procedure_signature_value (library, procedure_signature_value, "     * ", callbacks, stream));
						}
						if procedure_signature_variant_outputs.variadic {
							try_writeln! (stream, "     * `...` (i.e. variadic);");
						}
					} else {
						try! (write_procedure_signature_value (library, &procedure_signature_variant_outputs.values[0], "   * output: ", callbacks, stream));
					}
				} else {
					try_writeln! (stream, "   * outputs: none;");
				}
				if let Some (features) = &procedure_signature_variant.features {
					try_writeln! (stream, "   * requires: `{}`", dump_cmark_value_format (& features.format ()));
				}
			}
		}
	} else if definition.kind () .is_procedure () && configuration.generic.lints && configuration.signature {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Procedure signature");
		try_writeln! (stream);
		try_writeln! (stream, "**FIXME!**  No procedure signature was provided!");
	}
	
	if let Some (syntax_signature) = if configuration.signature { definition.syntax_signature () } else { None } {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Syntax signature");
		if ! syntax_signature.keywords.is_empty () {
			try_writeln! (stream);
			try_writeln! (stream, "Syntax keywords:");
			for syntax_signature_keyword in syntax_signature.keywords.iter () {
				match syntax_signature_keyword.deref () {
					SyntaxSignatureKeyword::Literal (identifier) =>
						try_writeln! (stream, " * `{}`: literal;", identifier),
					SyntaxSignatureKeyword::Identifier (identifier) =>
						try_writeln! (stream, " * `{}`: identifier;", identifier),
					SyntaxSignatureKeyword::Expression (identifier) =>
						try_writeln! (stream, " * `{}`: expression;", identifier),
					SyntaxSignatureKeyword::Constant { identifier, value } =>
						try_writeln! (stream, " * `{}`: constant with value `{}`;", identifier, dump_cmark_value_format (value)),
					SyntaxSignatureKeyword::Value { identifier, kind : value_kind } =>
						if let Some (value_kind) = value_kind {
							let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "definition"));
							try_writeln! (stream, " * `{}`: value of type [{}]({});", identifier, value_kind.identifier (), value_kind_anchor);
						} else {
							try_writeln! (stream, " * `{}`: value with unspecified type;", identifier);
						},
					SyntaxSignatureKeyword::Pattern { identifier, patterns } =>
						if ! patterns.is_empty () {
							try_writeln! (stream, " * `{}`: pattern with variants:", identifier);
							for pattern in patterns.iter () {
								try_writeln! (stream, "   * `{}`;", dump_cmark_value_format (& pattern.format ()));
							}
						},
				}
			}
		}
		if ! syntax_signature.variants.is_empty () {
			try_writeln! (stream);
			try_writeln! (stream, "Syntax variants:");
			for syntax_signature_variant in syntax_signature.variants.iter () {
				try_writeln! (stream, " * `{}`", dump_cmark_value_format (& syntax_signature_variant.pattern.format ()));
			}
		}
	} else if definition.kind () .is_syntax () && configuration.generic.lints && configuration.signature {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Syntax signature");
		try_writeln! (stream);
		try_writeln! (stream, "**FIXME!**  No syntax signature was provided!");
	}
	
	if configuration.value_kinds.direct && definition.has_referenced_value_kinds () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Referenced types");
		try_writeln! (stream);
		for value_kind in definition.referenced_value_kinds () {
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "definition"));
			if configuration.value_kinds.direct_compact {
				try_writeln! (stream, "[`{}`]({});", value_kind.identifier (), value_kind_anchor);
			} else {
				try_writeln! (stream, " * [`{}`]({});", value_kind.identifier (), value_kind_anchor);
			}
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, definition.description (), definition.links (), &configuration.generic, callbacks, "definition", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, definition.links (), &configuration.generic, callbacks, "definition", stream));
	}
	
	if configuration.aliases && definition.has_aliases () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Aliases");
		try_writeln! (stream);
		for alias in definition.aliases () {
			if configuration.aliases_compact {
				try_writeln! (stream, "`{}`;", alias);
			} else {
				try_writeln! (stream, " * `{}`;", alias);
			}
		}
	}
	
	if configuration.features {
		if let Some (features) = definition.features () {
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Features");
			try_writeln! (stream);
			try_writeln! (stream, "````");
			try_writeln! (stream, "{}", dump_cmark_value_format (& features.format ()));
			try_writeln! (stream, "````");
		}
	}
	
	try! (dump_cmark_entity_categories_write (library, definition.categories (), definition.categories_recursive (), &configuration.categories, &configuration.generic, callbacks, "definition", stream));
	
	try! (callbacks.break_write (library, &configuration.generic, "definition", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_kinds <'a> (library : &Library, value_kinds : impl iter::ExactSizeIterator<Item = &'a ValueKind>, configuration : &DumpCmarkValueKindsConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), None, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		
		let mut value_kinds_seen = StdSet::new ();
		for value_kind in value_kinds {
			if value_kind.has_parents () {
				continue;
			}
			try! (dump_cmark_value_kind_write_tree (library, value_kind, &mut value_kinds_seen, stream, configuration.toc_complete, configuration.toc_depth, callbacks, "value_kinds"));
		}
		
		try! (callbacks.break_write (library, &configuration.generic, "value_kinds", stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_value_kind (library : &Library, value_kind : &ValueKind, configuration : &DumpCmarkValueKindConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), None, &configuration.generic, stream));
	
	if configuration.tree
			&& value_kind.has_children ()
			&& value_kind.children () .count () != value_kind.children_recursive () .count ()
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Sub-types tree");
		try_writeln! (stream);
		let mut value_kinds_seen = StdSet::new ();
		for value_kind in value_kind.children () {
			try! (dump_cmark_value_kind_write_tree (library, value_kind, &mut value_kinds_seen, stream, configuration.tree_complete, configuration.tree_depth, callbacks, "value_kind"));
		}
	}
	
	let mut value_kind_covariants_seen = StdSet::new ();
	let mut value_kind_contravariants_seen = StdSet::new ();
	
	if configuration.hierarchy.super_direct && value_kind.has_parents () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-type");
		try_writeln! (stream);
		for value_kind in value_kind.parents () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.hierarchy.super_direct_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.hierarchy.super_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.super_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.hierarchy.super_recursive
				&& (configuration.hierarchy.super_recursive_complete || value_kind.parents () .count () != value_kind.parents_recursive () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Super-types recursive");
			try_writeln! (stream);
			for value_kind in value_kind.parents_recursive () {
				let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
					if configuration.hierarchy.super_recursive_complete { true } else { continue; }
				} else {
					value_kind_covariants_seen.insert (value_kind.identifier ()); false
				};
				let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
				let fixes = if configuration.hierarchy.super_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.super_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				}
			}
		}
	} else if configuration.hierarchy.super_direct {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Super-type");
		try_writeln! (stream);
		let value_kinds_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), "value_kind"));
		if configuration.hierarchy.super_direct_compact {
			try_writeln! (stream, "[(none)]({});", &value_kinds_anchor);
		} else {
			try_writeln! (stream, " * [(none)]({});", &value_kinds_anchor);
		}
	}
	
	if configuration.hierarchy.sub_direct && value_kind.has_children () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Sub-types");
		try_writeln! (stream);
		for value_kind in value_kind.children () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.hierarchy.sub_direct_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.hierarchy.sub_direct_complete && !seen { "**" } else { "" };
			if configuration.hierarchy.sub_direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.hierarchy.sub_recursive
				&& (configuration.hierarchy.sub_recursive_complete || value_kind.children () .count () != value_kind.children_recursive () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "##### Sub-types recursive");
			try_writeln! (stream);
			for value_kind in value_kind.children_recursive () {
				let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
					if configuration.hierarchy.sub_recursive_complete { true } else { continue; }
				} else {
					value_kind_contravariants_seen.insert (value_kind.identifier ()); false
				};
				let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
				let fixes = if configuration.hierarchy.sub_recursive_complete && !seen { "**" } else { "" };
				if configuration.hierarchy.sub_recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
				}
			}
		}
	}
	
	if configuration.covariants.direct
			&& value_kind.has_covariants ()
			&& (configuration.covariants.direct_complete || value_kind.covariants () .count () != value_kind_covariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Covariant types");
		try_writeln! (stream);
		for value_kind in value_kind.covariants () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.covariants.direct_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.covariants.direct_complete && !seen { "**" } else { "" };
			if configuration.covariants.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	}
	if configuration.covariants.recursive
			&& ! value_kind.covariants_recursive () .is_empty ()
			&& (configuration.covariants.recursive_complete || value_kind.covariants_recursive () .count () != value_kind_covariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Covariant types recursive");
		try_writeln! (stream);
		for value_kind in value_kind.covariants_recursive () {
			let seen = if value_kind_covariants_seen.contains (value_kind.identifier ()) {
				if configuration.covariants.recursive_complete { true } else { continue; }
			} else {
				value_kind_covariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.covariants.recursive_complete && !seen { "**" } else { "" };
			if configuration.covariants.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition producing this type, can be used instead of a definition producing any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	}
	
	if configuration.contravariants.direct
			&& value_kind.has_contravariants ()
			&& (configuration.contravariants.direct_complete || value_kind.contravariants () .count () != value_kind_contravariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contravariant types");
		try_writeln! (stream);
		for value_kind in value_kind.contravariants () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.contravariants.direct_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.contravariants.direct_complete && !seen { "**" } else { "" };
			if configuration.contravariants.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	}
	if configuration.contravariants.recursive
			&& ! value_kind.contravariants_recursive () .is_empty ()
			&& (configuration.contravariants.recursive_complete || value_kind.contravariants_recursive () .count () != value_kind_contravariants_seen.len ())
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contravariant types recursive");
		try_writeln! (stream);
		for value_kind in value_kind.contravariants_recursive () {
			let seen = if value_kind_contravariants_seen.contains (value_kind.identifier ()) {
				if configuration.contravariants.recursive_complete { true } else { continue; }
			} else {
				value_kind_contravariants_seen.insert (value_kind.identifier ()); false
			};
			let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), "value_kind"));
			let fixes = if configuration.contravariants.recursive_complete && !seen { "**" } else { "" };
			if configuration.contravariants.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, value_kind.identifier (), value_kind_anchor, fixes);
			}
		}
		if configuration.generic.notes {
			try_writeln! (stream);
			try_writeln! (stream, "Note:  A definition consuming this type, can be used instead of a definition consuming any of these listed types (provided that the other types used in the definition also \"match\").");
		}
	}
	
	if configuration.definitions_input.direct || configuration.definitions_input.recursive || configuration.definitions_input_contravariant {
		let mut value_kind_definitions_seen = StdSet::new ();
		if configuration.definitions_input.direct
				&& value_kind.has_definitions_input ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as input");
			try_writeln! (stream);
			for definition in value_kind.definitions_input () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input.direct_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_input.direct_complete && !seen { "**" } else { "" };
				if configuration.definitions_input.direct_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
		}
		if configuration.definitions_input.recursive
				&& value_kind.has_definitions_input ()
				&& (configuration.definitions_input.recursive_complete || value_kind.definitions_input_recursive () .count () != value_kind.definitions_input () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as input (recursive)");
			try_writeln! (stream);
			for definition in value_kind.definitions_input_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input.recursive_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_input.recursive_complete && !seen { "**" } else { "" };
				if configuration.definitions_input.recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions consume an input that is a super-type.");
			}
		}
		if configuration.definitions_input_contravariant
				&& ! value_kind.definitions_input_contravariant_recursive () .is_empty ()
				&& (configuration.definitions_input_contravariant_complete || value_kind.definitions_input_contravariant_recursive () .count () != value_kind.definitions_input_recursive () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as input (contravariant)");
			try_writeln! (stream);
			for definition in value_kind.definitions_input_contravariant_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_input_contravariant_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_input_contravariant_complete && !seen { "**" } else { "" };
				if configuration.definitions_input_contravariant_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions consume an input that is a super-type-like (i.e. contravariant).");
			}
		}
	}
	
	if configuration.definitions_output.direct || configuration.definitions_output.recursive || configuration.definitions_output_covariant {
		let mut value_kind_definitions_seen = StdSet::new ();
		if configuration.definitions_output.direct
				&& value_kind.has_definitions_output ()
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as output");
			try_writeln! (stream);
			for definition in value_kind.definitions_output () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output.direct_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_output.direct_complete && !seen { "**" } else { "" };
				if configuration.definitions_output.direct_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
		}
		if configuration.definitions_output.recursive
				&& value_kind.has_definitions_output ()
				&& (configuration.definitions_output.recursive_complete || value_kind.definitions_output_recursive () .count () != value_kind.definitions_output () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as output (recursive)");
			try_writeln! (stream);
			for definition in value_kind.definitions_output_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output.recursive_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_output.recursive_complete && !seen { "**" } else { "" };
				if configuration.definitions_output.recursive_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type.");
			}
		}
		if configuration.definitions_output_covariant
				&& ! value_kind.definitions_output_covariant_recursive () .is_empty ()
				&& (configuration.definitions_output_covariant_complete || value_kind.definitions_output_covariant_recursive () .count () != value_kind.definitions_output_recursive () .count ())
		{
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Referent definitions as output (covariant)");
			try_writeln! (stream);
			for definition in value_kind.definitions_output_covariant_recursive () {
				let seen = if value_kind_definitions_seen.contains (definition.identifier ()) {
					if configuration.definitions_output_covariant_complete { true } else { continue; }
				} else {
					value_kind_definitions_seen.insert (definition.identifier ()); false
				};
				let definition_anchor = try! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), "value_kind"));
				let fixes = if configuration.definitions_output_covariant_complete && !seen { "**" } else { "" };
				if configuration.definitions_output_covariant_compact {
					try_writeln! (stream, "{}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				} else {
					try_writeln! (stream, " * {}[`{}`]({}){};", fixes, definition.identifier (), definition_anchor, fixes);
				}
			}
			if configuration.generic.notes {
				try_writeln! (stream);
				try_writeln! (stream, "Note:  These definitions produce an output that is a sub-type-like (i.e. covariant).");
			}
		}
	}
	
	if configuration.description {
		try! (dump_cmark_description_write (library, value_kind.description (), value_kind.links (), &configuration.generic, callbacks, "value_kind", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, value_kind.links (), &configuration.generic, callbacks, "value_kind", stream));
	}
	
	if configuration.predicate {
		if let Some (predicate) = value_kind.predicate () {
			match predicate {
				ValueKindPredicate::None =>
					(),
				ValueKindPredicate::Inherit =>
					(),
				ValueKindPredicate::Fixme =>
					if configuration.generic.fixme {
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Predicate");
						try_writeln! (stream);
						try_writeln! (stream, "**FIXME!**");
					}
				ValueKindPredicate::Expression (ref value) =>
					{
						try_writeln! (stream);
						try_writeln! (stream);
						try_writeln! (stream, "#### Predicate");
						try_writeln! (stream);
						try_writeln! (stream, "```");
						try_writeln! (stream, "{}", dump_cmark_value_format (value));
						try_writeln! (stream, "```");
					},
			}
		} else {
			if configuration.generic.lints {
				try_writeln! (stream);
				try_writeln! (stream);
				try_writeln! (stream, "#### Predicate");
				try_writeln! (stream);
				try_writeln! (stream, "**FIXME!**  No predicate was provided!");
			}
		}
	}
	
	if configuration.aliases && value_kind.has_aliases () {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Aliases");
		try_writeln! (stream);
		for alias in value_kind.aliases () {
			if configuration.aliases_compact {
				try_writeln! (stream, "`{}`;", alias);
			} else {
				try_writeln! (stream, " * `{}`;", alias);
			}
		}
	}
	
	if configuration.features {
		if let Some (features) = value_kind.features () {
			try_writeln! (stream);
			try_writeln! (stream);
			try_writeln! (stream, "#### Features");
			try_writeln! (stream);
			try_writeln! (stream, "````");
			try_writeln! (stream, "{}", dump_cmark_value_format (& features.format ()));
			try_writeln! (stream, "````");
		}
	}
	
	try! (dump_cmark_entity_categories_write (library, value_kind.categories (), value_kind.categories_recursive (), &configuration.categories, &configuration.generic, callbacks, "value_kind", stream));
	
	try! (callbacks.break_write (library, &configuration.generic, "value_kind", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_appendices <'a> (library : &Library, appendices : impl iter::ExactSizeIterator<Item = &'a Appendix>, configuration : &DumpCmarkAppendicesConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("toc"), Some (library.identifier ()), Some ("appendices"), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("toc"), Some (library.identifier ()), Some ("appendices"), None, &configuration.generic, stream));
	
	if configuration.toc {
		
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Contents");
		try_writeln! (stream);
		
		for appendix in appendices {
			let appendix_anchor = try! (callbacks.anchor_generate (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), "appendices"));
			if let Some (title) = appendix.title () {
				try_writeln! (stream, "* [`{}`]({}) -- {};", appendix.identifier (), appendix_anchor, title);
			} else {
				try_writeln! (stream, "* [`{}`]({});", appendix.identifier (), appendix_anchor);
			}
		}
		
		try! (callbacks.break_write (library, &configuration.generic, "appendices", stream));
	}
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("toc"), Some (library.identifier ()), Some ("appendices"), stream_buffer));
	
	succeed! (());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_appendix (library : &Library, appendix : &Appendix, configuration : &DumpCmarkAppendixConfiguration, callbacks : &mut impl DumpCmarkCallbacks) -> (Outcome<()>) {
	
	let mut stream_buffer = callbacks.buffer_build ();
	{ // NOTE:  This begins the scope for `stream`!
	let stream = &mut stream_buffer;
	
	try_writeln! (stream);
	try_writeln! (stream);
	try! (callbacks.anchor_write (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), &configuration.generic, stream));
	try! (callbacks.title_write (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), appendix.title (), &configuration.generic, stream));
	
	if configuration.description {
		try! (dump_cmark_description_write (library, appendix.description (), appendix.links (), &configuration.generic, callbacks, "appendix", stream));
	}
	if configuration.links {
		try! (dump_cmark_links_write (library, appendix.links (), &configuration.generic, callbacks, "appendix", stream));
	}
	
	try! (callbacks.break_write (library, &configuration.generic, "appendix", stream));
	
	} // NOTE:  This ends the scope for `stream`!
	try! (callbacks.buffer_write (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), stream_buffer));
	
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_mangle_identifier (identifier : &str) -> (StdString) {
	let mut buffer = StdString::with_capacity (identifier.len ());
	let mut first = true;
	for character in identifier.chars () {
		match character {
			'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' =>
				buffer.push (character),
			'-' if !first =>
				buffer.push (character),
			_ => {
				let mut character_buffer = [0; 8];
				let character_bytes = character.encode_utf8 (&mut character_buffer) .as_bytes ();
				if first {
					buffer.push_str ("ZZZZ__");
				}
				if let Some (buffer_last) = buffer.as_bytes () .last () .cloned () {
					if buffer_last != b'_' {
						buffer.push ('_');
					}
				}
				for character_byte in character_bytes {
					buffer.push_str (& format! ("{:02x}", character_byte));
				}
				buffer.push ('_');
			}
		}
		first = false;
	}
	if let Some (buffer_last) = buffer.pop () {
		if buffer_last != '_' {
			buffer.push (buffer_last);
		}
	}
	return buffer;
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_path_generate (kind : Option<&str>, library : Option<&str>, entity : Option<&str>, prefix : &str, suffix : &str) -> (Outcome<StdString>) {
	match (kind, library, entity) {
		(Some ("toc"), Some (library), Some (entity)) => {
			let entity = match entity {
				"categories" => "categories",
				"exports" => "exports",
				"definitions" => "definitions",
				"value_kinds" => "types",
				"appendices" => "appendices",
				_ => fail! (0x4bef3a8f),
			};
			let library = dump_cmark_anchor_mangle_identifier (library);
			succeed! (format! ("{}{}/{}/_index{}", prefix, library, entity, suffix));
		},
		(Some ("library"), Some (library), None) => {
			let library = dump_cmark_anchor_mangle_identifier (library);
			succeed! (format! ("{}{}/_index{}", prefix, library, suffix));
		},
		(Some (kind), Some (library), Some (entity)) => {
			let kind = match kind {
				"category" => "categories",
				"export" => "exports",
				"definition" => "definitions",
				"value_kind" => "types",
				"appendix" => "appendices",
				_ => fail! (0xf8250848),
			};
			let library = dump_cmark_anchor_mangle_identifier (library);
			let entity = dump_cmark_anchor_mangle_identifier (entity);
			succeed! (format! ("{}{}/{}/{}{}", prefix, library, kind, entity, suffix));
		},
		_ =>
			fail! (0x165bf432),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_generate (kind : Option<&str>, library : Option<&str>, entity : Option<&str>) -> (Outcome<StdString>) {
	match (kind, library, entity) {
		(Some ("toc"), Some (library), Some (entity)) => {
			let entity = match entity {
				"categories" => "categories",
				"exports" => "exports",
				"definitions" => "definitions",
				"value_kinds" => "types",
				"appendices" => "appendices",
				_ => fail! (0x4bef3a8f),
			};
			let library = dump_cmark_anchor_mangle_identifier (library);
			succeed! (format! ("toc__{}__{}", library, entity));
		},
		(Some ("library"), Some (library), None) => {
			let library = dump_cmark_anchor_mangle_identifier (library);
			succeed! (format! ("library__{}", library));
		},
		(Some (kind), Some (library), Some (entity)) => {
			let kind = match kind {
				"category" => "category",
				"export" => "export",
				"definition" => "definition",
				"value_kind" => "type",
				"appendix" => "appendix",
				_ => fail! (0x69733dab),
			};
			let library = dump_cmark_anchor_mangle_identifier (library);
			let entity = dump_cmark_anchor_mangle_identifier (entity);
			succeed! (format! ("{}__{}__{}", kind, library, entity));
		},
		_ =>
			fail! (0x165bf432),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_title_generate (kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>) -> (Outcome<StdString>) {
	match (kind, library, entity) {
		(Some ("toc"), Some (library), Some (entity)) => {
			let entity = match entity {
				"categories" => "Categories",
				"exports" => "Exports",
				"definitions" => "Definitions",
				"value_kinds" => "Types",
				"appendices" => "Appendices",
				_ => fail! (0x0b252575),
			};
			if title.is_some () {
				fail! (0x1d4d80f6);
			}
			succeed! (format! ("`{}` {}", library, entity));
		},
		(Some ("library"), Some (library), None) => {
			if let Some (title) = title {
				succeed! (format! ("`{}` -- {}", library, title));
			} else {
				succeed! (format! ("`{}` Library", library));
			}
		},
		(Some (kind), Some (library), Some (entity)) => {
			let kind = match kind {
				"category" => "Categories",
				"export" => "Exports",
				"definition" => "Definitions",
				"value_kind" => "Types",
				"appendix" => "Appendices",
				_ => fail! (0xf8250848),
			};
			if let Some (title) = title {
				succeed! (format! ("`{}` -- {}", library, title));
			} else {
				succeed! (format! ("`{}` -- `{}` {}", entity, library, kind));
			}
		},
		_ =>
			fail! (0x165bf432),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_anchor_write (kind : Option<&str>, library : Option<&str>, entity : Option<&str>, configuration : &DumpCmarkGenericConfiguration, stream : &mut impl io::Write) -> (Outcome<()>) {
	if configuration.anchors {
		let anchor = try! (dump_cmark_anchor_generate (kind, library, entity));
		if !configuration.html {
			try_writeln! (stream, "<a id='{}'></a>\n", anchor);
		} else {
			try_writeln! (stream, "<div class='anchor'><a id='{}'></a></div>\n", anchor);
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_title_write (kind : Option<&str>, library : Option<&str>, entity : Option<&str>, title : Option<&str>, _configuration : &DumpCmarkGenericConfiguration, stream : &mut impl io::Write) -> (Outcome<()>) {
	let title = try! (dump_cmark_title_generate (kind, library, entity, title));
	try_writeln! (stream, "# {}", title);
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_format (value : &SchemeValue) -> (StdString) {
	match value.kind () {
		SchemeValueKind::Null =>
			StdString::from ("()"),
		_ => {
			FIXME! ("better handle `#null` case");
			let buffer = format! ("{}", value);
			let buffer = buffer.replace ("#null", "()");
			buffer
		}
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_value_kind_write_tree <'a> (library : &Library, value_kind : &'a ValueKind, value_kinds_seen : &mut StdSet<&'a str>, stream : &mut impl io::Write, recursive_complete : bool, recursive_depth : usize, callbacks : &mut impl DumpCmarkCallbacks, anchor_source : &str) -> (Outcome<()>) {
	let mut stack = StdVec::new ();
	stack.push ((value_kind, true, value_kind.children ()));
	while let Some ((value_kind, emit, sub_value_kinds)) = stack.pop () {
		if emit {
			let seen = if value_kinds_seen.contains (value_kind.identifier ()) {
				if recursive_complete { true } else { continue; }
			} else {
				value_kinds_seen.insert (value_kind.identifier ()); false
			};
			if recursive_complete || !seen {
				let padding = "  " .repeat (stack.len ());
				let value_kind_anchor = try! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), anchor_source));
				let fixes = if recursive_complete && !seen { "**" } else { "" };
				if value_kind.has_children () {
					try_writeln! (stream, "{}* {}[`{}`]({}){}:", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
				} else {
					try_writeln! (stream, "{}* {}[`{}`]({}){};", padding, fixes, value_kind.identifier (), value_kind_anchor, fixes);
				}
				stack.push ((value_kind, false, sub_value_kinds));
			}
		} else {
			let mut sub_value_kinds = sub_value_kinds;
			if (stack.len () + 1) <= recursive_depth {
				if let Some (sub_value_kind) = sub_value_kinds.next () {
					stack.push ((value_kind, false, sub_value_kinds));
					stack.push ((sub_value_kind, true, sub_value_kind.children ()));
				}
			} else if ! sub_value_kinds.is_empty () {
				let padding = "  " .repeat (stack.len () + 1);
				try_writeln! (stream, "{}* ...", padding);
			}
		}
	}
	succeed! (());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_entity_categories_write <'a> (library : &Library, categories_direct : impl iter::ExactSizeIterator<Item = &'a Category>, categories_recursive : impl iter::ExactSizeIterator<Item = &'a Category>, categories_configuration : &DumpCmarkLinkedCategoriesConfiguration, _generic_configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, anchor_source : &str, stream : &mut impl io::Write) -> (Outcome<()>) {
	let categories_direct_count = categories_direct.len ();
	let categories_recursive_count = categories_recursive.len ();
	let mut categories_seen = StdSet::new ();
	if categories_configuration.direct && (categories_direct_count > 0) {
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Categories");
		try_writeln! (stream);
		for category in categories_direct {
			let seen = if categories_seen.contains (category.identifier ()) {
				if categories_configuration.direct_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), anchor_source));
			let fixes = if categories_configuration.direct_complete && !seen { "**" } else { "" };
			if categories_configuration.direct_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
	}
	if categories_configuration.recursive
			&& (categories_configuration.recursive_complete || (categories_recursive_count != categories_direct_count))
	{
		try_writeln! (stream);
		try_writeln! (stream);
		try_writeln! (stream, "#### Categories recursive");
		try_writeln! (stream);
		for category in categories_recursive {
			let seen = if categories_seen.contains (category.identifier ()) {
				if categories_configuration.recursive_complete { true } else { continue; }
			} else {
				categories_seen.insert (category.identifier ()); false
			};
			let category_anchor = try! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), anchor_source));
			let fixes = if categories_configuration.recursive_complete && !seen { "**" } else { "" };
			if categories_configuration.recursive_compact {
				try_writeln! (stream, "{}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			} else {
				try_writeln! (stream, " * {}[`{}`]({}){};", fixes, category.identifier (), category_anchor, fixes);
			}
		}
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_description_write (library : &Library, description : Option<&Description>, links : Option<&Links>, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, anchor_source : &str, stream : &mut impl io::Write) -> (Outcome<()>) {
	let description = if let Some (description) = description {
		description
	} else {
		succeed! (());
	};
	if !configuration.fixme {
		let mut lines = description.lines ();
		if let Some (first) = lines.next () {
			if (first == "**FIXME!**") || (first == "FIXME!") {
				if lines.next () .is_none () {
					succeed! (());
				}
			}
		} else {
			succeed! (());
		}
	}
	let lines_empty = description.lines () .is_empty ();
	if lines_empty && !configuration.lints {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try_writeln! (stream, "#### Description");
	try_writeln! (stream);
	if lines_empty {
		try_writeln! (stream, "> **FIXME!**");
		succeed! (());
	}
	for line in description.lines () {
		let line = borrow::Cow::from (line);
		let line = DUMP_CMARK_CATEGORY_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xe66c9056);
					let identifier = identifier.as_str ();
					if let Some (category) = library.category_resolve (identifier) {
						let category_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("category"), Some (library.identifier ()), Some (category.identifier ()), anchor_source), 0x4a1b437d);
						format! ("[`{}`]({})", category.identifier (), category_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_EXPORT_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xb74a418c);
					let identifier = identifier.as_str ();
					if let Some (export) = library.export_resolve (identifier) {
						let export_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("export"), Some (library.identifier ()), Some (export.identifier ()), anchor_source), 0x038b6c15);
						format! ("[`{}`]({})", export.identifier (), export_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_VALUE_KIND_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0x017ef686);
					let identifier = identifier.as_str ();
					if let Some (value_kind) = library.value_kind_resolve (identifier) {
						let value_kind_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("value_kind"), Some (library.identifier ()), Some (value_kind.identifier ()), anchor_source), 0x438c2cde);
						format! ("[`{}`]({})", value_kind.identifier (), value_kind_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_DEFINITION_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0xe8c3f9f8);
					let identifier = identifier.as_str ();
					if let Some (definition) = library.definition_resolve (identifier) {
						let definition_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("definition"), Some (library.identifier ()), Some (definition.identifier ()), anchor_source), 0xf9025e58);
						format! ("[`{}`]({})", definition.identifier (), definition_anchor)
					} else {
						if configuration.lints {
							format! ("[`{}` **ERROR!**](#errors)", identifier)
						} else {
							format! ("[`{}`](#errors)", identifier)
						}
					}
				});
		let line = DUMP_CMARK_LINK_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0x18c49361);
					let identifier = identifier.as_str ();
					let mut link = None;
					if link.is_none () {
						if let Some (links) = links {
							link = links.link_resolve (identifier);
						}
					}
					if link.is_none () {
						if let Some (links) = library.links () {
							link = links.link_resolve (identifier);
						}
					}
					if let Some (link) = link {
						let link_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("link"), Some (library.identifier ()), Some (link.identifier ()), anchor_source), 0x62baae72);
						format! ("[[{}]]({})", link.identifier (), link_anchor)
					} else {
						//if configuration.lints {
						//	format! ("[[{}] **ERROR!**](#errors)", identifier)
						//	format! ("[[{}]](#errors)", identifier)
						//} else {
							format! ("[[{}]](#errors)", identifier)
						//}
					}
				});
		let line = DUMP_CMARK_APPENDIX_HREF_REGEX.replace_all (&line, |captures : &ext::regex::Captures| {
					let identifier = try_some_or_panic! (captures.get (1), 0x42082eb8);
					let identifier = identifier.as_str ();
					if let Some (appendix) = library.appendix_resolve (identifier) {
						let appendix_anchor = try_or_panic_0! (callbacks.anchor_generate (Some ("appendix"), Some (library.identifier ()), Some (appendix.identifier ()), anchor_source), 0x10a5c400);
						let appendix_label = appendix.title () .unwrap_or_else (|| appendix.identifier ());
						format! ("[\"{}\"]({})", appendix_label, appendix_anchor)
					} else {
						if configuration.lints {
							format! ("[[{}] **ERROR!**](#errors)", identifier)
						} else {
							format! ("[[{}]](#errors)", identifier)
						}
					}
				});
		try_writeln! (stream, "> {}", line);
	}
	succeed! (());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn dump_cmark_links_write (_library : &Library, links : Option<&Links>, configuration : &DumpCmarkGenericConfiguration, _callbacks : &mut impl DumpCmarkCallbacks, _anchor_source : &str, stream : &mut impl io::Write) -> (Outcome<()>) {
	let links = if let Some (links) = links {
		links
	} else {
		succeed! (());
	};
	let links_empty = links.links () .is_empty ();
	if links_empty && !configuration.lints {
		succeed! (());
	}
	try_writeln! (stream);
	try_writeln! (stream);
	try_writeln! (stream, "#### Links");
	try_writeln! (stream);
	if links_empty {
		try_writeln! (stream, "> **FIXME!**");
		succeed! (());
	}
	fail_unimplemented! (0x81cb5f76);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
#[ cfg_attr ( feature = "vonuvoli_lints_clippy", allow (cyclomatic_complexity) ) ]
fn dump_cmark_break_write (library : &Library, configuration : &DumpCmarkGenericConfiguration, callbacks : &mut impl DumpCmarkCallbacks, anchor_source : &str, stream : &mut impl io::Write) -> (Outcome<()>) {
	try_writeln! (stream);
	try_writeln! (stream, "----");
	if configuration.navigator {
		try_writeln! (stream);
		if !configuration.html {
			try_write! (stream, "Goto:");
		} else {
			try_write! (stream, "<div class='navigator'><span class='navigator-header'>Goto:</span>");
		}
		let mut empty = true;
		if configuration.navigator_library {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let library_anchor = try! (callbacks.anchor_generate (Some ("library"), Some (library.identifier ()), None, anchor_source));
			if !configuration.html {
				try_write! (stream, "[library]({})", &library_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>library</a>", &library_anchor);
			}
		}
		if configuration.navigator_categories {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let categories_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("categories"), anchor_source));
			if !configuration.html {
				try_write! (stream, "[categories]({})", &categories_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>categories</a>", &categories_anchor);
			}
		}
		if configuration.navigator_exports {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let exports_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("exports"), anchor_source));
			if !configuration.html {
				try_write! (stream, "[exports]({})", &exports_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>exports</a>", &exports_anchor);
			}
		}
		if configuration.navigator_definitions {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let definitions_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("definitions"), anchor_source));
			if !configuration.html {
				try_write! (stream, "[definitions]({})", &definitions_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>definitions</a>", &definitions_anchor);
			}
		}
		if configuration.navigator_value_kinds {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let value_kinds_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("value_kinds"), anchor_source));
			if !configuration.html {
				try_write! (stream, "[types]({})", &value_kinds_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>types</a>", &value_kinds_anchor);
			}
		}
		if configuration.navigator_appendices {
			if empty { try_write! (stream, " "); empty = false; } else { try_write! (stream, ", "); }
			let appendices_anchor = try! (callbacks.anchor_generate (Some ("toc"), Some (library.identifier ()), Some ("appendices"), anchor_source));
			if !configuration.html {
				try_write! (stream, "[appendices]({})", &appendices_anchor);
			} else {
				try_write! (stream, "<a class='navigator-link' href='{}'>appendices</a>", &appendices_anchor);
			}
		}
		if !empty {
			if !configuration.html {
				try_writeln! (stream, ".");
			} else {
				try_writeln! (stream, ".</div>");
			}
		} else {
			if !configuration.html {
				try_writeln! (stream, " (nothing).");
			} else {
				try_writeln! (stream, " (nothing).</div>");
			}
		}
		try_writeln! (stream);
		try_writeln! (stream, "----");
	}
	try_writeln! (stream);
	succeed! (());
}




pub struct DumpCpioWriter <Writer : io::Write> {
	writer : Writer,
	written_folders : StdSet<fs_path::PathBuf>,
	written_files : StdSet<fs_path::PathBuf>,
	timestamp : u64,
}


impl <Writer : io::Write> DumpCpioWriter<Writer> {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn open (writer : Writer) -> (Outcome<DumpCpioWriter<Writer>>) {
		let writer = DumpCpioWriter {
				writer : writer,
				written_folders : StdSet::new (),
				written_files : StdSet::new (),
				timestamp : try_or_fail! (time::UNIX_EPOCH.elapsed (), 0x943afc5a) .as_secs (),
			};
		succeed! (writer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn close (self) -> (Outcome<(Writer)>) {
		let writer = try_or_fail! (cpio::trailer (self.writer), 0xa81bcf20);
		succeed! (writer);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn write (&mut self, path : &fs_path::Path, data : &[u8]) -> (Outcome<()>) {
		
		if data.len () > (u32::max_value () as usize) {
			fail! (0xcc80443a);
		}
		
		{
			let mut parent = path;
			while let Some (entry_path) = parent.parent () {
				if fs_path::Path::eq (entry_path, fs_path::Path::new ("")) {
					fail! (0xf1cf9cac);
				}
				if fs_path::Path::eq (entry_path, fs_path::Path::new ("..")) {
					fail! (0x90ce4ede);
				}
				if fs_path::Path::eq (entry_path, fs_path::Path::new (".")) {
					break;
				}
				if ! self.written_folders.contains (entry_path) {
					self.written_folders.insert (fs_path::PathBuf::from (entry_path));
					let entry_ino = self.written_files.len () + self.written_folders.len ();
					
					FIXME! ("solve the issue of moving `stream` in and out of the CPIO writer");
					let original_stream : &mut dyn io::Write = unsafe { mem::transmute_copy (&self.writer) };
					
					let entry_stream = cpio::Builder
							::new (try_some_or_panic! (entry_path.to_str (), 0x710bc6c6))
							.mode (0o_040_000 | 0o_000_755)
							.ino (entry_ino as u32)
							.uid (0xfffe)
							.gid (0xfffe)
							.mtime (self.timestamp as u32)
							.nlink (1)
							.write (original_stream, 0);
					
					try_or_fail! (entry_stream.finish (), 0xc63d28a0);
				}
				parent = entry_path;
			}
		}
		
		{
			let entry_path = path;
			if ! self.written_files.insert (fs_path::PathBuf::from (entry_path)) {
				fail! (0x94276b39);
			}
			let entry_ino = self.written_files.len () + self.written_folders.len ();
			
			FIXME! ("solve the issue of moving `stream` in and out of the CPIO writer");
			let original_stream : &mut dyn io::Write = unsafe { mem::transmute_copy (&self.writer) };
			
			let mut entry_stream = cpio::Builder
					::new (try_some_or_panic! (entry_path.to_str (), 0x710bc6c6))
					.mode (0o_100_000 | 0o_000_644)
					.ino (entry_ino as u32)
					.uid (0xfffe)
					.gid (0xfffe)
					.mtime (self.timestamp as u32)
					.nlink (1)
					.write (original_stream, data.len () as u32);
			
			try_or_fail! (entry_stream.write_all (data), 0x52251862);
			
			try_or_fail! (entry_stream.finish (), 0xb0be881c);
		}
		
		succeed! (());
	}
}




lazy_static! {
	static ref DUMP_CMARK_CATEGORY_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#category\)"), 0x7a74ab93);
	static ref DUMP_CMARK_EXPORT_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#export\)"), 0x61f706ce);
	static ref DUMP_CMARK_VALUE_KIND_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#types\)"), 0x93297fed);
	static ref DUMP_CMARK_DEFINITION_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[`([^`]+)`\]\(#definitions\)"), 0x0e6d98c5);
	static ref DUMP_CMARK_LINK_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[\[([a-zA-Z0-9_-]+)\]\]\(#links\)"), 0xe10a7e4c);
	static ref DUMP_CMARK_APPENDIX_HREF_REGEX : ext::regex::Regex = try_or_panic_0! (ext::regex::Regex::new (r"\[\[([a-zA-Z0-9_-]+)\]\]\(#appendices\)"), 0x039d98a7);
}




static DUMP_HTML_PREFIX : &str =
r####"<!DOCTYPE html>
<html>
<head>
	<meta charset="UTF-8">
	<title>@{title}</title>
</head>
<body>
"####;


static DUMP_HTML_SUFFIX : &str =
r####"</body>
</html>
"####;


static DUMP_HTML_CSS : &str = include_str! ("../documentation/libraries.css");

