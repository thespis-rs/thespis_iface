/// Indicates that a certain type can be sent as an actor message.
///
/// - Must be UnwindSafe because messages should not hold shared mutable data or references.
///   If you cheat on this your warranty is void. You shouldn't include Atomics or
///   Mutexes/RwLocks either, even though these implement UnwindSafe.
///   Mailboxes must be able to recover from panics inside the handler method.
///   The trait bound has been removed because it is very inconsistently applied troughout
///   the ecosystem which means one would have to wrap every message in AssertUnwindSafe.
///   This fails the goal of actually detecting where a problem might be present, making
///   the trait more or less useless.
///   Also it is more a requirement of the implementation than of the interface, as
///   some implementations might chose not to recover from panics. In that case there is no
///   risk. `thespis_impl` in any case wraps your handlers in AssertUnwindSafe in order to
///   recover and allow actor supervision. So if you use that, be sure to respect the
///   directions given above.
///
/// - Must be 'static because Messages should not hold references.
/// - Must be Send because limitations in the Rust type system mean we would have to double
///   out the entire interface to support `Send` vs not `Send` messages.
//
pub trait Message: 'static + Send
{
	/// The type of response returned when using [Address::call](crate::Address::call). When the message is
	/// sent through [Address::send](futures-sink::SinkExt::send), no value will be returned.
	///
	/// If you don't need a return value but you want to guarantee that the message has
	/// been processed you can specify `()` here and use [Address::call](crate::Address::call).
	//
	type Return: 'static + Send;
}

