pub trait Message: 'static + Send + Sync
{
	type Return: 'static + Send + Sync;
}

