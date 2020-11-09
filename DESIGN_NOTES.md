Design notes for CoDirs
==============================================================

## Implementation notes

### Important firmware notes

The firmware is _not_ a drop-in replacement. The existing Android
OS/other OSes etc will need modification to work with the new features
of CoDirs

### Microcontroller

The chip used for the cover display is: STM32L4R9AII6.

This chip runs at 120Mhz clock speed, has 2Mbyte flash, and 640Kbyte
RAM. It also has a FPU.

We must optimise as _much as possible_. A balance is required between
size optimisations, and speed. A lot of this is dependent on the code
itself.

### OS

Ideally I'd like to do a microkernel RTOS. There's a slight
performance loss with microkernels, due to context switching, and
considering the clock speed of the STM32, a monolithic RTOS may well be more suitable.

However, microkernels offer the best security, and help to keep processes
isolated. Given the nature of the Cosmo Communicator - a mobile phone - it
_could_ be argued a microkernel is the ideal design.
Perhaps simplicity is what I'm after here.

Language for the OS should be Rust, perhaps with some C/Assembly if
necessary.

Rust provides fast performance (although, obviously this
would be dependent on the STM32 chip!), memory/thread safety, and
efficiency.

## Ideas

- Publish/subscribe model

    Users can select which notifications they want CoDi to
    display, or select all notifications, and opt-out of the ones they
    do not want to see. However, sane defaults should be provieded out of the
    box.

- Third-party API

    The Cover Display OS should provide a third-party API interface,
    for app developers and power users to directly interface with CoDi
    itself.

    We would need to settle on an efficient format for
    communication. This would need to be suited for `no_std` usage in
    the OS, and have a OS library for parsing the format.

    Additionally, we should create a library _for_ the API calls,
    providing a safe abstraction layer over the OS's third-party API
    interface.

    This would need to be created for the various OSes available for
    the Cosmo, including the Android OS.

    NOTE: The Android firmware/other OSes would have to use the new
    API provided by the new CoDi firmware. The existing hooks would
    not work with the new firmware.

- Serial baud rate to be increased?

    ~~Currently, my suspicion about phone OS<-> CoDi communication is
    that the baud rate is limited. My presumption is that this is to
    ensure integrity. I would like to be certain that it _is_ the baud
    rate. Given the slowness of uploading new firmware, and
    camera/video lag, that would tie in. Whenever this is the case,
    remains to be seen.~~

    ~~To ensure data integrity, my suggestion is to use simple checksums
    to ensure that communication is intact. I would personally only
    apply this to firmware updates, along with negotiating a slightly
    faster than current baud rate, to enhance speed and user
    satisfaction, whilst also maintaining integrity.~~

    Further information acquired states the current UART speed is
    1Mbit (set to 115200), and (apparently) cannot be increased.

    NOTE: Awaiting more details about the chip to ascertain if other data
    channels exist.

## Firmware delivery

Firmware delivery is currently done over HTTP. The metadata format for
the CoDi Assistant app to ascertain when a new version is out, along
with hashes, is insecure. The metadata can be easily intercepted and
redirected to malicious firmware. Same with the firmware images
themselves. This must be addressed, as it would be possible for a MITM
attack to be launched.

The metadata is also presented in a rather weird way. I would suggest
a JSON dictionary instead, and to define the format in a formal
specification. I note that the PC Wiki is over HTTP, and propose that
it should be HTTPS, again, to prevent MITM interceptions.
