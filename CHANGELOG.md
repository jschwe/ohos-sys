# Changelog

## v0.9.0

### Added

New sub-crates, each also re-exported as an umbrella feature on `ohos-sys`:

- `accesstoken` — `ohos-accesstoken-sys` (AbilityAccessControl / AccessToken NDK)
- `asset-store` — `ohos-asset-store-sys` (AssetStoreKit)
- `basic-services-kit` — `ohos-basic-services-kit-sys`, with sub-features
  `commonevent`, `battery-info`, `print`, `scan`, `os-account`, `time-service`
- `crypto` — `ohos-crypto-sys` (CryptoArchitectureKit)
- `huks` — `ohos-huks-sys` (Universal Keystore Kit)
- `ipckit` — `ohos-ipckit-sys`
- `locationkit` — `ohos-locationkit-sys`
- `media` — `ohos-media-sys` (Player / Recorder / Transcoder, AVCodec,
  demuxer / muxer, screen capture, low-power AV sinks; sub-crate existed
  previously, this release adds the umbrella re-export)
- `netmanager` / `net_ssl` / `netstack` — NetworkKit subsystems
  (`ohos-netmanager-sys`, `ohos-net-ssl-sys`, `ohos-netstack-sys`)
- `ohaudio` — `ohaudio-sys`
- `qos` — `ohos-libqos-sys` (QoS thread scheduling, plus the Gewu on-device
  LLM inference APIs that share `libqos.so` at API-20+)
- `rdb` — `ohos-rdb-sys` (Relational Database)
- `sensors` — `ohos-sensors-sys`
- `video-processing-engine` — `ohos-video-processing-engine-sys`
  (GPU-accelerated colour-space conversion, HDR metadata, scaling); split
  via `video-processing-engine-video-processing` /
  `video-processing-engine-image-processing`
- `web` — `arkweb-sys`

### Updated

- Update bindings to API-22 and API-23. New `api-22` / `api-23` features
  propagate to all sub-crates.
- New `drawing` modules: `lattice`, `path_iterator` (API-23).
- New `huks` modules: `native_huks_external_crypto_api`,
  `native_huks_external_crypto_type` (API-22).
- New `multimedia/player_framework` modules: `avmedia_base`,
  `avmedia_source`, `avmetakeys` (API-23).
- New `multimodal-input` module: `pointer_style` (API-22).

## v0.8.6 (2026-12-15)

- Fix re-export of udmf and rawfile crates.

## v0.8.5 (2025-12-15)

- Update bindings to API-21

## v0.8.4 (2025-07-31)

- Upgrade `keyboard_types` to `0.8.0` in xcomponent.

## v0.8.3 (2025-07-09)

- Fix issue with linking libpixelmap in imagekit-sys.

## v0.8.2

- Update bindings up to API-18.

## v0.8.1

- Update bindings to API-15

## v0.8.0

- Bump `ohos-image-kit-sys` to 0.3.0 (Result signature change, see 0.7.0 release notes.)

## v0.7.1 

- Fix API level propagation for native window, native image and native buffer.
- Add Pasteboard (`ohos-pasteboard-sys`)

## v0.7.0 (2025-06-06)

### Breaking 

- Update the signature of functions returning Error codes with zero representing the `Ok` value to 
  an equivalent `Result<(), NonZeroErrcode>` type. See the changelogs of the individual crates.

### Add

- Abilitykit (`ohos-abilitykit-sys`)
- Multimodal Input Kit (`ohos-input-sys`)
- Rawfile (`ohos-rawfile-sys`)
- Window Manager (`ohos-window-manager-sys`)

### Update

- Update bindings for OpenHarmony 5.0.2 (API-14)

## v0.6.0 (2025-01-09)

### Breaking 

- napi: `napi_property_descriptor`, `napi_node_version`, `napi_extended_error_info` no longer derive Copy/Clone.
- `xcomponent`: The constant `OH_NATIVE_XCOMPONENT_OBJ` is now a `CStr` instead of raw byte string
- native_window: Duplicate bindings for `native_buffer` types were removed. Use the bindings from `native_buffer` instead.
- native_buffer: `OH_NativeBuffer_MetadataType` is now a wrapper around `c_int` instead of `c_uint`.

### Add

- Made vsync bindings also available as dedicated ohos-vsync-sys crate. No user facing changes.
- Updated all bindings for API-13.

## v0.5.0 (2025-01-04)

### Breaking

- Remove `Debug` from opaque structs

### Features

- Internal changes to the bindings, to more easily allow feature guarding based on the API level.
- Improved the documentation, by converting doxygen comments to markdown.

## v0.4.0 (2024-10-29)

### Change

- Unify re-export of components
- Bump ime-sys, xcomponent-sys and drawing-sys

## v0.3.1 (2024-09-05)

### Add

- Re-export hitrace-sys binding (behind hitrace feature)

## v0.3.0 (2024-08-29)

### Breaking

- Change enum types in `native_buffer` and `native_window` to newtype pattern.

### Added

- deviceinfo bindings 
- native_buffer API-12 bindings
- native_image bindings
- syscap bindings

## v0.2.2 (2024-08-18)

### Added

- Added bindings for `native_vsync` (behind the `vsync` feature flag)

## v0.2.1

### Fixed

- `ohos-drawing-sys` is now an optional dependency. Usage was already guarded behind the `drawing`
  feature.

## v0.2.0 (2024-07-18)

### Breaking

- Renamed and moved the xcomponent module to the top-level  (from ace/xcomponent/native_interface_xcomponent)
- Guard each component behind a feature flag

### Added

- Added `native_drawing` API bindings (Also available separately as `ohos-drawing-sys` )
- Added bindings for API level 11 behind a feature flag

### Fixed

- `native_window` now links against the correct dynamic library.
- Remove Copy / Clone impls on opaque structs
