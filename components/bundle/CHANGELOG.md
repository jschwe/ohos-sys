# Changelog

## 0.1.0

- Initial release. Bindings to `libbundle_ndk.z.so` for API-9 through API-23.
  Covers the native BundleManager surface:
  - `OH_NativeBundle_ApplicationInfo` and
    `OH_NativeBundle_GetCurrentApplicationInfo` (API-9)
  - `OH_NativeBundle_GetAppId` / `…_GetAppIdentifier` (API-11)
  - `OH_NativeBundle_ElementName` and `…_GetMainElementName` (API-13)
  - `OH_NativeBundle_GetCompatibleDeviceType` (API-14)
  - `OH_NativeBundle_IsDebugMode`, `OH_NativeBundle_Metadata`,
    `OH_NativeBundle_ModuleMetadata`, `OH_NativeBundle_GetModuleMetadata`
    (API-20)
  - `BundleManager_ErrorCode`, the opaque
    `OH_NativeBundle_AbilityResourceInfo` and the file-type intent
    discovery flow (`OH_NativeBundle_GetAbilityResourceInfo`,
    `OH_NativeBundle_GetBundleName` / `…_GetModuleName` /
    `…_GetAbilityName` / `…_GetLabel` / `…_GetAppIndex` /
    `…_CheckDefaultApp` / `…_GetDrawableDescriptor` /
    `OH_AbilityResourceInfo_Destroy` / `OH_NativeBundle_GetSize`)
    (API-21)

  `OH_NativeBundle_GetDrawableDescriptor` hands back an
  `ArkUI_DrawableDescriptor` re-exported from `ohos-sys-opaque-types`, so the
  pointer is interchangeable with the one ArkUI APIs produce.
