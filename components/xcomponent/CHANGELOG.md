# Changelog

## 0.4.0

- `ace_ndk.z` is now only linked when compiling for ohos.
  This allows running tests using constants on the host.
- `keyboard-types` feature: extend the `Code` conversion.
  Changed: `KEY_PLAYPAUSE` now maps to `MediaPlayPause` instead of `Pause`.
- `keyboard-types` feature: add `named_key`, `location`, the US-layout
  character lookup `us_char`.

## 0.3.6

- Add support up to API-23. Adds `OH_ArkUI_XComponentSurfaceConfig` and
  `OH_ArkUI_SurfaceHolder_SetSurfaceConfig` at API-22.

## 0.3.5

- Add support up to API-21

## 0.3.4

- Upgrade keyboard_types to 0.8.0

## 0.3.3

- Update to API-16, 17 and 18.

## 0.3.2

- Add bindings for API-15

## 0.3.1 

- Updated bindings for API-14

## 0.3.0 (2025-01-09)

- The constant `OH_NATIVE_XCOMPONENT_OBJ` is now a `CStr` instead of raw byte string
- Updated bindings for API-13

## 0.2.0 (2025-01-04)

- Regenerated the bindings with new generator
