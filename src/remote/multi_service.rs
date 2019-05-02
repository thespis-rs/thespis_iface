use crate::{ import::*, * } ;


/// Represents a message as it is over the wire, with some metadata attached to the serialized `Message`.
///
/// The actual wire format is not defined here, but is implementation dependent. This seems bad for interop,
/// but it is actually not conceivalbe right now to define a single format and force that on all users. We
/// might actually want to implement a cap'n proto version to make it more compatible with other languages.
///
/// Different tradeoffs might require different layout. CPU vs Memory consumption for example.
/// Maybe a few default formats could be suggested later. Like a "compact" format and a "fast" format.
///
/// The Into and From Bytes are not mutable, implementations are supposed to just
/// wrap them without copying anything. Providing a view into the memory through the trait methods.
//
pub trait MultiService : Into< Bytes > + From< Bytes > + Send + Sync
{
	type ServiceID: UniqueID;
	type ConnID   : UniqueID;
	type CodecAlg : CodecAlg;

	/// Create a new MultiService. I don't like to put constructors in a trait, but for now we need it in
	/// our impl of Peer and service_map_macro.
	//
	fn create
	(
		service : Self::ServiceID ,
		conn_id : Self::ConnID    ,
		encoding: Self::CodecAlg  ,
		mesg    : Bytes           ,

	) -> Self

		where Self: Sized
	;

	/// The total length of the Multiservice in Bytes (header+payload)
	//
	fn len( &self ) -> usize;

	/// The service id of this message. When coming in over the wire, this identifies
	/// which service you are calling. A ServiceID should be unique for a given service.
	/// The reference implementation combines a unique type id with a namespace so that
	/// several processes can accept the same type of service under a unique name each.
	//
	fn service ( &self ) -> Result< Self::ServiceID, Error > ;

	/// The connection id. This is used to match responses to outgoing calls.
	//
	fn conn_id ( &self ) -> Result< Self::ConnID   , Error > ;

	/// The serialization codec for the payload.
	//
	fn encoding( &self ) -> Result< Self::CodecAlg , Error > ;

	/// The serialized payload message.
	//
	fn mesg( &self ) -> Bytes;
}
