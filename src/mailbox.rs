use crate :: { * };

/// TODO: Right now this exposes the existance of a default runtime, which is not defined by iface... So
/// this leaks implementation details.
//
pub trait Mailbox< A: Actor + Send >
{
	/// Start the mailbox. This consumes the mailbox, so get your addresses before running this.
	//
	fn start( self, actor: A ) -> ThesRes<()>;

	/// Return a future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn start_fut( self, actor: A ) -> Return<'static, ()>;
}


/// A mailbox that doesn't require the actor to be `Send`.
//
pub trait MailboxLocal< A: Actor >
{
	/// Start the mailbox. This consumes the mailbox, so get your addresses before running this.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn start_local( self, actor: A ) -> ThesRes<()>;

	/// Return a non-Send future that allows starting the mailbox by spawning it on the executor of your choice.
	///
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn start_fut_local( self, actor: A ) -> ReturnNoSend<'static, ()>;
}
