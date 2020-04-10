use crate :: { import::*, Return };


/// An actor is an isolated computing unit. For an introduction to the actor model, see:
/// - https://youtu.be/7erJ1DV_Tlo
//
pub trait Actor: 'static
{
	/// Gets called just before the mailbox starts listening for incoming messages.
	/// You can do this to do setup for your actor.
	//
	fn started<'a, 'b>( &'a mut self ) -> Return<'b, ()> where 'a: 'b, Self: 'b

	{ async {}.boxed() }

	/// Gets called just after the mailbox stops listening for messages.
	/// You can use this to do cleanup.
	//
	fn stopped<'a, 'b>( &mut self ) -> Return<'_, ()> where 'a: 'b, Self: 'b

	{ async {}.boxed() }
}

