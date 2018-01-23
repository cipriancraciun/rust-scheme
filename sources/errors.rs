

use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::Outcome;
	pub use super::Error;
	
	pub use super::error_generic;
	pub use super::error_unimplemented;
	pub use super::error_panic;
	
}




pub type Outcome<T> = Result<T, Error>;




#[ derive (Clone) ]
pub struct Error ( StdRc<ErrorInternals> );

pub enum ErrorInternals {
	Code (u64),
	WithMessage (Option<u64>, StdRc<StdBox<str>>),
	WithMessageAndArguments (Option<u64>, StdRc<StdBox<str>>, StdRc<StdBox<[Value]>>),
}


impl Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (code : u64) -> (Error) {
		let internals = ErrorInternals::Code (code);
		Error (StdRc::new (internals))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_with_message (code : Option<u64>, message : StdRc<StdBox<str>>) -> (Error) {
		let internals = ErrorInternals::WithMessage (code, message);
		Error (StdRc::new (internals))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new_with_message_and_arguments (code : Option<u64>, message : StdRc<StdBox<str>>, arguments : StdRc<StdBox<[Value]>>) -> (Error) {
		let internals = ErrorInternals::WithMessageAndArguments (code, message, arguments);
		Error (StdRc::new (internals))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code (&self) -> (u64) {
		match *self.internals_ref () {
			ErrorInternals::Code (code) =>
				code,
			ErrorInternals::WithMessage (code, _) =>
				code.unwrap_or (0x0000000000000000),
			ErrorInternals::WithMessageAndArguments (code, _, _) =>
				code.unwrap_or (0x0000000000000000),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn message (&self) -> (Option<&str>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_) =>
				None,
			ErrorInternals::WithMessage (_, ref message) =>
				Some (message.as_ref ()),
			ErrorInternals::WithMessageAndArguments (_, ref message, _) =>
				Some (message.as_ref ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn message_clone (&self) -> (Option<StringImmutable>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_) =>
				None,
			ErrorInternals::WithMessage (_, ref message) =>
				Some (StringImmutable::clone_rc (message)),
			ErrorInternals::WithMessageAndArguments (_, ref message, _) =>
				Some (StringImmutable::clone_rc (message)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments (&self) -> (Option<&[Value]>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_) =>
				None,
			ErrorInternals::WithMessage (_, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments) =>
				Some (arguments.as_ref ()),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments_clone_array (&self) -> (Option<ArrayImmutable>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_) =>
				None,
			ErrorInternals::WithMessage (_, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments) =>
				Some (ArrayImmutable::clone_rc (arguments)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn arguments_clone_values (&self) -> (Option<Values>) {
		match *self.internals_ref () {
			ErrorInternals::Code (_) =>
				None,
			ErrorInternals::WithMessage (_, _) =>
				None,
			ErrorInternals::WithMessageAndArguments (_, _, ref arguments) =>
				Some (Values::clone_rc (arguments)),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn code_2 (&self) -> (u32, u32) {
		let code = self.code ();
		let code_1 = ((code & 0xffffffff00000000) >> 32) as u32;
		let code_2 = ((code & 0x00000000ffffffff) >> 0) as u32;
		(code_1, code_2)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ErrorInternals) {
		self.0.as_ref ()
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Error) -> (bool) {
		let self_code = self.code ();
		let other_code = other.code ();
		self_code == other_code
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_generic (code : u32) -> (Error) {
	Error::new (code as u64)
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_unimplemented (code : u32) -> (Error) {
	Error::new (code as u64)
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn error_panic (code : u32) -> (Error) {
	Error::new (code as u64)
}




impl StdFrom<convert::Infallible> for Error {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn from (_error : convert::Infallible) -> (Error) {
		return error_panic (0xddde3965);
	}
}

