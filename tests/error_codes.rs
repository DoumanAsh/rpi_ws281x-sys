#[cfg(target_os = "linux")]
#[test]
fn should_get_error_code_description() {
    use rpi_ws281x_sys::ws2811_return_t;

    let errors = [
        (ws2811_return_t::WS2811_SUCCESS, "Success"),
        (ws2811_return_t::WS2811_ERROR_GENERIC, "Generic failure"),
        (ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY, "Out of memory"),
        (ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED, "Hardware revision is not supported"),
        (ws2811_return_t::WS2811_ERROR_MEM_LOCK, "Memory lock failed"),
        (ws2811_return_t::WS2811_ERROR_MMAP, "mmap() failed"),
        (ws2811_return_t::WS2811_ERROR_GPIO_INIT, "Unable to initialize GPIO"),
        (ws2811_return_t::WS2811_ERROR_PWM_SETUP, "Unable to initialize PWM"),
        (ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE, "Failed to create mailbox device"),
        (ws2811_return_t::WS2811_ERROR_DMA, "DMA error"),
        (ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO, "Selected GPIO not possible"),
        (ws2811_return_t::WS2811_ERROR_PCM_SETUP, "Unable to initialize PCM"),
        (ws2811_return_t::WS2811_ERROR_SPI_SETUP, "Unable to initialize SPI"),
        (ws2811_return_t::WS2811_ERROR_SPI_TRANSFER, "SPI transfer error"),
    ];

    for error in errors {
        println!("{:?}={}", error.0, error.1);
        assert_eq!(error.0.as_str(), error.1);
    }
}
