use crate :: { Return };


/// An actor is an isolated computing unit that runs concurrently to other actors.
/// You must implement this trait as well as [`Handler`](crate::Handler) to accept messages.
///
/// The type that implements this usually contains the (mutable) state that only this
/// actor can directly access.
///
/// Mailbox implementations can decide to `catch_unwind` when your actor handles a message
/// to keep the mailbox alive if the actor panics and make it possible to supervise actors.
/// For this reason your actor should be [`std::panic::UnwindSafe`]. This might become a
/// required trait bound in the future.
//
pub trait Actor: 'static
{
	/// Gets called just before the mailbox starts processing incoming messages.
	/// You can use this to do setup for your actor.
	//
	fn started ( &mut self ) -> Return<'_, ()> { Box::pin( async {} ) }

	/// Gets called just after the mailbox stops listening for messages.
	/// You can use this to do cleanup, however in the reference implementation,
	/// your actor will be returned when awaiting the `JoinHandle` of the mailbox
	/// and it could be spawned again later.
	///
	/// The real EOL of your actor in indicated by [`Drop`].
	//
	fn stopped ( &mut self ) -> Return<'_, ()> { Box::pin( async {} ) }
}

