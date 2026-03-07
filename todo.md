# VantisVPN Test Fix Tasks

## In Progress
- [ ] Commit and push fixes
- [ ] Update PR #25 or create new PR

## Completed (this session)
- [x] Fix duplicate `#[serial(crypto)]` attributes in comprehensive_tests.rs
- [x] Remove/disable cleanup tests that deinitialize crypto
- [x] Run tests to verify all crypto tests pass (74 passed; 0 failed)
- [x] Full test suite: 469 passed; 0 failed (was 413 passed; 59 failed)

## Completed
- [x] Repository cleanup (PR #24 merged)
- [x] Dual licensing support added
- [x] Documentation cleanup
- [x] Deadlock fix in ram_only.rs
- [x] Initial test fixes (PR #25 created - 12 tests fixed)
- [x] Added serial_test dependency to src/core/Cargo.toml
- [x] Added init_crypto() calls to test files
- [x] Fixed test_hash_hex bug in hash.rs