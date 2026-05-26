# OpenHarmony QoS bindings

Low-level bindings to the QoS (Quality of Service) thread scheduling APIs on
OpenHarmony, exposed by `libqos.so`. Lets a thread declare its scheduling
priority to the kernel via one of six `QoS_Level` classes (background, utility,
default, user-initiated, deadline-request, user-interactive).

Note: the same `libqos.so` ships a second, unrelated API surface starting at
API level 20 — the **Gewu** on-device LLM inference engine
(`OH_QoS_GewuCreateSession`, `OH_QoS_GewuSubmitRequest`, …). It is called
"QoS-aware" only in the sense that its internal worker pool cooperates with
the QoS scheduler; the API itself takes no `QoS_Level` and runs on threads
owned by the system service. To influence inference priority indirectly, set
your own foreground threads to a higher QoS so the scheduler preempts
inference when your UI runs.

C API reference:

- [Thread QoS guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/napi/qos-guidelines.md)
- [Gewu guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/napi/gewu-ndk-api-guidelines.md)
- [`qos.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-kernel-enhance-kit/capi-qos-h.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
