use crate :: { * };

/// TODO: Right now this exposes the existance of a default runtime, which is not defined by iface... So
/// this leaks implementation details.
//
pub trait Mailbox< A: Actor >
{
	/// Start the mailbox. This consumes the mailbox for now, so get your addresses before running this.
	/// You should spawn the future on a default executor.
	///
	//
	fn start( self, actor: A ) -> ThesRes<()>;

	/// Return a future that allows starting the mailbox.
	///
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn start_fut( self, actor: A ) -> Return<'static, ()>;
}
