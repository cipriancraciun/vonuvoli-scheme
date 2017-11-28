

use super::constants::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::BitwisePrimitive0;
	pub use super::BitwisePrimitive1;
	pub use super::BitwisePrimitive2;
	pub use super::BitwisePrimitive3;
	pub use super::BitwisePrimitive4;
	pub use super::BitwisePrimitiveN;
	
	pub use super::bitwise_primitive_0_evaluate;
	pub use super::bitwise_primitive_1_evaluate;
	pub use super::bitwise_primitive_2_evaluate;
	pub use super::bitwise_primitive_3_evaluate;
	pub use super::bitwise_primitive_4_evaluate;
	pub use super::bitwise_primitive_n_evaluate;
	
	pub use super::bitwise_primitive_n_alternative_0;
	pub use super::bitwise_primitive_n_alternative_1;
	pub use super::bitwise_primitive_n_alternative_2;
	pub use super::bitwise_primitive_n_alternative_3;
	pub use super::bitwise_primitive_n_alternative_4;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive0 {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive1 {
	
	Complement,
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive2 {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
	ShiftLeft,
	ShiftRight,
	
	RotateLeft,
	RotateRight,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum BitwisePrimitiveN {
	
	And,
	Or,
	Xor,
	
	Nand,
	Nor,
	Nxor,
	
}




pub fn bitwise_primitive_0_evaluate (primitive : BitwisePrimitive0) -> (Outcome<Value>) {
	
	let output = match primitive {
		
		BitwisePrimitive0::And =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Or =>
			ZERO.into (),
		
		BitwisePrimitive0::Xor =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Nand =>
			ZERO.into (),
		
		BitwisePrimitive0::Nor =>
			ZERO.bitnot (),
		
		BitwisePrimitive0::Nxor =>
			ZERO.into (),
		
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_1_evaluate (primitive : BitwisePrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	
	let input_1 = try_as_number_integer_ref! (input_1);
	
	let output = match primitive {
		
		BitwisePrimitive1::Complement =>
			input_1.bitnot (),
		
		BitwisePrimitive1::And =>
			input_1.clone (),
		
		BitwisePrimitive1::Or =>
			input_1.clone (),
		
		BitwisePrimitive1::Xor =>
			ZERO.into (),
		
		BitwisePrimitive1::Nand =>
			input_1.bitnot (),
		
		BitwisePrimitive1::Nor =>
			input_1.bitnot (),
		
		BitwisePrimitive1::Nxor =>
			ZERO.bitnot (),
		
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_2_evaluate (primitive : BitwisePrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	
	let input_1 = try_as_number_integer_ref! (input_1);
	let input_2 = try_as_number_integer_ref! (input_2);
	
	let output = match primitive {
		
		BitwisePrimitive2::And =>
			input_1.bitand (input_2),
		
		BitwisePrimitive2::Or =>
			input_1.bitor (input_2),
		
		BitwisePrimitive2::Xor =>
			input_1.bitxor (input_2),
		
		BitwisePrimitive2::Nand =>
			input_1.bitnand (input_2),
		
		BitwisePrimitive2::Nor =>
			input_1.bitnor (input_2),
		
		BitwisePrimitive2::Nxor =>
			input_1.bitnxor (input_2),
		
		BitwisePrimitive2::ShiftLeft =>
			try! (input_1.shl (input_2)),
		
		BitwisePrimitive2::ShiftRight =>
			try! (input_1.shr (input_2)),
		
		BitwisePrimitive2::RotateLeft =>
			try! (input_1.rotate_left (input_2)),
		
		BitwisePrimitive2::RotateRight =>
			try! (input_1.rotate_right (input_2)),
		
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_3_evaluate (primitive : BitwisePrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn bitwise_primitive_4_evaluate (primitive : BitwisePrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value) -> (Outcome<Value>) {
	match primitive {}
}




pub fn bitwise_primitive_n_evaluate (primitive : BitwisePrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	
	let mut output = match primitive {
		
		BitwisePrimitiveN::And | BitwisePrimitiveN::Nand =>
			(<u64>::max_value () as i64).into (),
		
		BitwisePrimitiveN::Or | BitwisePrimitiveN::Nor =>
			ZERO,
		
		BitwisePrimitiveN::Xor | BitwisePrimitiveN::Nxor =>
			ZERO,
		
	};
	
	for input in inputs {
		let input = try_as_number_integer_ref! (input);
		
		output = match primitive {
			
			BitwisePrimitiveN::And | BitwisePrimitiveN::Nand =>
				output.bitand (input),
			
			BitwisePrimitiveN::Or | BitwisePrimitiveN::Nor =>
				output.bitor (input),
			
			BitwisePrimitiveN::Xor | BitwisePrimitiveN::Nxor =>
				output.bitxor (input),
			
		}
	}
	
	output = match primitive {
		
		BitwisePrimitiveN::And | BitwisePrimitiveN::Or =>
			output,
		
		BitwisePrimitiveN::Nand | BitwisePrimitiveN::Nor =>
			output.bitnot (),
		
		BitwisePrimitiveN::Xor =>
			output.bitxor (&ZERO),
		
		BitwisePrimitiveN::Nxor =>
			output.bitxor (&ZERO) .bitnot (),
		
	};
	
	succeed! (output.into ());
}




pub fn bitwise_primitive_n_alternative_0 (primitive : BitwisePrimitiveN) -> (Option<BitwisePrimitive0>) {
	match primitive {
		_ => None,
	}
}


pub fn bitwise_primitive_n_alternative_1 (primitive : BitwisePrimitiveN) -> (Option<BitwisePrimitive1>) {
	match primitive {
		_ => None,
	}
}


pub fn bitwise_primitive_n_alternative_2 (primitive : BitwisePrimitiveN) -> (Option<BitwisePrimitive2>) {
	match primitive {
		_ => None,
	}
}


pub fn bitwise_primitive_n_alternative_3 (primitive : BitwisePrimitiveN) -> (Option<BitwisePrimitive3>) {
	match primitive {
		_ => None,
	}
}


pub fn bitwise_primitive_n_alternative_4 (primitive : BitwisePrimitiveN) -> (Option<BitwisePrimitive4>) {
	match primitive {
		_ => None,
	}
}


