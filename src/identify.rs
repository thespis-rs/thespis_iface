use crate :: { import::* };


/// Interface for uniquely identifying actors. Mainly useful for logging and debugging.
///
/// This is separate from [`Address`] because an [`Address`] implementer might want
/// to show id/name in a Debug impl, but Address<M> will only be available for specific
/// `M` for which the actor implements Handler. `M` is the generic message type. That causes issues.
/// This trait allows for identity information to available in places where no M is available.
///
/// [`Address`]: crate::Address
//
pub trait Identify
{
	/// Get a unique identifier for this actor. You can use this to verify
	/// whether two addresss deliver to the same actor.
	//
	fn id( &self ) -> usize;

	/// A human readable name of the actor.
	//
	fn name( &self ) -> Arc<str>;
}



impl<T: ?Sized> Identify for Box<T> where T: Identify
{
	fn id( &self ) -> usize { (**self).id() }

	fn name( &self ) -> Arc<str> { (**self).name() }
}



impl<T: ?Sized> Identify for Arc<T> where T: Identify
{
	fn id( &self ) -> usize { (**self).id() }

	fn name( &self ) -> Arc<str> { (**self).name() }
}



impl<T: ?Sized> Identify for Rc<T> where T: Identify
{
	fn id( &self ) -> usize { (**self).id() }

	fn name( &self ) -> Arc<str> { (**self).name() }
}



impl<T: ?Sized> Identify for &T where T: Identify
{
	fn id( &self ) -> usize { (**self).id() }

	fn name( &self ) -> Arc<str> { (**self).name() }
}



impl<T: ?Sized> Identify for &mut T where T: Identify
{
	fn id( &self ) -> usize { (**self).id() }

	fn name( &self ) -> Arc<str> { (**self).name() }
}
