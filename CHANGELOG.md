# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.21.0]

### Added

- DNA: new endpoint get_smart_agreement_all_versions and get_code_template_all_versions [#180](https://github.com/unytco/domino/pull/180)
- Updated role to pass the amount that they provide as <role_id>\_allocation in the smart agreement [#]()

### Updated

- UI updated to handle and mange views depending on the transaction type [#]()

## [0.20.0]

### Update

- new hash space, with new UI updates based on team testing

## [0.19.0]

### Added

- **Role-Based Input Rules in SmartAgreements:** `SmartAgreement` input rules now reference roles defined in the `CodeTemplate`, ensuring a clear and verifiable link between the two. This change streamlines validation by delegating role expectation management to the `CodeTemplate`. [#169](https://github.com/unytco/domino/pull/169)
- **Role Definitions in `CodeTemplate`:** The `agreement_definition_input` in `CodeTemplate` now defines roles and their `consumed_link` expectation. This centralizes role management, simplifying `SmartAgreement` validation. [#169](https://github.com/unytco/domino/pull/169)
- **Validation for `hdi_call` Inputs in SAVED:** `SAVED` now validates that inputs using `hdi_call` collect the correct values, improving data integrity and ensuring that `hdi_call` operations are properly validated.
- **Enhanced Validation for `unyt_allocation` in SAVED:** `SAVED` now validates that the source of a `unyt_allocation` is not reused, preventing duplicate spending and ensuring that all allocations are unique.[#169](https://github.com/unytco/domino/pull/169)
- CodeTemplateExt and SmartAgreementExt returns a vec of tags [#171](https://github.com/unytco/domino/pull/171)
- `ignore_notification` zome call to dismiss transaction notifications [#172](https://github.com/unytco/domino/pull/172)
- Simple transaction signaling to use `post_commit` hook, providing more reliable and detailed real-time updates to all parties involved in a transaction. [#173](https://github.com/unytco/domino/pull/173)
- DNA: `get_all_my_smart_agreements` added to get smart_agreements created by this agent itself [#176](https://github.com/unytco/domino/pull/176)
- `SmartAgreementExt` updated to return version [#176](https://github.com/unytco/domino/pull/176)
- DNA: added endpoint to get status of a tx or agreement based on hash [#177](https://github.com/unytco/domino/pull/177)

### Changed

- **Breaking Change:** `SmartAgreement` no longer defines its own role expectations and instead relies on the `CodeTemplate` to manage them. This change simplifies the `SmartAgreement` structure and centralizes role management.[#169](https://github.com/unytco/domino/pull/169)
- **Refined `hdi_call` Support:** `SmartAgreement` input rules now exclusively support `hdi_calls` to ensure all operations can be validated on the DHT. This improves security and data consistency.
- `create_smart_agreement` and `create_code_template` payloads updated to not expect a separate `tag` parameter. The `tags` are now included in the `SmartAgreement` and `CodeTemplate` structs. [#174](https://github.com/unytco/domino/pull/174)
- Tags are now returned as `TagFilter` objects instead of strings, providing more structured data. [#174](https://github.com/unytco/domino/pull/174)
- When updating `CodeTemplate` or `SmartAgreement` tags, old tags are removed and new tags are added, ensuring the entry's tags are always up-to-date. [#174](https://github.com/unytco/domino/pull/174)
- updated get_code_templates_by_tags to get_code_templates_by_folder
- updated get_all_agents_code_templates to get_my_code_templates

## [0.18.0]

### Changed

- new release flow using p2p-shipyard
- Switching holochain networking to use iroh

## [0.17.0]

### Changed

- renaming variables in the code base to match latest naming convention [#165](https://github.com/unytco/domino/pull/165)
  - Pieces > Units
  - Piece > Unit
  - pieces > units
  - piece > unit
  - SwimLane > ServiceNetwork
  - swimLane > serviceNetwork
  - swim_lane > service_network
  - swimlane > servicenetwork
  - pool > global
  - Pool > Global
  - RAVE > SAVED
  - rave_lib > smart_agreement_library
  - executable_agreement > smart_agreement
  - ExecutableAgreement > SmartAgreement
  - SLSpecialAgents > SNSpecialAgents
  - SLSAVEDAgreements > SNSAVEDAgreements
- code-templates tagged [#166](https://github.com/unytco/domino/pull/166)
- prep for shipyard releases [#167](https://github.com/unytco/domino/pull/167)
- compatible with holochain v0.5.4 [#168](https://github.com/unytco/domino/pull/168)

## [0.16.0]

### Changed

- **Breaking Change:** create_smart_agreement payload updated to accept tags [#](https://github.com/unytco/domino/pull/)
  - New endpoints: search_smart_agreement and get_all_smart_agreements
- New Home UI that wraps all the ui skins [#]
- Handle dynamic credit SAVED, hdk::query handled in instructions [#162](https://github.com/unytco/domino/pull/162)
- **Breaking Change:** dna bumped to run on holochain v0.5.3
- piecework_cli updated to domino_cli for latest work [#164](https://github.com/unytco/domino/pull/164)

## [0.15.0]

### Changed

- New field theme that is expected to be a hex string in ServiceNetworkBasicProperties [#145](https://github.com/unytco/domino/pull/145)
- Domino: Data Migration for version updates [#147](https://github.com/unytco/domino/pull/147)
- UI: basic and advanced mode, new request spend modals [#149](https://github.com/unytco/domino/pull/149)
- code-template versioning [#152](https://github.com/unytco/domino/pull/152)
- **Breaking Change:** optimize memory usage by storing units as maps instead of Vec's [#151](https://github.com/unytco/domino/pull/151)
- **Breaking Change:** create_smart_agreement payload updated to accept tags [#](https://github.com/unytco/domino/pull/)

## [0.14.2]

### Fixed

- Fix UI bug on timestamp on completed transaction table

## [0.14.1]

### Changed

- Fix UI bug on Effective start date display correctly

## [0.14.0]

### Changed

- DNA upgraded to run on holochain v0.5.2 [#133](https://github.com/unytco/domino/pull/133)
- DNA: expose endpoitns to be able to create service units on service-network gen [#134](https://github.com/unytco/domino/pull/134)
- bug: progenitor created check updated in validate [#134](https://github.com/unytco/domino/pull/134)
- Validation document added [#134](https://github.com/unytco/domino/pull/134)
- updated docs and ui naming updates [#135](https://github.com/unytco/domino/pull/135)
- improved code comments [#136](https://github.com/unytco/domino/pull/136)
- service network endpoints to call progenitor for approval [#144](https://github.com/unytco/domino/pull/144)

## [0.13.0]

### Added

- New common `Transaction` type for improved UI experience and consistency [#129](https://github.com/unytco/domino/pull/129)
  - Introduces standardized transaction states: pending, actionable, and completed
  - Replaces multiple transaction types with a unified interface
- DNA: agent_details for managing personal address book [#130](https://github.com/unytco/domino/pull/130)

### Changed

- Changed API endpoints to use the common `Transaction` type: [#129](https://github.com/unytco/domino/pull/129)
  - `get_sorted_requests_to_spend`: Now returns `Vec<Vec<Transaction>>`
  - `accept_paying_parked_invoice`: Now uses `AcceptPayingInvoices` type
  - `get_requests_to_execute_agreements`: Now uses `ExecutionRequests` type
  - `get_parked_spend`: Now returns `Vec<Transaction>`
  - `get_parked_links`: Now returns `Vec<Transaction>`
  - `get_incoming_saveds`: Now returns `Vec<Transaction>`
  - `collect_from_saved`: Now returns `Transaction`
  - `get_all_my_executed_saveds`: Now returns `Vec<Transaction>`
- Changed Language only on dev-hub [#132](https://github.com/unytco/domino/pull/132)

### Removed

- `SAVEDToCollect` type is deprecated in favor of the common `Transaction` type [#129](https://github.com/unytco/domino/pull/129)

### Changed

- ActionTable component for improved transaction management [#129](https://github.com/unytco/domino/pull/129)

## [0.12.0]

### Changed

- Defined common `Transaction` type for all actions[#124](https://github.com/unytco/domino/pull/124)
- Documentation of zome calls[#124](https://github.com/unytco/domino/pull/124)
- Testing: improved tests for credit checks [#125](https://github.com/unytco/domino/pull/125)
- Added possible_actions to the `Transaction` type [#126](https://github.com/unytco/domino/pull/126)
- Testing: improved tests for fees dropoff and collections [#127](https://github.com/unytco/domino/pull/127)
- ui: bug fix on the parked spend form to parse the payload [#127](https://github.com/unytco/domino/pull/127)
- ui: updated interact as button to show descriptions

## [0.11.0]

### Changed

- validation: reviewed validation for links [#118](https://github.com/unytco/domino/pull/118)
- validation: credit check for sales-agent [#118](https://github.com/unytco/domino/pull/118)
- create link from EA to SAVED to collect all the SAVED's that were executed for a EA [#118](https://github.com/unytco/domino/pull/118)
- UI: create custom conversion sheets [#119](https://github.com/unytco/domino/pull/119)
- upgraded UI workspace to maintain multiple UI skins [#121](https://github.com/unytco/domino/pull/121)
- improved rave_engine with tests [#123](https://github.com/unytco/domino/pull/123)

## [0.10.0]

### Changed

- revisiting validations [#112](https://github.com/unytco/domino/pull/112)
- default role_name is always set to lower-case and ui displays as pascal case [#113](https://github.com/unytco/domino/pull/113)
- ui: saved action buttons do not expect an executor if it already provided, and executor button does not show up if you are not the executor [#113](https://github.com/unytco/domino/pull/113)
- data blob: ability to add a record that could be used to reference as an input in the SAVED[#114](https://github.com/unytco/domino/pull/114)
- validate `_lane` SAVED's based on the service-network rules [#115](https://github.com/unytco/domino/pull/115)
- ui: clone and edit SmartAgreement [#115](https://github.com/unytco/domino/pull/115)
- sales-agent: specific endpoints for purchase and redemption [#115](https://github.com/unytco/domino/pull/115)
- ui: use custom sales-agent endpoints [#115](https://github.com/unytco/domino/pull/115)
- documentation: added new architecture docs for purchase and redemption [#115](https://github.com/unytco/domino/pull/115)

### Removed

- removed service_fees SAVED from SNSAVEDAgreements [#114](https://github.com/unytco/domino/pull/114)

## [0.9.1]

### Changed

- ui: persist data on saved transaction models [#111](https://github.com/unytco/domino/pull/111)

## [0.9.0]

### Changed

- code-template: update to allow updates [#82](https://github.com/unytco/domino/pull/82)
- exported saved library into a git submodule [#83](https://github.com/unytco/domino/pull/83)
- exported saved engine into a rust crate [#84](https://github.com/unytco/domino/pull/84)
- fee-collection: use aggregate spend saved to collect fees [#86](https://github.com/unytco/domino/pull/86)
- validation: check that fees owed is not higher than the fee trigger amount [#87](https://github.com/unytco/domino/pull/87)
- update aggregate receive saved to \_\_system_transaction_fee_collection [#88](https://github.com/unytco/domino/pull/88)
- ui: update code template table to show a dropdown arrow to expand the code template table [#89](https://github.com/unytco/domino/pull/89)
- ui: add unclaimed balance to the ledger status [#89](https://github.com/unytco/domino/pull/89)
- ux: add a toggle icon to the global display to expand the global display [#90](https://github.com/unytco/domino/pull/90)
- ui: setup common styles for the interface [#90](https://github.com/unytco/domino/pull/90)
- export saved instructions from the saved engine crate [#91](https://github.com/unytco/domino/pull/91)
- create a client for the domino app called `domino_cli`
- SmartAgreement: add a new field `title` [#92](https://github.com/unytco/domino/pull/95)
- ui: Persist Content in Agreement Interaction Modal(s) [#96](https://github.com/unytco/domino/pull/96)
- ui: add a new field `title` to the SmartAgreement table [#97](https://github.com/unytco/domino/pull/97)
- validation: credit limit needs to be checked with the appropriate global definition and if it has expired [#98](https://github.com/unytco/domino/pull/98)
- ui: rename saved lib to code lib [#100](https://github.com/unytco/domino/pull/100)
- rename: promise -> spend [#101](https://github.com/unytco/domino/pull/101)
- rename global definition variable system_saved_agreement [#102](https://github.com/unytco/domino/pull/102)
- transaction fees and trigger defined in global definition [#104](https://github.com/unytco/domino/pull/104)
- ui: display current applied credit limit for user [#105](https://github.com/unytco/domino/pull/105)
- rename application to domino [#106](https://github.com/unytco/domino/pull/106)
- new sn implementation [#107](https://github.com/unytco/domino/pull/107)
- upgrading a new endpoint to get the current global definition [#108](https://github.com/unytco/domino/pull/108)
- update holochain dependencies to use latest versions (v0.4.2) [#109](https://github.com/unytco/domino/pull/109)
- SmartAgreements with roles to specify input rules [#110](https://github.com/unytco/domino/pull/110)

## [0.8.0]

### Changed

- UI: updated to load saved library from saved library dir [#78](https://github.com/unytco/domino/pull/78)
- SAVED library: added conditional forward and aggregate receive saveds [#79](https://github.com/unytco/domino/pull/79)
- validations: checks the output amount against the spendd amount [#79](https://github.com/unytco/domino/pull/79)
- SAVED library: conditional forward check units transferred [#80](https://github.com/unytco/domino/pull/80)
- UI: handles amount sign based on the endpoints used [#81](https://github.com/unytco/domino/pull/81)

## [0.7.0]

### Changed

- add rhai helper functions for saveds [#74](https://github.com/unytco/domino/pull/74)
- add a Custom SAVED Instructions for inputs [#75](https://github.com/unytco/domino/pull/75)

## [0.6.1]

### Changed

- ui: network sync checks for global definition and code templates required to start transactions [#73](https://github.com/unytco/domino/pull/73)

## [0.6.0]

### Changed

- dna updates for multi unyt support [#65](https://github.com/unytco/domino/pull/65)
- ui updates for multi unyt support [#66](https://github.com/unytco/domino/pull/66)
- ui admin portal to setup global [#67](https://github.com/unytco/domino/pull/67)
- direct transaction upgraded to support multiple unyts [#68](https://github.com/unytco/domino/pull/68)
- endpoints to create an invoice auto updates the reference units to be negative [#70](https://github.com/unytco/domino/pull/70)
- validation: check input and output against code_template schema [#71](https://github.com/unytco/domino/pull/71)
- validation: check parked_spends unyt_allocation against saved outputs unyt_allocation [#72](https://github.com/unytco/domino/pull/72)

## [0.5.0]

- broken release due to build issues in the kangaroo repo

## [0.4.0]

### Changed

- Ability to set a optional list of executors to an SmartAgreement [#50](https://github.com/unytco/domino/pull/50)
- Ability to create a parked spend for multiple parked invoices [#51](https://github.com/unytco/domino/pull/51)
- Update Example Code Templates documents [#51](https://github.com/unytco/domino/pull/51)
- new endpoint to get sorted requests to spend to run aggregate spends [#51](https://github.com/unytco/domino/pull/51)
- add init script to create templates and agreements in the global admin tab [#56](https://github.com/unytco/domino/pull/56)
- update to include all SAVED actions to be triggered from the SmartAgreement Table [#58](https://github.com/unytco/domino/pull/58)
- add/update validation rules [#61](https://github.com/unytco/domino/pull/61)
  - code_template: check titles that can only be commited by the progenitor
  - remove the credit_limit_override from the DNA properties and move that logic based on the progenitor existence
  - accept validates ExecutionInstances
  - saveds validates its executed code
- compatible with holochain v0.4.1 [#62](https://github.com/unytco/domino/pull/62)

### Changed/Fixed

- `SendExecutorParkedInvoiceNotification` link to `SendExecutorParkedSpendNotification` [#49](https://github.com/unytco/domino/pull/49)
- ui code template form reorder inputs boxs [#52](https://github.com/unytco/domino/pull/52)
- ui upgrades that enables aggregate spends [#52](https://github.com/unytco/domino/pull/52)
- Ability to execute a saved without a parked spend or parked invoice [#53](https://github.com/unytco/domino/pull/53)
- bug: notification not showing for the request to spend in saveds [#54](https://github.com/unytco/domino/pull/54)
- update zfuel to 0.2.1 (type updated to u64) [#57](https://github.com/unytco/domino/pull/57)
- standardization of SAVED outputs [#60](https://github.com/unytco/domino/pull/60)
- ui: Actionable Transactions table for spends and invoices easier to interact with [#63](https://github.com/unytco/domino/pull/63)

## [0.3.0]

### Changed

- build process in kangaroo

## [0.2.0]

### Changed

- bump for hashspace

## [0.1.3]

### Fixed

- credit_limit SAVED expects typo fix `claiming_agent_pubkey` [#46](https://github.com/unytco/domino/pull/46)
- update admin tab to global admin and reorder [#47](https://github.com/unytco/domino/pull/47)
- update ServiceNetwork tab to update parent about ServiceNetworks existence [#47](https://github.com/unytco/domino/pull/47)
- update ui to update counts when tables change [#47](https://github.com/unytco/domino/pull/47)

## [0.1.2]

### Changed

- update app icons and hc-spin/holochain-client library [#44](https://github.com/unytco/domino/pull/44)
- Update SAVED flow that removes parked_invoice and parked_spend dependencies on each other [#45](https://github.com/unytco/domino/pull/45)
- remove parked_invoice_signature from code_template and payload from parked_invoice [#45](https://github.com/unytco/domino/pull/45)

## [0.1.1]

### Changed

- implemented a credit-limit check for spends and parked-spends [#40](https://github.com/unytco/domino/pull/40)
- update payment endpoints to check credit and auto apply credit SAVED [#41](https://github.com/unytco/domino/pull/41)
- parked_invoice: change from Record to Link type && parked_spend uses executor as the target [#42](https://github.com/unytco/domino/pull/42)
- ui credit_check on accept_transaction while accepting an invoice [#43](https://github.com/unytco/domino/pull/43)

## [0.1.0]

### Changed

### Fixed

### Removed
