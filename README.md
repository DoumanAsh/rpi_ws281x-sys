# rpi_ws281x-sys

![Rust](https://github.com/DoumanAsh/rpi_ws281x-sys/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/rpi_ws281x-sys.svg)](https://crates.io/crates/rpi_ws281x-sys)
[![Documentation](https://docs.rs/rpi_ws281x-sys/badge.svg)](https://docs.rs/crate/rpi_ws281x-sys/)

Bindings to [rpi_ws281x](https://github.com/jgarff/rpi_ws281x)

Sources from [1ba8e385708fb7802b09c0177a7ea4293948e25c](https://github.com/jgarff/rpi_ws281x/commit/1ba8e385708fb7802b09c0177a7ea4293948e25c)

### GPIO Usage:

The GPIOs that can be used are limited by the hardware of the Pi and will
vary based on the method used to drive them (PWM, PCM or SPI).
Beware that the GPIO numbers are not the same as the physical pin numbers
on the header.

PWM:
```
        PWM0, which can be set to use GPIOs 12, 18, 40, and 52.
        Only 12 (pin 32) and 18 (pin 12) are available on the B+/2B/3B

        PWM1 which can be set to use GPIOs 13, 19, 41, 45 and 53.
        Only 13 is available on the B+/2B/PiZero/3B, on pin 33
```

PCM:
```
        PCM_DOUT, which can be set to use GPIOs 21 and 31.
        Only 21 is available on the B+/2B/PiZero/3B, on pin 40.
```

SPI:
```
        SPI0-MOSI is available on GPIOs 10 and 38.
        Only GPIO 10 is available on all models.
        See also note for RPi 3 below.
```


### Power and voltage requirements

WS281X LEDs are generally driven at 5V. Depending on your actual
LED model and data line length you might be able to successfully drive
the data input with 3.3V. However in the general case you probably
want to use a level shifter to convert from the Raspberry Pi GPIO/PWM to 5V.

It is also possible to run the LEDs from a 3.3V - 3.6V power source, and
connect the GPIO directly at a cost of brightness, but this isn't
recommended.

Know what you're doing with the hardware and electricity. I take no responsibility for damage, harm, or mistakes.

### Important warning about DMA channels

You must make sure that the DMA channel you choose to use for the LEDs is not [already in use](https://www.raspberrypi.org/forums/viewtopic.php?p=609380#p609380) by the operating system.

For example, **using DMA channel 5 [will cause](https://github.com/jgarff/rpi_ws281x/issues/224) filesystem corruption** on the Raspberry Pi 3 Model B.

The default DMA channel (10) should be safe for the Raspberry Pi 3 Model B, but this may change in future software releases.

### Limitations:

#### PWM

Since this library and the onboard Raspberry Pi audio
both use the PWM, they cannot be used together.  You will need to
blacklist the Broadcom audio kernel module by creating a file
`/etc/modprobe.d/snd-blacklist.conf` with

    blacklist snd_bcm2835

If the audio device is still loading after blacklisting, you may also
need to comment it out in the /etc/modules file.

On headless systems you may also need to force audio through hdmi
Edit config.txt and add:

    hdmi_force_hotplug=1
    hdmi_force_edid_audio=1

A reboot is required for this change to take effect

Some distributions use audio by default, even if nothing is being played.
If audio is needed, you can use a USB audio device instead.

#### PCM

When using PCM you cannot use digital audio devices which use I2S since I2S
uses the PCM hardware, but you can use analog audio.

#### SPI

When using SPI the led string is the only device which can be connected to
the SPI bus. Both digital (I2S/PCM) and analog (PWM) audio can be used.

Many distributions have a maximum SPI transfer of 4096 bytes. This can be
changed in `/boot/cmdline.txt` by appending
```
    spidev.bufsiz=32768
```

On an RPi 3 you have to change the GPU core frequency to 250 MHz, otherwise
the SPI clock has the wrong frequency.

Do this by adding the following line to /boot/config.txt and reboot:

```
    core_freq=250
```

On an RPi 4 you must set a fixed frequency to avoid the idle CPU scaling changing the SPI frequency and breaking the ws281x timings:

Do this by adding the following lines to /boot/config.txt and reboot:

```
    core_freq=500
    core_freq_min=500
```

SPI requires you to be in the `gpio` group if you wish to control your LEDs without root.
