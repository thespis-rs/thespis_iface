/// Indicates that a certain type can be sent as an actor message.
//
pub trait Message: 'static + Send
{
	/// The type of response returned when using [Address::call]. When the message is
	/// sent through [Address::send], no value will be returned.
	///
	/// If you don't need a return value but you want to guarantee that the message has
	/// been processed you can specify `()` here and use [Address::call].
	//
	type Return: 'static + Send;
}

