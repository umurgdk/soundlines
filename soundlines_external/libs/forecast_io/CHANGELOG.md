# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

### Changed


## [0.4.0] - 2016-09-13

### Added

- Added a function, `get_forecast_with_options`, to get the forecast
with specified options.

### Changed

- All error fields have been changed from strings to optional f64's


## [0.3.0] - 2016-08-30

### Added

### Changed

- Structs have been replaced with near-fully-optional datapoints and
datablocks.


## [0.2.0] - 2016-08-29

### Added

### Changed

- Forecast::minutely is now optional, as non-UK/USA areas do not contain this
data;
- HourlyData::visibility is now optional due to an oversight


## [0.1.1] - 2016-08-26

### Added

### Changed

- Fixed a typo in an icon definition.


## [0.1.0] - 2016-08-23

Initial commit.


[Unreleased]: https://github.com/zeyla/forecast.io.rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/zeyla/forecast.io.rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/zeyla/forecast.io.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zeyla/forecast.io.rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/zeyla/forecast.io.rs/compare/v0.1.0...v0.1.1
