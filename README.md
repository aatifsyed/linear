## Cargo commands
```sh
cargo rustc -- \
    -Zunpretty=expanded \
    > expanded.rs

cargo rustc -- \
    -Z dump-mir=all \
    -Z dump-mir-graphviz \
    -Z dump-mir-dataflow \
    -Z dump-mir-spanview

cargo rustc -- \
    -Z unpretty=mir-cfg \
    > mir-cfg.dot

cargo rustc -- \
    -Z unpretty=hir-tree \
    > hir-tree.rs

cargo rustc -- \
    -Z unpretty=thir-tree \
    > thir-tree.rs
```