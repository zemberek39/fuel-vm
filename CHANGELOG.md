# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

Description of the upcoming release here.

## [Version 0.35.0]

The release mostly fixes funding during the audit and integration with the bridge. But the release also contains some new features like:
- Asynchronous predicate estimation/verification.
- Multi-asset support per contract.
- Support Secp256r1 signature recovery and Ed25519 verificaiton.


### Added

- [#486](https://github.com/FuelLabs/fuel-vm/pull/486/): Adds `ed25519` signature verification and `secp256r1` signature recovery to `fuel-crypto`, and corresponding opcodes `ED19` and `ECR1` to `fuel-vm`.

- [#500](https://github.com/FuelLabs/fuel-vm/pull/500): Introduced `ParallelExecutor` trait
    and made available async versions of verify and estimate predicates.
    Updated tests to test for both parallel and sequential execution.
    Fixed a bug in `transaction/check_predicate_owners`.

#### Breaking

- [#506](https://github.com/FuelLabs/fuel-vm/pull/506): Added new `Mint` and `Burn` variants to `Receipt` enum.
    It affects serialization and deserialization with new variants.

### Changed

#### Breaking

- [#506](https://github.com/FuelLabs/fuel-vm/pull/506): The `mint` and `burn` 
    opcodes accept a new `$rB` register. It is a sub-identifier used to generate an 
    `AssetId` by [this rule](https://github.com/FuelLabs/fuel-specs/blob/SilentCicero-multi-token/src/identifiers/asset.md). 
    This feature allows having multi-asset per one contract. It is a huge breaking change, and 
    after this point, `ContractId` can't be equal to `AssetId`.

    The conversion like `AssetId::from(*contract_id)` is no longer valid. Instead, the `ContractId` implements the `ContractIdExt` trait:
    ```rust
    /// Trait extends the functionality of the `ContractId` type.
    pub trait ContractIdExt {
        /// Creates an `AssetId` from the `ContractId` and `sub_id`.
        fn asset_id(&self, sub_id: &Bytes32) -> AssetId;
    }
    ```

- [#506](https://github.com/FuelLabs/fuel-vm/pull/506): The `mint` and `burn` 
    opcodes affect the `receipts_root` of the `Script` transaction.

### Removed

#### Breaking

- [#486](https://github.com/FuelLabs/fuel-vm/pull/486/): Removes apparently unused `Keystore` and `Signer` traits from `fuel-crypto`. Also renames `ECR` opcode to `ECK1`.

### Fixed

- [#500](https://github.com/FuelLabs/fuel-vm/pull/500): Fixed a bug where `MessageCoinPredicate` wasn't checked for in `check_predicate_owners`.

#### Breaking

- [#502](https://github.com/FuelLabs/fuel-vm/pull/502): The algorithm used by the
    binary Merkle tree for generating Merkle proofs has been updated to remove
    the leaf data from the proof set. This change allows BMT proofs to conform
    to the format expected by the Solidity contracts used for verifying proofs.

- [#503](https://github.com/FuelLabs/fuel-vm/pull/503): Use correct amount of gas in call
    receipts when limited by cgas. Before this change, the `Receipt::Call` could show an incorrect value for the gas limit.

- [#504](https://github.com/FuelLabs/fuel-vm/pull/504): The `CROO` and `CSIZ` opcodes require 
    the existence of corresponding `ContractId` in the transaction's 
    inputs(the same behavior as for the `CROO` opcode).

- [#504](https://github.com/FuelLabs/fuel-vm/pull/504): The size of the contract 
    was incorrectly padded. It affects the end of the call frame in the memory, 
    making it not 8 bytes align. Also, it affects the cost of the contract 
    call(in some cases, we charged less in some more).

- [#504](https://github.com/FuelLabs/fuel-vm/pull/504): The charging for `DependentCost`
    was done incorrectly, devaluing the `dep_per_unit` part. After the fixing of 
    this, the execution should become much more expensive.

- [#505](https://github.com/FuelLabs/fuel-vm/pull/505): The `data` field of the `Receipt` 
    is not part of the canonical serialization and deserialization anymore. The SDK should use the 
    `Receipt` type instead of `OpaqueReceipt`. The `Receipt.raw_payload` will be removed for the 
    `fuel-core 0.20`. The `data` field is optional now. The SDK should update serialization and 
    deserialization for `MessageOut`, `LogData`, and `ReturnData` receipts.

- [#505](https://github.com/FuelLabs/fuel-vm/pull/505): The `len` field of the `Receipt` 
    is not padded anymore and represents an initial value.

## [Version 0.34.1]

Mainly new opcodes prices and small performance improvements in the `BinaryMerkleTree`.

### Changed

- [#492](https://github.com/FuelLabs/fuel-vm/pull/492): Minor improvements to BMT
    internals, including a reduction in usage of `Box`, using `expect(...)` over
    `unwrap()`, and additional comments.

#### Breaking

- [#493](https://github.com/FuelLabs/fuel-vm/pull/493): The default `GasCostsValues`
    is updated according to the benches with `fuel-core 0.19`. 
    It may break some unit tests that compare actual gas usage with expected.

## [Version 0.34.0]

This release contains fixes for critical issues that we found before the audit. 
Mainly, these changes pertain to the Sparse Merkle Tree (SMT) and related 
code. The SMT API was extended to provide more flexibility and to allow users 
to select the most appropriate method for their performance needs. Where 
possible, sequential SMT updates were replaced with constructors that take in a
complete data set.

### Added

- [#476](https://github.com/FuelLabs/fuel-vm/pull/476): The `fuel_vm::Call` supports `From<[u8; Self::LEN]>` and `Into<[u8; Self::LEN]>`.

- [#484](https://github.com/FuelLabs/fuel-vm/pull/484): The `sparse::in_memory::MerkleTree`
    got new methods `from_set`, `root_from_set`, and `nodes_from_set` methods. These methods allow
    a more optimal way to build and calculate the SMT when you know all leaves.
    The `Contract::initial_state_root` is much faster now (by ~15 times).

### Removed

- [#478](https://github.com/FuelLabs/fuel-vm/pull/478): The `CheckedMemRange` is replaced by the `MemoryRange`.

### Changed

- [#477](https://github.com/FuelLabs/fuel-vm/pull/477): The `PanicReason::UnknownPanicReason` is `0x00`.
    The `PanicReason` now implements `From<u8>` instead of `TryFrom<u8>` and can't return an error anymore.

- [#478](https://github.com/FuelLabs/fuel-vm/pull/478): The `memcopy` method is updated
    and returns `MemoryWriteOverlap` instead of `MemoryOverflow`.

### Fixed

- [#482](https://github.com/FuelLabs/fuel-vm/pull/482): This PR address a security 
    issue where updates to a Sparse Merkle Tree could deliberately overwrite existing
    leaves by setting the leaf key to the hash of an existing leaf or node. This is 
    done by removing the insertion of the leaf using the leaf key.

- [#484](https://github.com/FuelLabs/fuel-vm/pull/484): Fixed bug with not-working `CreateMetadata`.


#### Breaking

- [#473](https://github.com/FuelLabs/fuel-vm/pull/473): CFS and CFSI were not validating
    that the new `$sp` value isn't below `$ssp`, allowing write access to non-owned
    memory. This is now fixed, and attempting to set an incorrect `$sp` value panics.

- [#485](https://github.com/FuelLabs/fuel-vm/pull/485): This PR addresses a security
    issue where the user may manipulate the structure of the Sparse Merkle Tree. 
    SMT expects hashed storage key wrapped into a `MerkleTreeKey` structure. 
    The change is breaking because it changes the `state_root` generated by the SMT 
    and may change the `ContractId` if the `Create` transaction has non-empty `StoargeSlot`s.


## [Version 0.33.0]

The release contains a lot of breaking changes. 
Most of them are audit blockers and affect the protocol itself.
Starting this release we plan to maintain the changelog file and describe all minor and major changes that make sense.

### Added

#### Breaking

- [#386](https://github.com/FuelLabs/fuel-vm/pull/386): The coin and message inputs 
    got a new field - `predicate_gas_used`. So it breaks the constructor API 
    of these inputs.

    The value of this field is zero for non-predicate inputs, but for the 
    predicates, it indicates the exact amount of gas used by the predicate 
    to execute. If after the execution of the predicate remaining gas is not 
    zero, then the predicate execution failed.
    
    This field is malleable but will be used by the VM, and each predicate 
    should be estimated before performing the verification logic. 
    The `Transaction`, `Create`, and `Script` types implement the 
    `EstimatePredicates` for these purposes.

    ```rust
    /// Provides predicate estimation functionality for the transaction.
    pub trait EstimatePredicates: Sized {
        /// Estimates predicates of the transaction.
        fn estimate_predicates(&mut self, params: &ConsensusParameters, gas_costs: &GasCosts) -> Result<(), CheckError>;
    }
    ```

    During the creation of the `Input`, the best strategy is to use a default 
    value like `0` and call the `estimate_predicates` method to actualize 
    the `predicate_gas_used` after.

- [#454](https://github.com/FuelLabs/fuel-vm/pull/454): VM native array-backed types 
`Address`, `AssetId`, `ContractId`, `Bytes4`, `Bytes8`, `Bytes20`, `Bytes32`, 
`Nonce`, `MessageId`, `Salt` now use more compact representation instead of 
hex-encoded string when serialized using serde format that sets 
`is_human_readable` to false.

- [#456](https://github.com/FuelLabs/fuel-vm/pull/456): Added a new type - `ChainId` to represent the identifier of the chain. 
It is a wrapper around the `u64`, so any `u64` can be converted into this type via `.into()` or `ChainId::new(...)`.

- [#459](https://github.com/FuelLabs/fuel-vm/pull/459) Require witness index to be specified when adding an unsigned coin to a transaction.
This allows for better reuse of witness data when using the transaction builder and helper methods to make transactions compact.

- [#462](https://github.com/FuelLabs/fuel-vm/pull/462): Adds a `cache` parameter to `Input::check` and `Input::check_signature`.
  This is used to avoid redundant signature recovery when multiple inputs share the same witness index.

### Changed

- [#458](https://github.com/FuelLabs/fuel-vm/pull/458): Automatically sort storage slots for creation transactions.

#### Breaking

- [#386](https://github.com/FuelLabs/fuel-vm/pull/386): Several methods of the `TransactionFee` are renamed `total` -> `max_fee`
  and `bytes` -> `min_fee`. The `TransactionFee::min_fee` take into account the gas used by predicates.

- [#450](https://github.com/FuelLabs/fuel-vm/pull/450): The Merkle root of a contract's code is now calculated by partitioning the code into chunks of 16 KiB, instead of 8 bytes. If the last leaf is does not a full 16 KiB, it is padded with `0` up to the nearest multiple of 8 bytes. This affects the `ContractId` and `PredicateId` calculations, breaking all code that used hardcoded values.

- [#456](https://github.com/FuelLabs/fuel-vm/pull/456): The basic methods `UniqueIdentifier::id`, `Signable::sign_inputs`, 
and `Input::predicate_owner` use `ChainId` instead of the `ConsensusParameters`. 
It is a less strict requirement than before because you can get `ChainId` 
from `ConsensusParameters.chain_id`, and it makes the API cleaner. 
It affects all downstream functions that use listed methods.

- [#463](https://github.com/FuelLabs/fuel-vm/pull/463): Moves verification that the `Output::ContractCreated` 
output contains valid `contract_id` and `state_root`(the values from the `Output` match with calculated 
values from the bytecode, storage slots, and salt) from `fuel-vm` to `fuel-tx`. 
It means the end-user will receive this error earlier on the SDK side before `dry_run` instead of after.

### Fixed

#### Breaking

- [#457](https://github.com/FuelLabs/fuel-vm/pull/457): Transactions got one more validity rule: 
Each `Script` or `Create` transaction requires at least one input coin or message to be spendable. 
It may break code/tests that previously didn't set any spendable inputs. 
Note: `Message` with non-empty `data` field is not spendable.

- [#458](https://github.com/FuelLabs/fuel-vm/pull/458): The storage slots with the same key inside the `Create` transaction are forbidden.
