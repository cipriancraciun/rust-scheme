

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxExtended;
	pub use super::SyntaxExtendedInternals;
	
}




#[ derive (Clone, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct SyntaxExtended ( StdRc<SyntaxExtendedInternals> );


#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum SyntaxExtendedInternals {}


impl SyntaxExtended {
	
	#[ inline (always) ]
	pub fn new (internals : SyntaxExtendedInternals) -> (SyntaxExtended) {
		return SyntaxExtended (StdRc::new (internals));
	}
	
	#[ inline (always) ]
	pub fn internals_ref (&self) -> (&SyntaxExtendedInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ inline (always) ]
	pub fn is_self (&self, other : &SyntaxExtended) -> (bool) {
		ptr::eq (self.0.as_ref (), other.0.as_ref ())
	}
}

