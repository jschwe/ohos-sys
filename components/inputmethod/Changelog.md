# Changelog

## 0.2.4

- Update bindings to API-22 and API-23. Adds
  `OH_TextEditorProxy_SetCallbackInMainThread` (API-22) and
  `OH_InputMethodController_AttachWithUIContext` (API-23).

## 0.2.3

- Update to API-21 bindings

## 0.2.2

- Update to API-18 bindings (no changes)

## 0.2.1

- Add API-15 bindings

## 0.2.0 

### Breaking 

- Replace `InputMethod_ErrorCode` with `InputMethodResult` (an alias to `Result<(), NonZero<InputMethodErrorCode>>`)


## 0.1.4 (2025-01-09)

- Added API-13 and API-14 feature (no changess)

# v0.1.2 (2025-01-04)

- Internal changes: Regenerate bindings with new generator

# v0.1.1

### Add

- Add convenience `is_ok()` method on error type
