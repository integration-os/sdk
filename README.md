# Rust SDK NAPI Building

This document describes the process for building and publishing the [`@integrationos/node`](https://www.npmjs.com/package/@integrationos/node) and related npm packages.

## Dependencies

You will need write access to the `@integrationos` npm organization.

You will need to install the `zig` language

```bash
brew install zig
```

You will need to install all 3 target architectures we support

```bash
rustup target add x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin
```

You will need to install [`@napi-rs/cli`](https://napi.rs/docs/introduction/getting-started#install-cli)

```bash
npm install -g @napi-rs/cli
```

## Updating Version

In the [`package.json`](./package.json) file, update the version.

## Building

Build the three architectures

```bash
npm run build-all
```

## Publishing

Do a dry-run to make sure everything is going to be published correctly. You must also go into each architecture module in `npm` and run the following command there as well.

```bash
npm publish --otp=<2FA_ONE_TIME_PASSWORD> --dry-run
```

Remove `--dry-run` to publish.

## Tests

Tests are in the [`__test__`](./__test__/) folder

```bash
npm install
npm run test
```
