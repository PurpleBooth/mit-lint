# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v4.0.2 - 2026-06-16
#### Bug Fixes
- (**test**) discard invalid quickcheck cases with comment prefix - (325be2d) - purplebooth

- - -

## v4.0.1 - 2026-06-14
#### Bug Fixes
- with_label_for_line passes byte offset as char column to from_location - (54c7f6f) - Billie Thompson
- replace inadequate quickcheck guard in duplicate_trailers::fail_check - (cc23007) - Billie Thompson
- body-width lint no longer highlights the subject line - (8cd92c3) - Billie Thompson
- GitHub ID regex accepts ids followed by punctuation - (0fbe9b6) - Billie Thompson
- JIRA regex no longer matches lowercase "keys" - (e015ebf) - Billie Thompson
#### Continuous Integration
- use shared install-rust action - (312831b) - Billie Thompson
- migrate from Woodpecker to Forgejo Actions - (90e836e) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update all dependencies - (82259da) - Billie Thompson

- - -

## v4.0.0 - 2026-06-13
#### Features
- <span style="background-color: #d73a49; color: white; padding: 2px 6px; border-radius: 3px; font-weight: bold; font-size: 0.85em;">BREAKING</span>(**not-conventional-commit**) accept hyphens in commit scope - (6f86e2e) - Billie Thompson
#### Bug Fixes
- replace inadequate quickcheck guard in not_conventional_commit::fail_check - (d32f637) - Billie Thompson
- remove unnecessary clones and O(n^2) allocation in Lints::try_from - (209230f) - Billie Thompson
- use correct byte length for multi-byte first char in subject-not-capitalized label - (8f5bfd6) - Billie Thompson
- missing-github-id regex now accepts underscores in repo names - (fc55bb8) - Billie Thompson
- correct typo in doc comment, fix misleading quickcheck guard - (d7439e9) - Billie Thompson
- missing-jira-issue-key excludes scissors section from check - (4605618) - Billie Thompson
- duplicate-trailers labels only highlight actual trailer lines - (be073e6) - Billie Thompson
- problem-builder label length uses byte offsets for Unicode correctness - (bc8fab4) - Billie Thompson
- subject-too-long label uses byte offsets for Unicode correctness - (3b998fe) - Billie Thompson
- subject-not-capitalized label offset uses byte positions of leading whitespace only - (f18cb81) - Billie Thompson
- resolve clippy pedantic warnings (implicit_clone, manual_is_variant_and, collapsible_if, manual_checked_ops) - (c7e87b2) - Billie Thompson
#### Tests
- add regression tests for fail_check quickcheck guard in not_conventional_commit - (badbd23) - Billie Thompson
#### Refactoring
- replace unused_must_use allow with explicit discard, clarify dead_code allow - (9704dd3) - Billie Thompson
- eliminate string allocations in Lint TryFrom<&str> - (3386c95) - Billie Thompson
- remove unnecessary Vec allocations in iterator chains - (0072de9) - Billie Thompson

- - -

## v3.4.1 - 2026-06-12
#### Bug Fixes
- subject period label offset uses byte positions, not char count - (ddd1633) - Billie Thompson
- duplicate trailer label offset uses byte positions correctly - (76d035c) - Billie Thompson
- resolve pre-existing clippy errors - (4430b4d) - Billie Thompson
- parse_conventional_commit handles multi-byte UTF-8 scopes - (71da41d) - Billie Thompson
#### Build system
- remove duplicated lint config - (f173919) - Billie
#### Continuous Integration
- switch to woodpecker - (198b8c7) - Billie
#### Miscellaneous Chores
- (**deps**) update rust docker digest to 087fe68 - (91d6391) - Solace System Renovate Fox
- (**deps**) pin rust docker tag to c0601cf - (ee9f595) - Solace System Renovate Fox
- (**deps**) update rust crate tokio to v1.48.0 - (9933e8f) - Solace System Renovate Fox

- - -

## v3.4.0 - 2025-10-08
#### Continuous Integration
- update fail-on flag to uppercase CRITICAL in concourse pipeline - (5b29663) - Billie Thompson
- reduce resource check interval from 24h to 1h - (7a1738c) - Billie Thompson
- update Concourse pipeline configuration with minor formatting and dependency fixes - (3b3f1ca) - Billie Thompson
- replace docker-rust with ci-rust-env in concourse pipeline - (da0a394) - Billie Thompson
- enable tag fetching for mit-lint repository - (510f698) - Billie Thompson
- add git author and committer details for renovate bot - (86e2b6e) - Billie Thompson
- update GAR resource credentials for docker images - (6f3c068) - Billie Thompson
- update CI runtime image to custom repository and tag - (4f3cce3) - Billie Thompson
- remove Forgejo workflow configuration - (030f2cc) - Billie Thompson
- add Concourse pipeline configuration for CI/CD workflow - (02e0fff) - Billie Thompson
- run less often - (a1727a6) - PurpleBooth
- set a lower retension - (cf7d54c) - PurpleBooth
#### Refactoring
- remove redundant image specifications in Concourse pipeline - (530d4af) - Billie Thompson
- replace grype with trivy in Concourse pipeline tasks - (f348eaf) - Billie Thompson
- Simplify Codeberg release task configuration in Concourse pipeline - (484db60) - Billie Thompson
- Vereinfache Kontrollfluss und Bedingungslogik in Rust-Commit-Prüfungen - (0e77fa0) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update rust crate tokio to v1.47.1 - (337e96c) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (452818e) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v4.3.0 - (ea015d0) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (f0098ea) - Solace System Renovate Fox
- (**deps**) update rust crate criterion to v0.7.0 - (ab3f502) - Solace System Renovate Fox
- (**deps**) update rust crate tokio to v1.46.1 - (f391693) - Solace System Renovate Fox
- remove post-bump git push hooks from cog.toml - (07907a4) - Billie Thompson
- update branch whitelist to include HEAD in cog configuration - (c26d6b3) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (cb0e8e6) - Solace System Renovate Fox [bot]
- (**yamlfix**) apply auto-fixes - (63f8c39) - Solace System Renovate Fox [bot]
- (**yamlfix**) apply auto-fixes - (29015f4) - Solace System Renovate Fox [bot]

- - -

## v3.4.0 - 2025-05-29
#### Bug Fixes
- allow empty descriptions in conventional commits - (5140e31) - Billie Thompson
- fix git push - (75379ae) - PurpleBooth
- Add missing imports to body_wider_than_72_characters_test.rs - (0a66eb1) - Billie Thompson
- add missing imports for Report, GraphicalReportHandler, and TestResult - (f160351) - Billie Thompson
- clone commit message to resolve From trait implementation error - (4db949e) - Billie Thompson
- convert CommitMessage to String using From trait - (72e5f8a) - Billie Thompson
- prevent duplicate trailer detection with improved warning generation - (8db751c) - Billie Thompson
- resolve String::from conversion for CommitMessage by cloning - (1863414) - Billie Thompson
- convert trailer key to str before checking duplicates - (c703692) - Billie Thompson
- update doctests to use public API instead of private modules - (365a697) - Billie Thompson
- resolve ownership issue in README example by cloning message - (f6d9e90) - Billie Thompson
#### Continuous Integration
- switch to less privilaged user for ci - (6b18ae3) - PurpleBooth
- clone from readable-name-generator - (a96e3d1) - PurpleBooth
- add comprehensive GitHub Actions workflow for Rust project - (23286a2) - Billie Thompson
#### Documentation
- reformat code example in README for better readability - (56ffc77) - Billie Thompson
#### Features
- Update mit-commit dependency and refactor body wider than 72 characters check - (d15d41f) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust crate tokio to v1.45.1 - (596d1d2) - Solace System Renovate Fox
- **(deps)** update rust crate criterion to v0.6.0 - (dc578f8) - Solace System Renovate Fox
- **(deps)** pin dependencies - (9fd62a7) - Solace System Renovate Fox
- **(version)** v3.4.0 [skip ci] - (0589564) - PurpleBooth
- remove test-and-tag GitHub workflow - (47e2ae4) - Billie Thompson
- migrate repository from GitHub to Codeberg - (1ec138e) - Billie Thompson (aider)
- update Renovate configuration for library dependencies - (36d4abd) - Billie Thompson
#### Refactoring
- **(body_wider_than_72_characters)** use constant for limit - (11bbabd) - Billie Thompson
- pass line length limit as parameter to improve flexibility - (2cf8c3b) - Billie Thompson
- simplify logic for detecting non-capitalized subjects - (16b832b) - Billie Thompson
- simplify return value construction in line length check - (9875488) - Billie Thompson
- reorganize lint logic and extract helper functions - (0e3d32c) - Billie Thompson
- simplify lint logic and extract helper functions - (5b25607) - Billie Thompson
- Rename test functions to follow naming guidelines - (7405383) - Billie Thompson
- Merge body_wider_than_72_characters test into implementation - (afdd07a) - Billie Thompson
- Merge body_wider_than_72_characters_test.rs into main implementation file - (cece435) - Billie Thompson
- simplify commit text conversion and add comment in test file - (de8d730) - Billie Thompson
- remove unnecessary clone in duplicate trailers lint - (c25f283) - Billie Thompson
- remove unnecessary clone in duplicate_trailers.rs - (942668d) - Billie Thompson
- Update Rust project dependencies and code style - (26ec279) - Billie Thompson
- remove unnecessary `.to_string()` in duplicate trailers check - (a4c29fb) - Billie Thompson
- optimize get_duplicated_trailers to reduce unnecessary string allocations - (8cd722b) - Billie Thompson
- update lint method signature and example for duplicate trailers check - (cde88a8) - Billie Thompson
- optimize duplicate trailers check and add documentation - (7c52d91) - Billie Thompson
#### Style
- improve test naming in body_wider_than_72_characters.rs - (4fa16b3) - Billie Thompson
- apply linter fixes to body_wider_than_72_characters.rs - (137b5e9) - Billie Thompson
- apply linter fixes to body_wider_than_72_characters.rs - (e79f694) - Billie Thompson
- fix spacing and method calls in README example code - (5677965) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).