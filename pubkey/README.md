# pubkey

Serde Solana Pubkey serializer/deserializer.

## Installation

Add `pubkey` to your `Cargo.toml`.

## Usage

```rust
use solana_program::pubkey::Pubkey;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyStruct {
    /// Token pubkey.
    #[serde(with = "pubkey")]
    pub address: Pubkey,
}
```

## License

The `pubkey` crate is licensed under the Apache 2.0 License.

License: Apache-2.0
