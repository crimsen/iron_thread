# Iron Thread (iron-thread)

An App written in Rust and TypeScript based on Tauri.app and Quasar.dev to manage your fabrics.

## Install the dependencies

| dependencies | version |
| ------------ | ------- |
| rustc        | 1.93.1  |
| node         | 22.15.1 |
| yarn         | 1.22.19 |
| cargo        | 1.93.1  |

```bash
cargo install tauri-cli --version "^2.1.0" --locked
```

Optional you install cli's to create migration files with sea-orm-cli

```bash
cargo install sea-orm-cli --version "^1.1.19" --locked
```

### Start the app in development mode (hot-code reloading, error reporting, etc.)

```bash
cargo tauri android dev
# or for iOS (but not yet included)
cargo tauri ios dev
```

### Lint the files

To lint and check the frontend:

```bash
yarn lint
```

For backend:

```bash
cd src-tauri && cargo check
```

### Format the files

To format file in frontend:

```bash
yarn format
```

To format files backend use rustfmt:

### Build the app for production

```bash
cargo tauri android build
# or for iOS (but not yet included)
cargo tauri ios build
```
