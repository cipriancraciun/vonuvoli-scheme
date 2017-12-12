

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::StringPrimitive0;
	pub use super::StringPrimitive1;
	pub use super::StringPrimitive2;
	pub use super::StringPrimitive3;
	pub use super::StringPrimitive4;
	pub use super::StringPrimitive5;
	pub use super::StringPrimitiveN;
	pub use super::StringPrimitiveV;
	
	pub use super::string_primitive_0_evaluate;
	pub use super::string_primitive_1_evaluate;
	pub use super::string_primitive_2_evaluate;
	pub use super::string_primitive_3_evaluate;
	pub use super::string_primitive_4_evaluate;
	pub use super::string_primitive_5_evaluate;
	pub use super::string_primitive_n_evaluate;
	
	pub use super::string_primitive_v_alternative_0;
	pub use super::string_primitive_v_alternative_1;
	pub use super::string_primitive_v_alternative_2;
	pub use super::string_primitive_v_alternative_3;
	pub use super::string_primitive_v_alternative_4;
	pub use super::string_primitive_v_alternative_5;
	pub use super::string_primitive_v_alternative_n;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive0 {
	
	StringBuild,
	StringAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive1 {
	
	StringLength,
	StringClone,
	StringReverse,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	
	StringToList,
	ListToString,
	StringToArray,
	ArrayToString,
	StringToBytes,
	BytesToString,
	
	StringToSymbol,
	SymbolToString,
	StringToNumber,
	NumberToString,
	CharacterToNumber,
	NumberToCharacter,
	
	StringToUpperCase,
	StringToLowerCase,
	StringToFoldCase,
	SymbolToUpperCase,
	SymbolToLowerCase,
	SymbolToFoldCase,
	CharacterToUpperCase,
	CharacterToLowerCase,
	CharacterToFoldCase,
	CharacterToDigitNumber,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive2 {
	
	StringAt,
	
	StringMake,
	
	StringBuild,
	StringAppend,
	
	StringFill,
	StringCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive3 {
	
	StringAtSet,
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	NumberToString,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive4 {
	
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitive5 {
	
	StringRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitiveN {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum StringPrimitiveV {
	
	StringMake,
	StringBuild,
	StringAppend,
	
	StringRangeFill,
	StringRangeCopy,
	StringRangeClone,
	
	StringRangeToList,
	ListRangeToString,
	StringRangeToArray,
	ArrayRangeToString,
	StringRangeToBytes,
	BytesRangeToString,
	
	StringToNumber,
	NumberToString,
	CharacterToDigitNumber,
	
}




pub fn string_primitive_0_evaluate (primitive : StringPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive0::StringBuild =>
			succeed! (string_empty ()),
		
		StringPrimitive0::StringAppend =>
			succeed! (string_empty ()),
		
	}
}




pub fn string_primitive_1_evaluate (primitive : StringPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive1::StringLength => {
			let length = try! (string_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		StringPrimitive1::StringClone =>
			return string_clone (input_1),
		
		StringPrimitive1::StringReverse =>
			return string_reverse (input_1),
		
		StringPrimitive1::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), None),
		
		StringPrimitive1::StringBuild =>
			return string_build_1 (input_1),
		
		StringPrimitive1::StringAppend =>
			return string_clone (input_1),
		
		StringPrimitive1::StringFill =>
			return string_fill_range (input_1, None, None, None),
		
		StringPrimitive1::StringToList =>
			return string_range_to_list (input_1, None, None),
		
		StringPrimitive1::ListToString =>
			return list_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToArray =>
			return string_range_to_array (input_1, None, None),
		
		StringPrimitive1::ArrayToString =>
			return array_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToBytes =>
			return string_range_to_bytes (input_1, None, None),
		
		StringPrimitive1::BytesToString =>
			return bytes_range_to_string (input_1, None, None),
		
		StringPrimitive1::StringToSymbol =>
			return string_to_symbol (input_1),
		
		StringPrimitive1::SymbolToString =>
			return symbol_to_string (input_1),
		
		StringPrimitive1::StringToNumber =>
			return string_to_number (input_1, None),
		
		StringPrimitive1::NumberToString =>
			return number_to_string (input_1, None, None),
		
		StringPrimitive1::CharacterToNumber =>
			return character_to_number (input_1),
		
		StringPrimitive1::NumberToCharacter =>
			return number_to_character (input_1),
		
		StringPrimitive1::StringToUpperCase =>
			return string_to_upper_case (input_1),
		
		StringPrimitive1::StringToLowerCase =>
			return string_to_lower_case (input_1),
		
		StringPrimitive1::StringToFoldCase =>
			return string_to_fold_case (input_1),
		
		StringPrimitive1::SymbolToUpperCase =>
			return symbol_to_upper_case (input_1),
		
		StringPrimitive1::SymbolToLowerCase =>
			return symbol_to_lower_case (input_1),
		
		StringPrimitive1::SymbolToFoldCase =>
			return symbol_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToUpperCase =>
			return character_to_upper_case (input_1),
		
		StringPrimitive1::CharacterToLowerCase =>
			return character_to_lower_case (input_1),
		
		StringPrimitive1::CharacterToFoldCase =>
			return character_to_fold_case (input_1),
		
		StringPrimitive1::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, None),
		
	}
}




pub fn string_primitive_2_evaluate (primitive : StringPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive2::StringAt =>
			return string_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		StringPrimitive2::StringMake =>
			return string_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), Some (input_2)),
		
		StringPrimitive2::StringBuild =>
			return string_build_2 (input_1, input_2),
		
		StringPrimitive2::StringAppend =>
			return string_append_2 (input_1, input_2),
		
		StringPrimitive2::StringFill =>
			return string_fill_range (input_1, Some (input_2), None, None),
		
		StringPrimitive2::StringCopy =>
			return string_copy_range (input_1, None, input_2, None, None),
		
		StringPrimitive2::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), None),
		
		StringPrimitive2::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), None),
		
		StringPrimitive2::ArrayRangeToString =>
			return array_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), None),
		
		StringPrimitive2::BytesRangeToString =>
			return bytes_range_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::StringToNumber =>
			return string_to_number (input_1, Some (input_2)),
		
		StringPrimitive2::NumberToString =>
			return number_to_string (input_1, Some (input_2), None),
		
		StringPrimitive2::CharacterToDigitNumber =>
			return character_to_digit_number (input_1, Some (input_2)),
		
	}
}




pub fn string_primitive_3_evaluate (primitive : StringPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive3::StringAtSet =>
			return string_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		StringPrimitive3::StringBuild =>
			return string_build_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringAppend =>
			return string_append_3 (input_1, input_2, input_3),
		
		StringPrimitive3::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		StringPrimitive3::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, None, None),
		
		StringPrimitive3::StringRangeClone =>
			return string_clone_range (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeToList =>
			return string_range_to_list (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::ListRangeToString =>
			return list_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeToArray =>
			return string_range_to_array (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::ArrayRangeToString =>
			return array_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::StringRangeToBytes =>
			return string_range_to_bytes (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::BytesRangeToString =>
			return bytes_range_to_string (input_1, Some (input_2), Some (input_3)),
		
		StringPrimitive3::NumberToString =>
			return number_to_string (input_1, Some (input_2), Some (try_as_boolean_ref! (input_3) .value ())),
		
	}
}




pub fn string_primitive_4_evaluate (primitive : StringPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive4::StringBuild =>
			return string_build_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringAppend =>
			return string_append_4 (input_1, input_2, input_3, input_4),
		
		StringPrimitive4::StringRangeFill =>
			return string_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		StringPrimitive4::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




pub fn string_primitive_5_evaluate (primitive : StringPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitive5::StringRangeCopy =>
			return string_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




pub fn string_primitive_n_evaluate (primitive : StringPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		StringPrimitiveN::StringMake =>
			fail_panic! (0x81290cad),
		
		StringPrimitiveN::StringBuild =>
			return string_build_n (inputs),
		
		StringPrimitiveN::StringAppend =>
			return string_append_n (inputs),
		
		StringPrimitiveN::StringRangeFill =>
			fail_panic! (0x04d2afc0),
		
		StringPrimitiveN::StringRangeCopy =>
			fail_panic! (0x8c5e5181),
		
		StringPrimitiveN::StringRangeClone =>
			fail_panic! (0x0d49ddab),
		
		StringPrimitiveN::StringRangeToList =>
			fail_panic! (0x273584ff),
		
		StringPrimitiveN::ListRangeToString =>
			fail_panic! (0xd0a9123a),
		
		StringPrimitiveN::StringRangeToArray =>
			fail_panic! (0x091d3683),
		
		StringPrimitiveN::ArrayRangeToString =>
			fail_panic! (0x5d6d69b0),
		
		StringPrimitiveN::StringRangeToBytes =>
			fail_panic! (0xe7f5f988),
		
		StringPrimitiveN::BytesRangeToString =>
			fail_panic! (0xa4900e52),
		
		StringPrimitiveN::StringToNumber =>
			fail_panic! (0x3fefec61),
		
		StringPrimitiveN::NumberToString =>
			fail_panic! (0x8c1917bf),
		
		StringPrimitiveN::CharacterToDigitNumber =>
			fail_panic! (0x49dc7b05),
		
	}
}




pub fn string_primitive_v_alternative_0 (primitive : StringPrimitiveV) -> (Option<StringPrimitive0>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive0::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive0::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			None,
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		StringPrimitiveV::StringRangeToArray =>
			None,
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


pub fn string_primitive_v_alternative_1 (primitive : StringPrimitiveV) -> (Option<StringPrimitive1>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			Some (StringPrimitive1::StringMake),
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive1::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive1::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive1::StringFill),
		StringPrimitiveV::StringRangeCopy =>
			None,
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive1::StringClone),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive1::StringToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive1::ListToString),
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive1::StringToArray),
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive1::ArrayToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive1::StringToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive1::BytesToString),
		StringPrimitiveV::StringToNumber =>
			Some (StringPrimitive1::StringToNumber),
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive1::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			Some (StringPrimitive1::CharacterToDigitNumber),
	}
}


pub fn string_primitive_v_alternative_2 (primitive : StringPrimitiveV) -> (Option<StringPrimitive2>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			Some (StringPrimitive2::StringMake),
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive2::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive2::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive2::StringFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive2::StringCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive2::StringRangeClone),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive2::StringRangeToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive2::ListRangeToString),
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive2::StringRangeToArray),
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive2::ArrayRangeToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive2::StringRangeToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive2::BytesRangeToString),
		StringPrimitiveV::StringToNumber =>
			Some (StringPrimitive2::StringToNumber),
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive2::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			Some (StringPrimitive2::CharacterToDigitNumber),
	}
}


pub fn string_primitive_v_alternative_3 (primitive : StringPrimitiveV) -> (Option<StringPrimitive3>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive3::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive3::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive3::StringRangeFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive3::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			Some (StringPrimitive3::StringRangeClone),
		StringPrimitiveV::StringRangeToList =>
			Some (StringPrimitive3::StringRangeToList),
		StringPrimitiveV::ListRangeToString =>
			Some (StringPrimitive3::ListRangeToString),
		StringPrimitiveV::StringRangeToArray =>
			Some (StringPrimitive3::StringRangeToArray),
		StringPrimitiveV::ArrayRangeToString =>
			Some (StringPrimitive3::ArrayRangeToString),
		StringPrimitiveV::StringRangeToBytes =>
			Some (StringPrimitive3::StringRangeToBytes),
		StringPrimitiveV::BytesRangeToString =>
			Some (StringPrimitive3::BytesRangeToString),
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			Some (StringPrimitive3::NumberToString),
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


pub fn string_primitive_v_alternative_4 (primitive : StringPrimitiveV) -> (Option<StringPrimitive4>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			Some (StringPrimitive4::StringBuild),
		StringPrimitiveV::StringAppend =>
			Some (StringPrimitive4::StringAppend),
		StringPrimitiveV::StringRangeFill =>
			Some (StringPrimitive4::StringRangeFill),
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive4::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		StringPrimitiveV::StringRangeToArray =>
			None,
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


pub fn string_primitive_v_alternative_5 (primitive : StringPrimitiveV) -> (Option<StringPrimitive5>) {
	match primitive {
		StringPrimitiveV::StringMake =>
			None,
		StringPrimitiveV::StringBuild =>
			None,
		StringPrimitiveV::StringAppend =>
			None,
		StringPrimitiveV::StringRangeFill =>
			None,
		StringPrimitiveV::StringRangeCopy =>
			Some (StringPrimitive5::StringRangeCopy),
		StringPrimitiveV::StringRangeClone =>
			None,
		StringPrimitiveV::StringRangeToList =>
			None,
		StringPrimitiveV::ListRangeToString =>
			None,
		StringPrimitiveV::StringRangeToArray =>
			None,
		StringPrimitiveV::ArrayRangeToString =>
			None,
		StringPrimitiveV::StringRangeToBytes =>
			None,
		StringPrimitiveV::BytesRangeToString =>
			None,
		StringPrimitiveV::StringToNumber =>
			None,
		StringPrimitiveV::NumberToString =>
			None,
		StringPrimitiveV::CharacterToDigitNumber =>
			None,
	}
}


pub fn string_primitive_v_alternative_n (primitive : StringPrimitiveV) -> (Option<StringPrimitiveN>) {
	match primitive {
		_ =>
			None,
	}
}

