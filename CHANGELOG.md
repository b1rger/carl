# Changelog

## [0.6.0](https://github.com/b1rger/carl/compare/v0.5.1...v0.6.0) (2026-01-17)


### Features

* replace xdg with directories crate to enable more platforms ([494b2ce](https://github.com/b1rger/carl/commit/494b2ce0644420e136913fa5d53fdb45abd6f032))

## [0.5.1](https://github.com/b1rger/carl/compare/v0.5.0...v0.5.1) (2025-12-17)


### Bug Fixes

* **deps:** bump icalendar version to 0.17.6 ([a943942](https://github.com/b1rger/carl/commit/a943942bb732eb97a7516940ca439ffe6b0a852d)), closes [#182](https://github.com/b1rger/carl/issues/182)

## [0.5.0](https://github.com/b1rger/carl/compare/v0.4.0...v0.5.0) (2025-12-07)


### Features

* **dateproperties:** add "today" related date properties ([6457745](https://github.com/b1rger/carl/commit/64577453cfa503fc6e533fefec5c430f3e18444c))
* **properties:** add "specified date" related properties ([db79c1a](https://github.com/b1rger/carl/commit/db79c1a7a93c97ba931283959b6ef855c4419a4c))
* **theme:** adapt default theme ([f326dbf](https://github.com/b1rger/carl/commit/f326dbf5144fe2287c243d445b049c5cfdf71b49))

## [0.4.0](https://github.com/b1rger/carl/compare/v0.3.1...v0.4.0) (2025-09-28)


### Features

* add a simple `style` template method ([8219932](https://github.com/b1rger/carl/commit/82199320fa451d23465a2191da6a4ddfc1f74d31))
* implement -n, --months &lt;num&gt; ([0bf5887](https://github.com/b1rger/carl/commit/0bf58875e68b7aa19a3cc74442ed1ebc26fca71a))
* **templates:** use `style` template method instead of terminal seq ([643f138](https://github.com/b1rger/carl/commit/643f13801f76f292a36b8e3bcc42d103e9140669))
* use jinja templates to render output of carl ([c175ec3](https://github.com/b1rger/carl/commit/c175ec33beb66372f15231c4921f136bf9b1976c))


### Bug Fixes

* don't print more weekdays headers than we have months in the chunk ([ad60895](https://github.com/b1rger/carl/commit/ad608951c4ebe2181af8f1aaecaff2526f6cebc7))

## [0.3.1](https://github.com/b1rger/carl/compare/v0.3.0...v0.3.1) (2024-08-31)


### Bug Fixes

* correct getting months for -3 ([7cdeda0](https://github.com/b1rger/carl/commit/7cdeda0e1f4851f62e7c13d136022ea4c778c955))

## [0.3.0](https://github.com/b1rger/carl/compare/v0.2.1...v0.3.0) (2024-01-13)


### ⚠ BREAKING CHANGES

* **cli:** allow exclusive actions

### Features

* **agenda:** implement agenda with eventinstances ([8b008aa](https://github.com/b1rger/carl/commit/8b008aaee7a54d411bc558be2bbd1773875ae734))
* **cli:** allow exclusive actions ([1f2a7c0](https://github.com/b1rger/carl/commit/1f2a7c00e634fca5bc9e287d4f68c431a4fce7b9)), closes [#97](https://github.com/b1rger/carl/issues/97)
* implement year progress feature ([9652441](https://github.com/b1rger/carl/commit/96524414d67101c1c3f3e2e8755eadb8b0ef4d63)), closes [#92](https://github.com/b1rger/carl/issues/92)


### Bug Fixes

* also show julian date in agenda ([3ca35b3](https://github.com/b1rger/carl/commit/3ca35b3b74eb473d1758376754ff64c97f9fb3fa)), closes [#100](https://github.com/b1rger/carl/issues/100)
* readd agenda header and style it ([a74337a](https://github.com/b1rger/carl/commit/a74337ab219e2fc37a7c90b9a898e998705bd222))

## [0.2.1](https://github.com/b1rger/carl/compare/v0.2.0...v0.2.1) (2024-01-06)


### Bug Fixes

* replace wildcard dependency with versioned one ([b3f40ca](https://github.com/b1rger/carl/commit/b3f40cad249e4b62208a741b10f690bd229b4d88))

## [0.2.0](https://github.com/b1rger/carl/compare/v0.1.2...v0.2.0) (2024-01-06)


### Features

* implement calendar parsing using icalendar ([9705c2b](https://github.com/b1rger/carl/commit/9705c2bdbcf5221125341a8b55eb862ad7ef7c4f))
* implement DateRange ([500c685](https://github.com/b1rger/carl/commit/500c68532ec345cc2b094fae12e4a5bd7b55d0af))
* use rrule instead of parsing RRULE manually ([7480a89](https://github.com/b1rger/carl/commit/7480a896969808a9a3ea6588a4de402bd62a4cbb))

## [0.1.2](https://github.com/b1rger/carl/compare/v0.1.1...v0.1.2) (2023-12-05)


### Bug Fixes

* drop `colors` keyword, because there can only be 5 keywords ([b1ad930](https://github.com/b1rger/carl/commit/b1ad9300ee28438ad07e3f6a7b9093e28ecf2a9b))

## [0.1.1](https://github.com/b1rger/carl/compare/v0.1.0...v0.1.1) (2023-12-05)


### Bug Fixes

* **cargo:** update metadata in Cargo.toml ([42159e1](https://github.com/b1rger/carl/commit/42159e1ef57e9eee4db468da9e4d47e0ff7c3b72))
* don't crash when dealing with weekly events ([5108474](https://github.com/b1rger/carl/commit/5108474b2fa6b198d32990433bc6285b0e344eaa)), closes [#46](https://github.com/b1rger/carl/issues/46)
* unnecessarily eager cloning of iterator items ([bdc334b](https://github.com/b1rger/carl/commit/bdc334b524b9fb573051d35574137bbc98670fd6))

## [0.1.0](https://github.com/b1rger/carl/compare/v0.0.4...v0.1.0) (2023-07-29)


### Features

* add new date properties ([0e2b0ee](https://github.com/b1rger/carl/commit/0e2b0eef58868b3f3b1e51ac5293838b7b7c0d15))
* **dependencies:** update clap to 4.0.0 ([7caaf64](https://github.com/b1rger/carl/commit/7caaf6403703020353235a934a58c02fa184964e))


### Bug Fixes

* **dependencies:** move from nu_ansi_term to anstyle ([56bbd91](https://github.com/b1rger/carl/commit/56bbd91caec0273030fa78da7f4f24f5eaba8e17)), closes [#13](https://github.com/b1rger/carl/issues/13)
* **dependencies:** use chrono without standard features ([bd913f8](https://github.com/b1rger/carl/commit/bd913f89b9b3b7336a0b8298cb0f5bdff66fb143)), closes [#9](https://github.com/b1rger/carl/issues/9)
* **doc:** update default.theme link in README ([bdabdf2](https://github.com/b1rger/carl/commit/bdabdf229560402496c0575b7ec990eab9d0949d))

## [0.0.4](https://github.com/b1rger/carl/compare/0.0.3...v0.0.4) (2023-07-03)


### Bug Fixes

* **dependencies:** downgrade toml dependency ([d021a30](https://github.com/b1rger/carl/commit/d021a3022c1f624baf42847988eac911b864d00d))
* **dependencies:** switch from ansi_term to nu-ansi-term ([11e5790](https://github.com/b1rger/carl/commit/11e579091718e1eb320f0522087e0cbe85c34a7b)), closes [#10](https://github.com/b1rger/carl/issues/10)
