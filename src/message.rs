pub trait Message: 'static
{
	type Return: 'static;
}

