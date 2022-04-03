# sl-rs

Rust implementation of the `sl` command, for practice.

## Bugs

Currently, the train's beginning position is wrong.

## TODO

the origin `sl`

```bash
❯ lsd -l /usr/bin/sl
.rwxrwxr-x root root 34 KB Thu Feb  3 14:14:07 2022  /usr/bin/sl
```

but `sl-rs`

```bash
❯ lsd -l ./target/release/sl-rs
.rwxr-xr-x w users 258 KB Sun Apr  3 17:57:17 2022  ./target/release/sl-rs
```

Maybe need more effort to reduce the size of the binary.
