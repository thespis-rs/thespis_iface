use crate::{ import::*, * } ;

/// A unique ID that can be used to globally identify a service in the network. The user
/// will provide the concrete type and usually implementations for peers and routers can
/// work with a trait object thanks to the bounds.
///
pub trait UniqueID :

	Into< Bytes >    +
	TryFrom< Bytes > +
	Debug            +
	Display          +
	Clone            +
	PartialEq        +
	Eq               +
	Hash             +

{
	/// Seed the uniqueID. It might be data that will be hashed to generate the id.
	/// An identical input here should always give an identical UniqueID.
	//
	fn from_seed( data: &[u8] ) -> Self;


	/// And empty UniqueID. Can be used to signify the abscence of an id, would usually be all
	/// zero bytes.
	//
	fn null() -> Self;


	/// Predicate for null values.
	//
	fn is_null( &self ) -> bool;
}
