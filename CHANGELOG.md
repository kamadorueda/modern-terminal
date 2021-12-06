# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD
### Added
- Enumeration `components::table::Size::Weight(f64)`
  to support distributing
  the console width
  not covered by `components::table::Size::Cells(usize)`
  into weighted sections
## [0.7.0] - 2021-12-05
### Added
- Table component
## [0.6.0] - 2021-12-05
### Added
- Enumeration `components::text::TextAlignment`,
  to support left, center, and right alignment
- Module `core::segment`,
  to represent a line of styled text
  that can be written to the console
- Module `core::style`
  to represent the different style options
  (background color, boldness, foreground color)
### Changed
- Module `components::text` for supporting text alignment
- Rename module `base` to `core`
## [0.5.0] - 2021-12-02
### Added
- Crates.io dependency `textwrap`
- Documentation examples and screenshots
- Module `components::text`
### Changed
- Rename function `base::console::Console::from_os` to `base::console::Console::from_fd`
## [0.4.0] - 2021-12-01
### Added
- Function `base::color::model::hsl_to_rgbn`
- Function `base::color::model::hsl_to_rgb`
- Function `base::color::model::rgb_to_hsl`
- Function `base::color::model::rgbn_to_rgb`
- Module `components::color_palette`
- Trait `base::render::Render`
### Removed
- `serde_json` dependency
## [0.3.0] - 2021-12-01
### Added
- Crates.io dependencies: `libc`, `regex` and `serde_json`
- Module `console`
### Changed
- Organized the codebase into a `base` module
## [0.2.0] - 2021-11-30
### Changed
- Rename `Space::Bits2` to `Space::Bits1`.
  This space can hold only 2 colors (1 bit of information)
## [0.1.0] - 2021-11-30

---

[Unreleased]: https://github.com/kamadorueda/modern-terminal/compare/0.7.0...latest
[0.7.0]: https://github.com/kamadorueda/modern-terminal/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/kamadorueda/modern-terminal/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/kamadorueda/modern-terminal/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/kamadorueda/modern-terminal/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/kamadorueda/modern-terminal/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/kamadorueda/modern-terminal/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/kamadorueda/modern-terminal/compare/ef01d19d03c233249c56200534e21a683f4d9a8b...0.1.0
