

pub trait Message
{
	type Result;
}


pub trait ThreadSafeMessage : Message + Send  where <Self as Message>::Result: Send
{
}



impl<M> ThreadSafeMessage for M

	where M        : Send + Message,
	      M::Result: Send          ,

{
}
