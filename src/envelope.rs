use crate :: { * };


/// Wrapper for a message that is generic over actor instead of over message type.
///
/// It knows how to call handle on a given actor of the correct type.
//
pub trait Envelope<A> where A: Actor
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( self: Box<Self>, actor: &mut A ) -> ReturnNoSend<()>;
}
