

pub trait Message
{
	type Result;
}



pub trait ThreadSafeMessage : Message + Send
{
	type Result: Send;
}



impl<M> ThreadSafeMessage for M

	where M        : Send + Message,
	      M::Result: Send          ,

{
	type Result = M::Result;
}
