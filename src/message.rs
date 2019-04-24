pub trait Message: 'static + Send
{
	type Return: 'static + Send;
}

