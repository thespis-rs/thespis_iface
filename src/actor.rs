use crate :: { *, import::* };

pub trait Actor: Sized + 'static
{
	// Currently doesn't work because it drops the future returned from mb.start.
	//
	fn start( self, mb: impl Mailbox<Self> )
	{
		mb.start( self );
	}


	fn started (){}
	fn stopping(){}
	fn stopped (){}
}

