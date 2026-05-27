# OpenHarmony BundleManager bindings

Low-level bindings to the OpenHarmony BundleManager (native bundle) NDK,
exposed by `libbundle_ndk.z.so`. The C API surface lives under
`bundle/native_interface_bundle.h` and provides:

- application identity (`bundleName`, `appId`, `appIdentifier`, fingerprint,
  compatible device type, debug-mode flag)
- main-element discovery (`OH_NativeBundle_GetMainElementName`)
- module metadata (`OH_NativeBundle_GetModuleMetadata`)
- file-type intent discovery (`OH_NativeBundle_GetAbilityResourceInfo`,
  API-21+) — enumerate the abilities able to open a given file type

C API reference:

- [Native_Bundle module overview](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ability-kit/capi-native-interface-bundle-h.md)
- [BundleManager common types](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ability-kit/capi-bundle-manager-common-h.md)
- [`OH_NativeBundle_AbilityResourceInfo`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ability-kit/capi-native-bundle-oh-nativebundle-abilityresourceinfo.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
