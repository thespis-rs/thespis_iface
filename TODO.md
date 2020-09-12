## TODO:

- !Send Messages.

- performance:
  - channel impls
  - thread sync
  - heap allocations

- benchmark the difference if Address would have poll_call instead of call and an extension trait that returns a future Call, like the futures lib does.
  inspired by https://github.com/Freax13/async-trait-ext


- go over actix features and see what would be useful to have, and at least list the things we don't have.
- polish async_chanx
- write docs and guide

- Actor should also require AssertUnwindSafe, but currently types from other libraries that use things like `UnsafeCell<Option<AtomicUsize>>`, like tokio channels will not implement that even if they probably should, which makes it to much of a hassle right now. Opened one issue at futures: https://github.com/rust-lang/futures-rs/issues/2211.



## Blocked

- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
  blocked on stabilization of defaults for associated types.

- on generic impls, tag methods as default, so that users can override them for specific types.
  blocked on stabilization of specialization.

- async in traits
