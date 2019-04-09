# thespis_iface
The interface of thespis (contains only traits)


## Remote Actors Design

The user has to define their services as such:

```rust
use thespis::ServiceMap;

#[ derive( ServiceMap, FromPrimitive, ToPrimitive ) ]
//
enum MyServices
{
	RegisterPeer   ,
	WeatherForecast,
	...,
}
```

The FromPrimitive, ToPrimitive derives allow serializing the service to an unsigned varint (from multiformats). Actually these traits just do `Enum::Variant as i64`, so maybe it's not worth having a dependency for this.

The derive ServiceMap is provided by thespis. It will add deserialization like:

```rust
deserialize<T>( service: MyServices, msg: BytesMut ) -> Result<T, failure::Error>
{
	match service
	{
		MyServices::RegisterPeer => serde::deserialize::<RegisterPeer>( msg )?,
		...
	}

};
```

This means that the type RegisterPeer must be in scope. And it must correctly deserialize, otherwise an error is returned.

We need this minimal boilerplate because we need to register the deserialization types at compile time. Otherwise we can't provide deserialization into the concrete message types that actors take.

The alternative I see is to have every actor accept messgages of the type MultiService and do the deserialization themselves. It would seem to me that this is a far inferior solution, since an actor is no longer handler for a specific type, but for MultiService and deserialization code needs to be added to each handler.

An extension to actor will be made that adds a method to self. This allows to register this actor for handling service x by  calling `self.register_service( MyServices::RegisterPeer );` in the `started()` method of the actor.

A router will take an AsyncRead + AsyncWrite and a `Box< dyn ServiceMap >`. Like this it has all information to handle the connection and dispatch messages to the right actors.

```rust

struct RegisterService
{

}

impl Handler<RegisterService> for Router
{
	fn handle( &mut self, msg: RegisterService ) -> TupleResponse
	{

	}
}

```
