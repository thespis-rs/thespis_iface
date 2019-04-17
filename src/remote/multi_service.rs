use crate::{ import::*, * } ;




/// The serialization codec. The numbers come from multiformats multicodec.
///
/*#[ derive( Debug, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive ) ]
//
pub enum CodecAlg
{
	Cbor = 0x51,
}
*/



/// Represents a message as it is over the wire, with some metadata attached to the serialized `Message`.
///
/// The actual wire format is not defined here, but is implementation dependent. This seems bad for intorp,
/// but it is actually not conceivalbe right now to define a single format and force that on all users.
///
/// Different tradeoffs might require different layout. CPU vs Memory consumption for example.
/// Maybe a few default formats could be suggested later. Like a "compact" format and a "fast" format.
///
/// The trait bound `for<'a> TryFrom< &'a mut BytesMut >` is for codecs like Tokio Codec. You should
/// implement this as you would implement Decoder::decode. Advancing the pointer and taking possesion
/// of the data that successfully decoded once an entire frame is present.
///
/// In contrast, the into and try_from Bytes are not mutable, and implementations are supposed to just
/// wrap them without copying anything. Providing a view into the memory through the trait methods.
//
pub trait MultiService : Into< Bytes > + From< Bytes >
{
	type ServiceID: UniqueID;
	type ConnID   : UniqueID;
	type CodecAlg : CodecAlg;

	fn len     ( &self ) -> usize                            ;
	fn service ( &self ) -> Result< Self::ServiceID, Error > ;
	fn conn_id ( &self ) -> Result< Self::ConnID   , Error > ;
	fn encoding( &self ) -> Result< Self::CodecAlg , Error > ;
	fn mesg    ( &self ) -> Bytes                            ;
}
