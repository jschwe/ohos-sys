use ohos_qos_sys as qos;

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_thread_qos() {
    unsafe {
        let _ = qos::OH_QoS_ResetThreadQoS();
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_gewu() {
    use core::ptr;
    unsafe {
        let _ = qos::OH_QoS_GewuCreateSession(ptr::null());
    }
}
