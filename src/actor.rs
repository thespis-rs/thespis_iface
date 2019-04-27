use crate :: { *, import::* };


/// An actor is an isolated computing unit. For an introduction to the actor model, see:
/// - https://youtu.be/7erJ1DV_Tlo
//
pub trait Actor: 'static
{
	/// Gets called just before the mailbox starts listening for incoming messages.
	/// You can do this to do setup for your actor.
	//
	fn started ( &mut self ) -> Return<()> { async {}.boxed() }

	/// Gets called just after the mailbox stops listening for messages.
	/// You can use this to do cleanup.
	//
	fn stopped ( &mut self ) -> Return<()> { async {}.boxed() }
}

