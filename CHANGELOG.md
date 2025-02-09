# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/RDMA-Rust/rdma-mummy-sys/compare/v0.2.0...v0.2.1) - 2025-02-09

### Added

- *(ibverbs)* port ibv_query_gid_ex

## [0.2.0](https://github.com/RDMA-Rust/rdma-mummy-sys/compare/v0.1.0...v0.2.0) - 2024-09-24

### Added

- *(verbs)* derive Clone, Copy for ibv_wc
- *(verbs)* add _compat suffix to ibv_query_port

### Other

- *(verbs)* return T* directly in inline function

## v0.1.0 (2024-09-01)

### Documentation

 - <csr-id-42c6c8c08b22cb2c420720e683834fbcc78a83e9/> add FujiZ as one of the authors

### New Features

 - <csr-id-b4f582b6db08a908b883c4b023d1f0c283bde521/> generate link layer type manually as they are anonymous enum
 - <csr-id-beda7f4e9e9201be11d65c8454cf94c97692a024/> upgrade rdma-core-mummy for ibv_query_table
 - <csr-id-bf646caa7a24a009d2e2ea072af6b8ce48abdc98/> add binding for driver.h
 - <csr-id-7788fe84ddb4c7e4eb31747e881664a9bcc16305/> upgrade rdma-core-mummy for more symbols
 - <csr-id-ee85a55844321c743998383164e40636b21d26f6/> adapt binding generation for rdma-core-mummy

### Bug Fixes

 - <csr-id-6f03b3585933f1cf9e3be14fa3c13ded90bee8ec/> fix verbs_get_ctx
   Converting u32::MAX to usize will yield a wrong value due to unsigned
   bit extension.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#10](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/10), [#14](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/14), [#19](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/19), [#20](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/20), [#22](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/22)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/10)**
    - Fix build failure ([`fa0b1c9`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/fa0b1c96b0afcdbc9468f44c8abae084519df3ac))
 * **[#14](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/14)**
    - Refine Cargo toml and readme ([`64890a2`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/64890a248e43d32d76b2a83f7fe485f16cff2f7b))
 * **[#19](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/19)**
    - Add ibv_opcode_* constants definition ([`49a36dc`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/49a36dc42382b6526dcbabfd4850f2c3311b8f7c))
 * **[#20](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/20)**
    - Fix bindings type options ([`41f04dc`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/41f04dc2f871332656cf272448bdf6df069f100e))
 * **[#22](https://github.com/RDMA-Rust/rdma-mummy-sys/issues/22)**
    - Change `ibv_evnet_type` from u32 to rust enum. ([`8bdaed8`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/8bdaed8f4353114ca962f12a7376ba03fdf45d2f))
 * **Uncategorized**
    - Release rdma-mummy-sys v0.1.0 ([`c8ade64`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/c8ade64d34edc1e3165b2d9ec9acd0b2915ba17c))
    - Add FujiZ as one of the authors ([`42c6c8c`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/42c6c8c08b22cb2c420720e683834fbcc78a83e9))
    - Generate link layer type manually as they are anonymous enum ([`b4f582b`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/b4f582b6db08a908b883c4b023d1f0c283bde521))
    - Upgrade rdma-core-mummy for ibv_query_table ([`beda7f4`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/beda7f4e9e9201be11d65c8454cf94c97692a024))
    - Add binding for driver.h ([`bf646ca`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/bf646caa7a24a009d2e2ea072af6b8ce48abdc98))
    - Upgrade rdma-core-mummy for more symbols ([`7788fe8`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/7788fe84ddb4c7e4eb31747e881664a9bcc16305))
    - Fix verbs_get_ctx ([`6f03b35`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/6f03b3585933f1cf9e3be14fa3c13ded90bee8ec))
    - Adapt binding generation for rdma-core-mummy ([`ee85a55`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/ee85a55844321c743998383164e40636b21d26f6))
    - Update version ([`ad39d0c`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/ad39d0cba38a5ffe57c3596cfca7a4476c41b61c))
    - Use submodule to setup rdma env ([`aec591d`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/aec591d4ab56be5a3cfe5f5ef43061dcb30e7481))
    - Update version to 0.2.0 ([`7509049`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/75090499b5b060f475caaa93ac8d1f7aa003a175))
    - Test examples in ci ([`9288a38`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/9288a38e9442553d4b57bc2a93583d51e0fd395d))
    - Add cm examples ([`a2b6709`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/a2b67092cb35cd2ff4d0372de395876341479650))
    - Remove parse callback ([`713d867`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/713d867074982c82660ad501bc19a91d52570085))
    - Allow custom rdma-core installation ([`f4cb538`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/f4cb538e391cf0d7c7e883ed54233cef24248990))
    - Bump ci rust toolchain to 1.61.0 ([`33002af`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/33002af7873422beb659ecb0ad1377215512706e))
    - Fix clippy lints ([`b019783`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/b01978383bc97f91b55870d1d87a80fb6b34e281))
    - Change module structure ([`a5be970`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/a5be9702aa28d0c7857318bfa6754f9d35f69459))
    - Change path of bindings.rs ([`1868046`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/18680467ad57d69cadc7c783aa6b559a8826bf49))
    - Cargo build failure ([`5c1a331`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/5c1a33184302f25c27d7b33a6c85afc038db58de))
    - Add macros from rdma_cma.h ([`2bb2c5b`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/2bb2c5bd7e58f5f8bc5584cfacc16c6587664e7c))
    - Remove default ([`fd0d4ab`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/fd0d4ab9fe630ad2c8af377529ab749410fe78b0))
    - Use *mut instead of &mut ([`79f79bb`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/79f79bb36383ef8465799f13ca6d4b132a669801))
    - Binding refactor ([`da6159c`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/da6159c5a5a2c018182374d2f12e7663f4d3bfd2))
    - Libibverbs-dev and librdmacm-dev bindgen ([`b738668`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/b7386686cdb275d01c9f2e8a453ff82b30712a38))
    - First commit ([`65f6637`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/65f6637a6d2c37fc1d7f196b1ed6c3d6d6350687))
    - Initial commit ([`1bb7cf2`](https://github.com/RDMA-Rust/rdma-mummy-sys/commit/1bb7cf2d1cc6644c56bc5d89fc7c0a5843fae9b3))
</details>

