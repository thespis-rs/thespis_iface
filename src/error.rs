use crate::{ import::* };


/// The main error type for thespis_impl. Use [`ThesErr::kind()`] to know which kind of
/// error happened. ThesErrKind implements Eq, so you can the following if all you want to
/// know is the kind of error. You can obviously also match the data contained in the ThesErrKind
/// if you want, but you don't have to:
///
/// ```ignore
/// match return_a_result()
/// {
///    Err(e) =>
///    {
///       match e.kind()
///       {
///          ThesErrKind::MailboxFull{..} => println!( "{}", e ),
///          _ => {},
///       }
///
///       if let ThesErrKind::MailboxFull{..} = e.kind()
///       {
///          println!( "{}", e );
///       }
///    },
///
///    Ok(_) => {}
/// }
/// ```
//
#[ derive( Debug ) ]
//
pub struct ThesErr
{
	inner: FailContext<ThesErrKind>,
}



/// The different kind of errors that can happen when you use the thespis_impl API.
//
#[ derive( Clone, PartialEq, Eq, Debug, Fail ) ]
//
pub enum ThesErrKind
{
	/// A connection error happened
	//
	#[ fail( display = "A connection error happened: {}", what ) ]
	//
	Connection
	{
		/// Error string from the underlying error.
		//
		what: String
	},

	/// Failed to deserialize
	//
	#[ fail( display = "Deserialize: Failed to deserialize: {}", what ) ]
	//
	Deserialize
	{
		/// Error string from the underlying error.
		//
		what: String
	},

	/// Cannot initialize global executor twice.
	//
	#[ fail( display = "DoubleExecutorInit: Cannot initialize global executor twice" ) ]
	//
	DoubleExecutorInit,

	/// MailboxClosed: Mailbox panicked before we could send the message.
	//
	#[ fail( display = "MailboxClosed: Mailbox panicked before we could send the message, actor: {}", actor ) ]
	//
	MailboxClosed
	{
		/// Id of the actor that panicked.
		//
		actor: String
	},

	/// MailboxClosedBeforeResponse: Mailbox crashed after the message was sent but before we got a response.
	//
	#[ fail( display = "MailboxClosedBeforeResponse: Mailbox crashed after the message was sent but before we got a response, actor: {}", actor ) ]
	//
	MailboxClosedBeforeResponse
	{
		/// Id of the actor that panicked.
		//
		actor: String
	},

	/// MailboxFull, cannot queue any more messages.
	//
	#[ fail( display = "MailboxFull: Mailbox Full for: {}", actor ) ]
	//
	MailboxFull
	{
		/// Id of the concerned actor.
		//
		actor: String
	},

	/// Failed to serialize.
	//
	#[ fail( display = "Serialize: Failed to serialize: {}", what ) ]
	//
	Serialize
	{
		/// Error string from the underlying error.
		//
		what: String
	},

	/// Failed to spawn a future.
	//
	#[ fail( display = "Spawn: Failed to spawn a future in: {}", context ) ]
	//
	Spawn
	{
		/// Context of error.
		//
		context: String
	},

	/// Some operation timed out
	//
	#[ fail( display = "Timeout: {}", context ) ]
	//
	Timeout
	{
		/// Context of error.
		//
		context: String
	},
}



impl fmt::Display for ThesErr
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		fmt::Display::fmt( &self.inner, f )
	}
}


impl ThesErr
{
	/// Allows matching on the error kind
	//
	pub fn kind( &self ) -> &ThesErrKind
	{
		self.inner.get_context()
	}
}

impl From<ThesErrKind> for ThesErr
{
	fn from( kind: ThesErrKind ) -> ThesErr
	{
		ThesErr { inner: FailContext::new( kind ) }
	}
}

impl From< FailContext<ThesErrKind> > for ThesErr
{
	fn from( inner: FailContext<ThesErrKind> ) -> ThesErr
	{
		ThesErr { inner: inner }
	}
}


impl std::error::Error for ThesErr {}


