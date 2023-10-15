# Reo: Distributed Key Value Store

Reo is a fast distributed key-value store written in Rust.

- Invalidate records based on TTL.
- Supports both persistent storage and in-memory
- Supports sharding

For node configuration refer to NodeConfig.json.

`dev.sh` can be used to run Reo in development mode.

## TODOs

- [ ] Replication support
- [ ] Faster hashmap for in-memory use-cases
- [ ] Authentication mechanism(s)
- [ ] Faster server to server communication
- [ ] Centralized configuration
- [ ] Better test environment

## CONTRIBUTING

Create an issue on Github or email me at [emincan@emincanozcan.com](mailto:emincan@emincanozcan.com) before working on Reo to ensure your idea will be accepted.

- [ ] Fork
- [ ] Contribute
- [ ] Pull request

## PRODUCTION USAGE

Not ready for production yet. The project is still in development.

## LICENSE

MIT
