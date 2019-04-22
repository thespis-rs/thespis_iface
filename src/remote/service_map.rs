use crate::{ import::*, * } ;


/// Type responsible for knowing how call and send messages to an actor based on an Any pointer
/// ot that actors recipient, and a ServiceID.
///
/// This is the part of the code that is necessarily in the client code, usually by using a macro,
/// because the types of services are not known to the actor implementation.
///
/// TODO: generic type or associated type? Does it make sense to ever impl this twice for the same type
///       with a different MultiService type?
//
pub trait ServiceMap<MulService: MultiService>
{
	fn send_service( &self, msg: MulService, receiver: &Box<dyn Any> );

	fn call_service
	(
		&self                                            ,
		 msg        :  MulService                        ,
		 receiver   : &Box< dyn Any >                    ,
		 return_addr:  Box< dyn Recipient< MulService >> ,

	);
}
