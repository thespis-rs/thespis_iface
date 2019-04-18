use crate::{ import::*, * } ;


pub trait ServiceMap<MulService>
{
	fn deserialize( &self );

	fn send_service( &self, msg: MulService, receiver: &Box<dyn Any> );

	fn call_service
	(
		&self                                            ,
		 msg        :  MulService                        ,
		 receiver   : &Box< dyn Any >                    ,
		 return_addr:  Box< dyn Recipient< MulService >> ,

	);
}
