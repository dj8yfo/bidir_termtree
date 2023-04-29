# bidirectional termtree 

> Visualize tree-like data on the command-line

## Example

An example program is provided under the "examples" directory to mimic the `tree(1)`
linux program

```bash
$ cargo run --example tree target
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/tree target`
target
└── debug
    ├── .cargo-lock
    ├── .fingerprint
    |   └── termtree-21a5bdbd42e0b6da
    |       ├── dep-example-tree
    |       ├── dep-lib-termtree
    |       ├── example-tree
    |       ├── example-tree.json
    |       ├── lib-termtree
    |       └── lib-termtree.json
    ├── build
    ├── deps
    |   └── libtermtree.rlib
    ├── examples
    |   ├── tree
    |   └── tree.dSYM
    |       └── Contents
    |           ├── Info.plist
    |           └── Resources
    |               └── DWARF
    |                   └── tree
    ├── libtermtree.rlib
    └── native
```

```bash
❯ cargo run --example tree_up target/debug/.fingerprint
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/tree_up target/debug/.fingerprint`
    ┌── example-tree
    ├── invoked.timestamp
    ├── example-tree.json
    ├── dep-example-tree
┌── bidir_termtree-2668ad9a2a26e5f3
│   ┌── lib-bidir_termtree.json
│   ├── invoked.timestamp
│   ├── lib-bidir_termtree
│   ├── dep-lib-bidir_termtree
├── bidir_termtree-ef09f09b8a0fb98c
│   ┌── example-tree
│   ├── invoked.timestamp
│   ├── example-tree.json
│   ├── dep-example-tree
├── bidir_termtree-57357bb7081cd257
│   ┌── dep-example-tree_up
│   ├── invoked.timestamp
│   ├── example-tree_up.json
│   ├── example-tree_up
├── bidir_termtree-3320445c1593b068
│   ┌── test-lib-bidir_termtree.json
│   ├── dep-test-lib-bidir_termtree
│   ├── test-lib-bidir_termtree
│   ├── invoked.timestamp
├── bidir_termtree-99ab0a9de60ddeac
│   ┌── lib-bidir_termtree.json
│   ├── invoked.timestamp
│   ├── lib-bidir_termtree
│   ├── dep-lib-bidir_termtree
├── bidir_termtree-1c8302dada71b921
│   ┌── dep-example-tree_up
│   ├── invoked.timestamp
│   ├── example-tree_up.json
│   ├── example-tree_up
├── bidir_termtree-d3fe7015bb6fbf1a
.fingerprint
```

## Related Crates

- [`termtree`](https://crates.io/crates/termtree): bidir_termtree was forked from this.
- [`treeline`](https://crates.io/crates/treeline): termtree was forked from this.
- [`tree_decorator`](https://crates.io/crates/tree_decorator)
- [`xtree`](https://crates.io/crates/xtree)
- [`ptree`](https://crates.io/crates/ptree)

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
