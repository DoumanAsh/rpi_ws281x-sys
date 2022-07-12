#[cfg(target_os = "linux")]
#[test]
fn should_detect_rpi_version() {
    use rpi_ws281x_sys::rpi_hw_detect;

    let rpi = unsafe {
        rpi_hw_detect()
    };

    //On non-RPI systems returns NULL
    assert!(rpi.is_null());
}
