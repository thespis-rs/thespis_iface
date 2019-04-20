# thespis_iface
The interface of thespis (contains only traits)


## TODO:

- Get rid of the threadsafe variants of address
- oblige all implementors of all traits to implement Debug?
- go over actix features and see what would be useful to have (backpressure?)
- remote Addr? if the actor is known?
- compile time service uids and if not, at least don't re-hash them on every access
- ergonomics. Get an address to a started actor without having to manually make and start a mailbox (implementation?)
- defaults for associated types, like () for Message::Result, and possibility to derive with defaults
- how can an actor stop itself, should mb be fed to handle, should there be a method self.mb, should there be a stop method on mailbox
- do some proper research on tokio reactor. Just figured out we don't need a tokio runtime to use stuff that uses epoll. A futures 0.3 executor will do just fine, just using compat on the futures and streams from tokio.


