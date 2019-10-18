use crate :: { *, import::* };


/// Behavior representing the capability of delivering a message to an actor's mailbox.
///
/// The send method comes from Sink:
///
/// Send a message without wanting a return from the actor. This is a one-way operation.
/// This still returns a future because the mailbox might be async, so delivering the
/// message might be async, but this will resolve as soon as the message is sent to the mailbox.
/// You will not get notified when the message is handled by receiver.
///
/// This returns result because sending to the mailbox might be a fallible action.
/// If any errors happen after the message is sent to the mailbox, you shall not be notified.
/// There shall be no acknowledgement of reception.
//
pub trait Recipient<M>

	where  Self: Sink<M> + Any + fmt::Debug + Unpin + Send,
	       M   : Message,

{
	/// Call an actor and receive the result of the call. This is a two-way operation. Calling with
	/// a message type that has `Return=()` will notify you that the message has been handled by the
	/// receiver.
	///
	/// This is the equivalent of a method call in the actor model. The overhead in a single thread
	/// is about x40 compared to a method call. It is x3 compared to a [send].
	///
	/// TODO: put some actual benchmark results.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <Self as Sink<M> >::Error >>;

	/// Get a clone of this recipient as a `Box<Recipient<M>>`.
	//
	fn clone_box( &self ) -> BoxRecipient<M, <Self as Sink<M> >::Error>;

	/// Get a unique identifier for the actor this will send to, so you can verify
	/// if two recipients deliver to the same actor.
	///
	/// If you get an error: `cannot infer type for `M`, try:
	///
	/// ```ignore
	/// // Replace XXX with a message type this actor supports.
	/// //
	/// <Addr<Actor> as Recipient<XXX>>::actor_id( &addr );
	/// ```
	/// I have only come across this problem in a macro, but felt it better to give hints
	/// here for others. Normally this is not needed.
	//
	fn actor_id( &self ) -> usize;
}

