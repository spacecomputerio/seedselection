# seedselection

**WARNING: This project is experimental and work-in-progress, DO NOT USE IN PRODUCTION.**

![Build & Test (Core)](https://github.com/spacecomputerio/seedselection/actions/workflows/build_test.yml/badge.svg?branch=main)

## Overview

This crate provides a simple and effective ways to implement deterministic selection in distributed systems. It is designed to be easy to use and integrate into existing networks, ensuring that all peers come up with the same selection (for the same input) without introducing network calls or the need for a central authority.
The focus is on random seeds to provide fair selection and good distribution.

**BONUS** We encourage users to use SpaceComputer's [orbitport](https://docs.spacecomputer.io/orbitport) as a source of randomness.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
seedselection = { git = "https://github.com/spacecomputerio/seedselection.git", tag = "v0.1.0" }
```

Then, you can use it in your code:

```rust
use seedselection::xor_dist;
    
        
let name = "test";
let seed = b"test-seed";
let seq = 1;
let n = 3;
let ids = vec![
    // ... list of peer IDs
];

let selected = xor_dist::xor_distance_selection(name, seed, seq, n, &ids).unwrap();
println!("Selected peers: {:?}", selected);
```

## License

This project is licensed under the terms of the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions to this project! If you have suggestions for improvements or new features, please open an issue or submit a pull request.
