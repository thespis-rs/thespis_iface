# thespis

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/thespis.svg?branch=master)](https://travis-ci.org/najamelan/thespis)
[![Docs](https://docs.rs/thespis/badge.svg)](https://docs.rs/thespis)
[![crates.io](https://img.shields.io/crates/v/thespis.svg)](https://crates.io/crates/thespis)


> Interface of the thespis actor model.

The interface of the thespis actor model (contains only traits). This defines the expected behavior for Addresses that can send to Actors, as well as the `Handler` trait and the `Message` trait.

There used to be a `Mailbox` trait, but it turns out that the mailbox is not depended on by any of the other components, so it's iplementation can be freely changed without requiring an interface.

The purpose for the split between interface and implementation is 2-fold:
1. Libraries can expose an actor based interface without having to depend on an implementation. Consumers can then choose any implementation they want and everything will remain inter-operable.
2. Each component can be individually replaced and composed if you need a different behavior then the reference implementation.

The reference implementation can be found in the `thespis_impl` crate.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add thespis`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   thespis: ^0.1
```

In Cargo.toml:
```toml
[dependencies]

   thespis = "0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/thespis/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security




## Usage



### Basic example

```rust

```

## API

API documentation can be found on [docs.rs](https://docs.rs/thespis).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/thespis/blob/master/CONTRIBUTING.md).


### Testing


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)




## TODO:

- !Send Messages.

- performance:
  - channel impls
  - thread sync
  - heap allocations

- benchmark the difference if Address would have poll_call instead of call and an extension trait that returns a future Call, like the futures lib does.
  inspired by https://github.com/Freax13/async-trait-ext



- check mut requirements. we require mut in alot of places, like when sending on an address the address has to be mut. Should we relieve certain of those. It means for example that a struct which holds an addr must also be mut or put it in Refcell just to send messages.
- impl traits on Box, Rc, Arc, &/&mut, etc

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




## Blocked

- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
  blocked on stabilization of defaults for associated types.

- on generic impls, tag methods as default, so that users can override them for specific types.
  blocked on stabilization of specialization.


## Design issues:

- Address is capability. Currently we work on "in process" stuff (the programmer compiling is deemed responsible for the outcome), so not much security considerations are in place. And we work on cross process communication where we consider an all public interface. A process provides services, and if it accepts a connection, it specifies which services will respond on that connection, but it's not more fine grained than that ATM. Eg. We have not implemented any authentication/authorization/encryption/signing.
Consider how you make something available to some entity that is authorized only.
