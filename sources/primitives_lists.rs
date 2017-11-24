

use super::constants::exports::*;
use super::builtins::exports::*;
use super::errors::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::ListPrimitive0;
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitive3;
	pub use super::ListPrimitive4;
	pub use super::ListPrimitiveN;
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive0 {
	
	ListBuild,
	ListAppend,
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive1 {
	
	PairLeft,
	PairRight,
	
	ListFirstOfFirst,
	ListRestOfFirst,
	
	ListFirstAt2,
	ListRestAt2,
	
	ListLength,
	ListClone,
	ListReverse,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	PairLeftSet,
	PairRightSet,
	
	ListPairAt,
	ListFirstAt,
	ListRestAt,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive3 {
	
	ListFirstAtSet,
	ListRestAtSet,
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitive4 {
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListPrimitiveN {
	
	ListMake,
	ListBuild,
	ListAppend,
	
}




pub fn list_primitive_0_evaluate (primitive : ListPrimitive0) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive0::ListBuild =>
			succeed! (NULL.into ()),
		
		ListPrimitive0::ListAppend =>
			succeed! (NULL.into ()),
		
	}
}




pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input_1 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive1::PairLeft =>
			return list_first (input_1),
		
		ListPrimitive1::PairRight =>
			return list_rest (input_1),
		
		ListPrimitive1::ListFirstAt2 =>
			return list_first_at (input_1, 1),
		
		ListPrimitive1::ListRestAt2 =>
			return list_rest_at (input_1, 1),
		
		ListPrimitive1::ListFirstOfFirst =>
			return list_first (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListRestOfFirst =>
			return list_rest (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListLength => {
			let length = try! (list_length (input_1));
			let length : NumberInteger = try! (StdTryFrom::try_from (length));
			succeed! (length.into ());
		},
		
		ListPrimitive1::ListClone =>
			return list_clone (input_1),
		
		ListPrimitive1::ListReverse =>
			return list_reverse (input_1),
		
		ListPrimitive1::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &UNDEFINED.into ()),
		
		ListPrimitive1::ListBuild =>
			succeed! (list_build_1 (input_1)),
		
		ListPrimitive1::ListAppend =>
			succeed! (input_1.clone ()),
		
	}
}




pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive2::Pair =>
			succeed! (pair (input_1, input_2)),
		
		ListPrimitive2::PairLeftSet =>
			return pair_left_set (input_1, input_2),
		
		ListPrimitive2::PairRightSet =>
			return pair_right_set (input_1, input_2),
		
		ListPrimitive2::ListPairAt =>
			return list_pair_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListFirstAt =>
			return list_first_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListRestAt =>
			return list_rest_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2),
		
		ListPrimitive2::ListBuild =>
			succeed! (list_build_2 (input_1, input_2)),
		
		ListPrimitive2::ListAppend =>
			return list_append_2 (input_1, input_2),
		
	}
}




pub fn list_primitive_3_evaluate (primitive : ListPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive3::ListFirstAtSet =>
			return list_first_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListRestAtSet =>
			return list_rest_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListBuild =>
			succeed! (list_build_3 (input_1, input_2, input_3)),
		
		ListPrimitive3::ListAppend =>
			return list_append_3 (input_1, input_2, input_3),
		
	}
}




pub fn list_primitive_4_evaluate (primitive : ListPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive4::ListBuild =>
			succeed! (list_build_4 (input_1, input_2, input_3, input_4)),
		
		ListPrimitive4::ListAppend =>
			return list_append_4 (input_1, input_2, input_3, input_4),
		
	}
}




pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[Value]) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match primitive {
		
		ListPrimitiveN::ListMake =>
			match inputs_count {
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListMake, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListMake, &inputs[0], &inputs[1]),
				_ =>
					fail! (0xdd5940d5),
			},
		
		ListPrimitiveN::ListBuild =>
			match inputs_count {
				0 =>
					return list_primitive_0_evaluate (ListPrimitive0::ListBuild),
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListBuild, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListBuild, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListBuild, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListBuild, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					succeed! (list_build_n (inputs)),
			},
		
		ListPrimitiveN::ListAppend =>
			match inputs_count {
				0 =>
					return list_primitive_0_evaluate (ListPrimitive0::ListAppend),
				1 =>
					return list_primitive_1_evaluate (ListPrimitive1::ListAppend, &inputs[0]),
				2 =>
					return list_primitive_2_evaluate (ListPrimitive2::ListAppend, &inputs[0], &inputs[1]),
				3 =>
					return list_primitive_3_evaluate (ListPrimitive3::ListAppend, &inputs[0], &inputs[1], &inputs[2]),
				4 =>
					return list_primitive_4_evaluate (ListPrimitive4::ListAppend, &inputs[0], &inputs[1], &inputs[2], &inputs[3]),
				_ =>
					return list_append_n (inputs),
			},
		
	}
}

