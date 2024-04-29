## Overview

A OCI runtime thats used to execute ops nanovm unikerenels in a Kuberenetes/OpenShift cluster

### Unikernels 

So why unikernels

- Unikernels are specialized, single-address-space machine images constructed by using library operating systems. 
- They are lightweight, secure, and fast to boot.
- They offer improved isolation

### OCI Runtime

This project is an OCI runtime that is used together with crun (used as OCI runtime in CRI-O and Podman).
Calls are made to this runtime by virtue of a RuntimeClass in Kubernetes/OpenShift.

A recommended approach would be to use dedicated node/s for running unikernels. This is to ensure that the unikernels are not competing with other workloads on the same node.
This approach also lends itself to edge environments where disk space and memeory are limited.

A detailed setup and howto can be found [here]()

### Features

### Typical workflow

- Install the ops nanovm tooling
- Create your application (as long as you can create an ELF binary)
- Create a unikernel using the ops nanovm tooling
- Create a OCI image using the unikernel - see the example in the examples directory
- Push your image to a registry
- Setup and configure your Kubernetes/OpenShift cluster (installing qemu-kvm and qemu-system-{ARCH})
- Create a RuntimeClass (see the examples directory)
- Create a pod using the RuntimeClass
- Deploy to your cluster


### Usage

Ensure you have installed the latest Rust toolchain

Clone this repo


```bash
cd rust-ucrun
make build 
```

### Testing

Ensure grcov and  llvm tools-preview are installed

```
cargo install grcov 

rustup component add llvm-tools-preview

```

execute the tests

```
# add the -- --nocapture or --show-ouput flags to see println! statements
$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

# for individual tests
$ CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test create_diff_tar_pass -- --show-output
```

check the code coverage

```
$ grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "src/main.rs" -o target/coverage/html

```

### Coverage Overview
