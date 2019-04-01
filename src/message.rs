pub trait Message: 'static
{
	type Result: 'static;
}

