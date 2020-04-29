# thespis_iface
The interface of the thespis actor model (contains only traits).


## TODO:

- benchmark the difference if Address would have poll_call instead of call and an extension trait that returns a future Call, like the futures lib does.
  inspired by https://github.com/Freax13/async-trait-ext

- derive
  - Message
  - #[ thespis::Handler( MyActor, Show ) ] async fn ...

- check mut requirements. we require mut in alot of places, like when sending on an address the address has to be mut. Should we relieve certain of those. It means for example that a struct which holds an addr must also be mut or put it in Refcell just to send messages.
- impl traits on Box, Rc, Arc, &, &mut, etc

- go over actix features and see what would be useful to have, and at least list the things we don't have.
- impl Sink for references? &'a Addr<A>
- polish async_chanx
- make pharos generic over channels, checkout broadcast channels.
- enable CI testing
- enable dependabot
- write docs and guide
- try to relax pinning requirements where we can and impl unpin for things like addr to avoid
  forwarding pinning requirements to Actor.
- check TODO's and FIXME's in code

## Guide level docs

- think about and write up the difference in Send + reply_to and Call. Performance, possiblity to pass on reply_to, link the outgoing send to the incoming response? Possibility to send a reply_to address that is not your own?


### Design patterns
- how can you stop a mailbox before all adresses are dropped?
- how can an actor stop it's own mailbox
- how can an actor send messages to itself
- concurrent message processing
- concurrent message processing with update self (without actorfuture)
- interfaces: take an address to an actor that implements Handler<X> + Handler<Y> + ...


## Blocked

- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
  blocked on stabilization of defaults for associated types.

- on generic impls, tag methods as default, so that users can override them for specific types.
  blocked on stabilization of specialization.


## Design issues:

- The wire format is a hand baked solution just to get it working. Now we should find out what the final formats might look like. Cap'n proto? or SBE? : https://polysync.io/blog/session-types-for-hearty-codecs

- notion of "one message at a time". This is only necessary if the actor has mutable state. If the actor works without mutable state, it can be run in parallel.
  Should be possible by just spawning from within the handler and returning immediately. We should create an example that demonstrates this.

- Address is capability. Currently we work on "in process" stuff (the programmer compiling is deemed responsible for the outcome), so not much security considerations are in place. And we work on cross process communication where we consider an all public interface. A process provides services, and if it accepts a connection, it specifies which services will respond on that connection, but it's not more fine grained than that ATM. Eg. We have not implemented any authentication/authorization/encryption/signing.
Consider how you make something available to some entity that is authorized only.

- Send vs !Send messages, see: https://users.rust-lang.org/t/how-do-you-all-make-your-dynamic-code-send-agnostic/27567/5
  Currently !Send actors are supported, but not !Send messages.
