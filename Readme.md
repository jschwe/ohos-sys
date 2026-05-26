# ohos-sys

FFI-bindings for the native API of [OpenHarmony OS]. See the [documentation] for a list of supported components.
This crate is under active development, and not officially affiliated with OpenHarmony OS.

## Status of the bindings

Here is an overview of the available C header directories in the OpenHarmony sysroot, and for which of them
this crate already provides bindings. The API-level column denotes up to which api level the bindings have
already been generated.

| API name                                | status | API-level | crate                     |
|-----------------------------------------|--------|-----------|---------------------------|
| AbilityKit                              | ✅      | 23        | [ohos-abilitykit-sys]     |
| BasicServicesKit                        | ✅      | 23        | [ohos-basic-services-kit-sys] |
| Background Process Manager              |        |           |                           |
| ConnectivityKit                         |        |           |                           |
| CryptoArchitectureKit                   | ✅      | 23        | [ohos-crypto-sys]         |
| DataProtectionKit                       |        |           |                           |
| GameControllerKit                       |        |           |                           |
| IPCKit                                  | ✅      | 23        | [ohos-ipckit-sys]         |
| LocationKit                             | ✅      | 23        | [ohos-locationkit-sys]    |
| NotificationKit                         |        |           |                           |
| TEEKit                                  |        |           |                           |
| accesstoken                             | ✅      | 23        | [ohos-accesstoken-sys]    |
| ace/xcomponent                          | ✅      | 23        | [xcomponent-sys]          |
| ark_runtime                             |        |           |                           |
| arkui                                   | ✅      | 23        | [arkui-sys]               |
| asset                                   | ✅      | 23        | [ohos-asset-store-sys]    |
| bundle                                  |        |           |                           |
| database                                | ✅      | 23        | [ohos-rdb-sys]            |
| ddk                                     |        |           |                           |
| distributedhardware                     |        |           |                           |
| ffrt                                    |        |           |                           |
| filemanagement                          |        |           |                           |
| hiappevent                              |        |           |                           |
| hicollie                                |        |           |                           |
| hid                                     |        |           |                           |
| hidebug                                 |        |           |                           |
| hilog                                   | ✅      | 23        | [hilog-sys]               |
| hitrace                                 | ✅      | 23        | [hitrace-sys]             |
| huks                                    | ✅      | 23        | [ohos-huks-sys]           |
| info                                    | ✅      | 23        | [ohos-deviceinfo-sys]     |
| inputmethod                             | ✅      | 23        | [ohos-ime-sys]            |
| mindspore                               |        |           |                           |
| multimedia/av_session                   |        |           |                           |
| multimedia/drm_framework                |        |           |                           |
| multimedia/image_effect                 |        |           |                           |
| multimedia/image_framework              | ✅      | 23        | [ohos-image-kit-sys]      |
| multimedia/media_library                |        |           |                           |
| multimedia/player_framework             | ✅      | 23        | [ohos-media-sys]          |
| multimedia/video_processing_engine      | ✅      | 23        | [ohos-video-processing-engine-sys] |
| multimodalinput                         | ✅      | 23        | [ohos-input-sys]          |
| napi                                    | ✅      | 23        |                           |
| native_buffer                           | ✅      | 23        | [ohos-window-sys]         |
| native_color_space_manager              |        |           |                           |
| native_display_soloist                  |        |           |                           |
| native_drawing                          | ✅      | 23        | [ohos-drawing-sys]        |
| native_effect                           |        |           |                           |
| native_fence                            | ✅      | 23        | [ohos-window-sys]         |
| native_image                            | ✅      | 23        | [ohos-window-sys]         |
| native_vsync                            | ✅      | 23        | [ohos-vsync-sys]          |
| native_window                           | ✅      | 23        | [ohos-window-sys]         |
| network/netmanager                      | ✅      | 23        | [ohos-netmanager-sys]     |
| network/netstack                        | ✅      | 23        | [ohos-netstack-sys]       |
| network/netstack/net_ssl                | ✅      | 23        | [ohos-net-ssl-sys]        |
| neural_network_runtime                  |        |           |                           |
| ohaudio                                 | ✅      | 23        | [ohaudio-sys]             |
| ohcamera                                |        |           |                           |
| Pasteboard                              | ✅      | 23        | [ohos-pasteboard-sys]     |
| purgeable_memory                        |        |           |                           |
| qos                                     | ✅      | 23        | [ohos-libqos-sys]         |
| rawfile                                 | ✅      | 23        | [ohos-rawfile-sys]        |
| resourcemanager                         |        |           |                           |
| sensors                                 | ✅      | 23        | [ohos-sensors-sys]        |
| SCSI Peripherals                        |        |           |                           |
| telephony                               |        |           |                           |
| transient_task                          |        |           |                           |
| Unified Data Management Framework(UDMF) | ✅      | 23        | [udmf-sys]                |
| usb                                     |        |           |                           |
| usb serial                              |        |           |                           |
| web                                     | ✅      | 23        | [arkweb-sys]            |
| window_manager                          | ✅      | 23        | [ohos-window-manager-sys] |

[arkui-sys]: https://docs.rs/arkui-sys/latest/arkui_sys/
[hilog-sys]: https://docs.rs/hilog-sys/latest/hilog_sys/
[hitrace-sys]: https://docs.rs/hitrace-sys/latest/hitrace_sys/
[ohos-drawing-sys]: https://docs.rs/ohos-drawing-sys/latest/ohos_drawing_sys/
[ohos-ime-sys]: https://docs.rs/ohos-ime-sys/latest/ohos_ime_sys/
[ohos-input-sys]: https://docs.rs/ohos-input-sys/latest/ohos_input_sys/
[ohos-ipckit-sys]: https://docs.rs/ohos-ipckit-sys/latest/ohos_ipckit_sys/
[ohos-locationkit-sys]: https://docs.rs/ohos-locationkit-sys/latest/ohos_locationkit_sys/
[ohos-netmanager-sys]: https://docs.rs/ohos-netmanager-sys/latest/ohos_netmanager_sys/
[ohos-net-ssl-sys]: https://docs.rs/ohos-net-ssl-sys/latest/ohos_net_ssl_sys/
[ohos-netstack-sys]: https://docs.rs/ohos-netstack-sys/latest/ohos_netstack_sys/
[ohos-image-kit-sys]: https://docs.rs/ohos-image-kit-sys/latest/ohos_image_kit_sys/
[ohos-media-sys]: https://docs.rs/ohos-media-sys/latest/ohos_media_sys/
[ohos-video-processing-engine-sys]: https://docs.rs/ohos-video-processing-engine-sys/latest/ohos_video_processing_engine_sys/
[ohaudio-sys]: https://docs.rs/ohaudio-sys/latest/ohaudio_sys/
[ohos-pasteboard-sys]: https://docs.rs/ohos-pasteboard-sys/latest/ohos_pasteboard_sys/
[ohos-rawfile-sys]: https://docs.rs/ohos-rawfile-sys/latest/ohos_rawfile_sys/
[rdb-sys]: https://docs.rs/ohos-rdb-sys/latest/ohos-rdb_sys/
[ohos-window-manager-sys]: https://docs.rs/ohos-window-manager-sys/latest/ohos_window_manager_sys/
[ohos-window-sys]: https://docs.rs/ohos-window-sys/latest/ohos_window_sys/
[arkweb-sys]: https://docs.rs/arkweb-sys/latest/arkweb_sys/
[ohos-abilitykit-sys]: https://docs.rs/ohos-abilitykit-sys/latest/ohos_abilitykit_sys/
[ohos-vsync-sys]: https://docs.rs/ohos-vsync-sys/latest/ohos_vsync_sys/
[ohos-libqos-sys]: https://docs.rs/ohos-libqos-sys/latest/ohos_libqos_sys/
[udmf-sys]: https://docs.rs/udmf-sys/latest/udmf_sys/
[ohos-sensors-sys]: https://docs.rs/ohos-sensors-sys/latest/ohos_sensors_sys/
[xcomponent-sys]: https://docs.rs/xcomponent-sys/latest/xcomponent_sys/
[ohos-asset-store-sys]: https://docs.rs/ohos-asset-store-sys/latest/ohos_asset_store_sys/
[ohos-basic-services-kit-sys]: https://docs.rs/ohos-basic-services-kit-sys/latest/ohos_basic_services_kit_sys/
[ohos-crypto-sys]: https://docs.rs/ohos-crypto-sys/latest/ohos_crypto_sys/
[ohos-huks-sys]: https://docs.rs/ohos-huks-sys/latest/ohos_huks_sys/
[ohos-accesstoken-sys]: https://docs.rs/ohos-accesstoken-sys/latest/ohos_accesstoken_sys/
[ohos-deviceinfo-sys]: https://docs.rs/ohos-deviceinfo-sys/latest/ohos_deviceinfo_sys/


## Development

The current bindings are generated with `bindgen` using `scripts/generate_bindings.sh`.
Bindings are currently generated with the OpenHarmony SDK API level 23, but with items
added after API level 10 feature guarded behind `api-XX` features. This is handled
automatically by the tool based on the documentation comments in the C header files.

# Contributing

There are still quite a few OpenHarmony APIs missing. Feel free to contribute missing APIs, but be sure to adapt
the script, so your bindings are reproducible!
The OpenHarmony SDK can be downloaded from the release notes of the respective release, e.g. the
[5.0.0 release notes](https://docs.openharmony.cn/pages/v5.0/en/release-notes/OpenHarmony-v5.0.0-release.md).
Navigate to the `Acquiring Source Code from Mirrors` section, select the `Public SDK package for the standard system`
for your host Operating System (Windows / Linux / Mac) click download and optionally verify the SHA-256 checksum
of the downloaded archive.
Extract the archive to a suitable location. Please note that the <os_name> subfolder contains more archives.
For the purpose of generating the bindings extracting the `native` archive is sufficient.

Once you have setup your local SDK, you should set the environment variable `OHOS_SDK_NATIVE` to
`/path/to/ohos-sdk/<your_host_os>/native`. Afterwards you can run the script to generate the bindings
and adapt it to incorporate new modules.

Please also check the following:

- Preferably generate the bindings with libclang in `C` mode. However, if a header file is not C-compliant
  due to an issue of the OpenHarmony SDK, then setting `libclang` to C++ mode is fine.
- Be sure to guard the new component behind a cargo feature and document the feature in Cargo.toml.


## License

This crate is licensed under the Apache-2.0 license, matching the OpenHarmony OS SDK.

[OpenHarmony OS]: https://docs.openharmony.cn/pages/v5.0/en/OpenHarmony-Overview.md
[documentation]: https://docs.rs/ohos-sys/latest/ohos_sys/
