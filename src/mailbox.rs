use crate :: { * };

/// Interface specifying that a type can act as a mailbox for an actor.
//
pub trait Mailbox< A: Actor + Send > : Identify + Sized
{
	/// Return a future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled." ]
	//
	fn start_fut( self, actor: A ) -> Return<'static, Option<Self>>;
}


/// A mailbox that doesn't require the actor to be `Send`.
//
pub trait MailboxLocal< A: Actor > : Identify + Sized
{
	/// Return a non-Send future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled." ]
	//
	fn start_fut_local( self, actor: A ) -> ReturnNoSend<'static, Option<Self>>;
}
