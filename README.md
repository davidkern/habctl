# HABctl

HAB truck camper backend services.

# Building

This binary is built for a Linux SBC (currently running Debian).  The following dependencies should be installed:

* libudev-dev - for serialport crate

# Development

Mount the Hab server's devices locally by running `sudo scripts/share-devices` on the server,
and `scripts/mount-devices` on the development machine.  Devices will be mounted in `/tmp` and
can be used for local development.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
