/// Indicates that a certain type can be sent as an actor message.
///
/// - Must be UnwindSafe because messages should not hold shared mutable data or references.
///   If you cheat on this your warranty is void. You shouldn't include Atomics or
///   Mutexes/RwLocks either, even though these implement UnwindSafe.
///   Mailboxes must be able to recover from panics inside the handler method.
///
/// - Must be 'static because Messages should not hold references.
/// - Must be Send because limitations in the Rust type system mean we would have to double
///   out the entire interface to support `Send` vs not `Send` messages.
//
pub trait Message: 'static + Send + std::panic::UnwindSafe
{
	/// The type of response returned when using [Address::call]. When the message is
	/// sent through [Address::send], no value will be returned.
	///
	/// If you don't need a return value but you want to guarantee that the message has
	/// been processed you can specify `()` here and use [Address::call].
	//
	type Return: 'static + Send;
}

