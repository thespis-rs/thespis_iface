use crate :: { *, import::* };

pub trait Actor: Sized + 'static
{
	fn started (){}
	fn stopping(){}
	fn stopped (){}
}

