use crate :: { Return };


/// An actor is an isolated computing unit that runs concurrently to other actors.
/// You must implement this trait as well as [`Handler`](crate::Handler) to accept messages.
///
/// The struct that implements this usually contains the (mutable) state that only this
/// actor can directly access.
//
pub trait Actor: 'static
{
	/// Gets called just before the mailbox starts listening for incoming messages.
	/// You can use this to do setup for your actor.
	//
	fn started ( &mut self ) -> Return<'_, ()> { Box::pin( async {} ) }

	/// Gets called just after the mailbox stops listening for messages.
	/// You can use this to do cleanup.
	//
	fn stopped ( &mut self ) -> Return<'_, ()> { Box::pin( async {} ) }
}

