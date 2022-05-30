# thespis

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/thespis.svg?branch=release)](https://travis-ci.org/najamelan/thespis)
[![Docs](https://docs.rs/thespis/badge.svg)](https://docs.rs/thespis)
[![crates.io](https://img.shields.io/crates/v/thespis.svg)](https://crates.io/crates/thespis)


> Interface of the thespis actor model.

The interface of the thespis actor model (contains only traits). This defines the expected behavior for Addresses that can send to Actors, as well as the [`Handler`] trait and the [`Message`] trait.

There used to be a `Mailbox` trait, but it turns out that the mailbox is not depended on by any of the other components, so it's iplementation can be freely changed without requiring an interface.

The purpose for the split between interface and implementation is 2-fold:
1. Libraries can expose an actor based interface without having to depend on an implementation. Consumers can then choose any implementation they want and everything will remain inter-operable.
2. Each component can be individually replaced and composed if you need a different behavior then the reference implementation.

The reference implementation can be found in the [_thespis_impl_](https://docs.rs/thespis_impl) crate.

To get started with _thespis_, please check out the [guide level documentation](https://thespis-rs.github.io/thespis_guide/).

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

   thespis: ^0.2
```

In Cargo.toml:
```toml
[dependencies]

   thespis = "0.2"
```

### Upgrade

Please check out the [changelog](https://github.com/thespis-rs/thespis_iface/blob/release/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you. Check [`Cargo.yml`](https://github.com/thespis-rs/thespis_iface/blob/release/Cargo.yml) for the list of dependencies.

There is one optional feature, `derive`, enabled by default which adds proc macros for deriving the `Message` trait as well as removing some boilerplate when implementing `Handler`.


### Security

This crate does not use unsafe, but it's dependencies do.


## Usage

Please refer to the [_thespis_impl_](https://github.com/thespis-rs/thespis_impl/blob/release/examples) crate to see examples of usage.

## API

API documentation can be found on [docs.rs](https://docs.rs/thespis).


## Contributing

Please check out the [contribution guidelines](https://github.com/thespis-rs/thespis/blob/release/CONTRIBUTING.md).


### Testing

As this crate only provides traits, there aren't any tests. You can check the [_thespis_impl_](https://github.com/thespis-rs/thespis_impl/blob/release/tests) crate for the tests.

### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)
