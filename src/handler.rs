use crate :: { * };

/// Trait indicating that the implementor can receive messages of a certain type.
/// An actor can implement Handler for as many types as desired.
//
pub trait Handler< M: Message > where Self: Actor
{
	/// Define how your actor handles this message type. If your actor is !Send,
	/// you should implement `handle_local` and give `handle` an "unreachable!()" implementation.
	/// It shall never be called.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle( &mut self, msg: M ) -> Return< '_, <M as Message>::Return >;



	/// Define how your actor handles this message type.
	///
	/// You still have to implement `handle` because that is a required method, but if your actor is Send
	/// and you implement `handle`, you get `handle_local` automatically.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn handle_local( &mut self, msg: M ) -> ReturnNoSend< '_, <M as Message>::Return >
	{
		self.handle( msg )
	}
}
