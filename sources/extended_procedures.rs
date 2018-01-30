

use super::builtins::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ProcedureExtended;
	pub use super::ProcedureExtendedInternals;
	
	pub use super::procedure_extended_evaluate_0;
	pub use super::procedure_extended_evaluate_1;
	pub use super::procedure_extended_evaluate_2;
	pub use super::procedure_extended_evaluate_3;
	pub use super::procedure_extended_evaluate_4;
	pub use super::procedure_extended_evaluate_5;
	pub use super::procedure_extended_evaluate_n;
	
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct ProcedureExtended ( StdRc<ProcedureExtendedInternals> );


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ProcedureExtendedInternals {
	
	ComposedPrimitive1 (StdBox<[ProcedurePrimitive1]>),
	
	RecordKindIs (RecordKind),
	RecordBuild (RecordKind, Option<StdBox<[usize]>>, Option<bool>),
	RecordGet (RecordKind, usize),
	RecordSet (RecordKind, usize),
	
}


impl ProcedureExtended {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (internals : ProcedureExtendedInternals) -> (ProcedureExtended) {
		return ProcedureExtended (StdRc::new (internals));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ProcedureExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ProcedureExtended) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_0 (extended : &ProcedureExtended, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_0 (kind, option_box_as_ref (fields), immutable),
		
		_ =>
			fail! (0x9507fccd),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_1 (extended : &ProcedureExtended, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::ComposedPrimitive1 (ref primitives) => {
			// FIXME:  Extract this!
			let primitives = primitives.as_ref ();
			if primitives.is_empty () {
				fail! (0x3ba06e9c);
			}
			let mut value = input_1.clone ();
			for primitive in primitives.iter () .rev () {
				value = try! (procedure_primitive_1_evaluate (*primitive, &value, evaluator));
			}
			succeed! (value);
		}
		
		ProcedureExtendedInternals::RecordKindIs (ref kind) =>
			return record_kind_is (kind, input_1) .into_0 (),
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_1 (kind, option_box_as_ref (fields), input_1, immutable),
		
		ProcedureExtendedInternals::RecordGet (ref kind, field) =>
			return record_get (kind, field, input_1),
		
		_ =>
			fail! (0x224ed4b5),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_2 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_2 (kind, option_box_as_ref (fields), input_1, input_2, immutable),
		
		ProcedureExtendedInternals::RecordSet (ref kind, field) =>
			return record_set (kind, field, input_1, input_2),
		
		_ =>
			fail! (0x786569ea),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_3 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_3 (kind, option_box_as_ref (fields), input_1, input_2, input_3, immutable),
		
		_ =>
			fail! (0x3a0174c2),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_4 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_4 (kind, option_box_as_ref (fields), input_1, input_2, input_3, input_4, immutable),
		
		_ =>
			fail! (0x25d23c58),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_5 (extended : &ProcedureExtended, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match *extended.internals_ref () {
		
		ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable) =>
			return record_build_n (kind, option_box_as_ref (fields), &[input_1, input_2, input_3, input_4, input_5], immutable),
		
		_ =>
			fail! (0x80e07b4f),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn procedure_extended_evaluate_n (extended : &ProcedureExtended, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	let inputs_count = inputs.len ();
	match (inputs_count, extended.internals_ref ()) {
		
		(1, &ProcedureExtendedInternals::ComposedPrimitive1 (_)) =>
			// FIXME:  Extract this!
			return procedure_extended_evaluate_1 (extended, inputs[0], evaluator),
		
		(1, &ProcedureExtendedInternals::RecordGet (ref kind, field)) =>
			return record_get (kind, field, inputs[0]),
		
		(2, &ProcedureExtendedInternals::RecordSet (ref kind, field)) =>
			return record_set (kind, field, inputs[0], inputs[1]),
		
		(_, &ProcedureExtendedInternals::RecordBuild (ref kind, ref fields, immutable)) =>
			return record_build_n (kind, option_box_as_ref (fields), inputs, immutable),
		
		_ =>
			fail! (0x7b179cf1),
		
	}
}

