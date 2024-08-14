# Argon2 Plugin

This plugin provides a simple interface to the Argon2 password hashing algorithm.

## Installation

To use this plugin, use this command in your project directory:

```sh
query plugin install https://github.com/gc-victor/query-plugin-argon2
```

It will download the plugin to the `./plugins` folder and add it to your `.query/plugins.toml` file.

## API

- hash: It takes a password and a salt and returns the hashed password.
- verify: It takes a password, a salt, and a hashed password and returns a boolean indicating if the password matches the hashed password.

## Usage

First, we recommend creating a wrapper around the plugin to make it easier to use in multiple places.

```js
// src/plugins/argon2.js
import { plugin } from 'query:plugin';

export const argon2 = {
  hash: (password) => plugin("plugin_argon2.wasm", "hash", "password", null),
  verify: (password, hash) => plugin("plugin_argon2.wasm", "verify", JSON.stringify({password, hash}), null) == "true"
};
```

Then we can use the wrapper to hash and verify passwords.

```js
// src/functions/get.index.js
import { argon2 } from '../plugins/argon2';

export async function handleRequest(req) {
    const password = "password";
    const hash = await argon2.hash(password);
    const isValid = argon2.verify(password, hash);

    return new Response(isValid ? `✔️` : `❌`, {
      status: 200,
      headers: {
          "Content-Type": "application/json",
      },
  });
}
```

References:

- [Query Plugin System](https://github.com/gc-victor/query?tab=readme-ov-file#plugins-system)
- [Query CLI Plugin](https://github.com/gc-victor/query?tab=readme-ov-file#plugin)
- [Extism - Rust PDK](https://github.com/extism/rust-pdk)
