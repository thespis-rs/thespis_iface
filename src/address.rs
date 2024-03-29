use crate :: { *, import::* };


/// Behavior representing the capability of delivering a message to an actor's mailbox.
///
/// The send method comes from [Sink]:
///
/// Send a message without wanting a return from the actor. This is a one-way operation.
/// This still returns a future because the mailbox might be async, so delivering the
/// message might be async, but this will resolve as soon as the message is delivered to the mailbox.
/// You will not get notified when the message is handled by receiving actor.
///
/// This returns result because sending to the mailbox might be a fallible action.
/// If any errors happen after the message is sent to the mailbox, you shall not be notified.
/// There shall be no acknowledgement of reception.
///
/// The call method provides a request-response pattern. You can also use it when the actor
/// returns `()` to be notified that the message has been processed.
//
pub trait Address<M>: Identify

where Self: Sink<M> + fmt::Debug + Unpin + Send                           ,
      M   : Message                                                       ,
      <Self as Sink<M>>::Error: std::error::Error + Send + Sync + 'static ,

{
	/// Call an actor and receive the result of the call. This is a two-way operation. Calling with
	/// a message type that has `Return=()` will notify you that the message has been handled by the
	/// receiver.
	///
	/// Note that all types implementing `Address` also implement Sink if you want to throw a message
	/// in a bottle. See the trait documentation for more information on the `send` method.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <Self as Sink<M> >::Error >>;

	/// Get a clone of this address as a `Box<Address<M>>`.
	//
	fn clone_box( &self ) -> BoxAddress<M, <Self as Sink<M> >::Error>;

	/// Upcast `&self` to `&dyn Address`.
	//
	fn as_address( &self ) -> &dyn Address<M, Error = <Self as Sink<M> >::Error>

		where Self: Sized,
	{
		self
	}
}


impl<M, T> Address<M> for Box<T>

	where  M: Message                                                      ,
	       T: Address<M> + Identify + ?Sized                               ,
	       T: Sink<M> + fmt::Debug + Unpin + Send                          ,
	      <T as Sink<M>>::Error: std::error::Error + Send + Sync + 'static ,
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <T as Sink<M> >::Error >>
	{
		(**self).call( msg )
	}

	/// Get a clone of this address as a `Box<Address<M>>`.
	//
	fn clone_box( &self ) -> BoxAddress<M, <T as Sink<M> >::Error>
	{
		(**self).clone_box()
	}
}



impl<M, T> Address<M> for &mut T

	where  M: Message                                                      ,
	       T: Address<M> + Identify + ?Sized                               ,
	       T: Sink<M> + fmt::Debug + Unpin + Send                          ,
	      <T as Sink<M>>::Error: std::error::Error + Send + Sync + 'static ,
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <T as Sink<M> >::Error >>
	{
		(**self).call( msg )
	}

	/// Get a clone of this address as a `Box<Address<M>>`.
	//
	fn clone_box( &self ) -> BoxAddress<M, <T as Sink<M> >::Error>
	{
		(**self).clone_box()
	}
}
