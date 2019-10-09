use crate :: { * };


/// Wrapper for a message that is generic over actor instead of over message type.
///
/// It knows how to call handle on a given actor of the correct type.
//
pub trait Envelope<A> where A: Actor, Self: Send
{
	/// Have the actor handle the message
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( self: Box<Self>, actor: &mut A ) -> Return<()> where A: Send;


	/// Have the actor handle the message, for !Send Actors.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle_local( self: Box<Self>, actor: &mut A ) -> ReturnNoSend<()>;
}
