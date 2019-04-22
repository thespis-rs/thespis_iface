use crate :: { *, import::* };

pub trait Actor: Sized + 'static
{
	/// Gets called just before the mailbox starts listening for incoming messages.
	/// You can do this to do setup for your actor.
	//
	fn started ( &mut self ) -> Response<()> { async {}.boxed() }

	/// Gets called just after the mailbox stops listening for messages.
	/// You can use this to do cleanup.
	//
	fn stopped ( &mut self ) -> Response<()> { async {}.boxed() }
}

