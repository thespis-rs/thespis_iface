use crate :: { * };


/// An address that allows you to send messages to an actor.
//
pub trait Address<A, M> : Clone + Recipient<M>

	where  A                     : Actor + Handler<M>,
	       M                     : Message           ,
	      <M as Message>::Return :                   ,
{
	/// Get a boxed [Recipient] to the message type of your choice, as long as the Actor
	/// implements Handler for that message type.
	//
	fn recipient( &self ) -> BoxRecipient< M, < Self as Sink<M> >::Error >;
}

