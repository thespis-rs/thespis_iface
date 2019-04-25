use crate::{ import::* } ;

/// Represents a serialization algorithm used for deserializing self describing messages.
/// Over the wire a protobuf varint shall be used, so it should not be bigger than a u64.
///
/// It's up to the implementer of routers and peers to provide a table with supported codecs.
///
/// This is inspired from multiformat's multicodec, however their table only has protobuf and
/// cbor as serialization codecs, so that's not very multi. However if you use a codec included
/// in their table, you might want to choose the same binary representation, which might help
/// interop in the future.
///
/// This can generally be implemented with an enum. The crate num-traits provides IntoPrimitive
/// for enums which helps conversion to u64.
///
pub trait CodecAlg :

	Into< Bytes >    +
	TryFrom< Bytes > +
	Debug            +
	Display          +
	Clone            +
	PartialEq        +
	Eq               +
	Hash             +
	Send             +
	Sync             +

{}
