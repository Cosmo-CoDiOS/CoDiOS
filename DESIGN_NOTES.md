Design notes for CoDirs
==============================================================

## Implementation notes

### Important firmware notes

This firmware is designed to be a drop-in replacement. It should work with the
existing protocol, and not need any modifications on the Android OS, or other
OSes. Previous iterations of the design did not aim for a drop-in replacement,
but this has now been changed.

### Microcontroller

The chip used for the cover display is: STM32L4R9AII6.

This chip runs at 120Mhz clock speed, has 2Mbyte flash, and 640Kbyte
RAM. It has a FPU.

The cover display also has:

- 4MB external RAM
- 32MB external flash.

We must optimise as _much as possible_. A balance is required between
size optimisations, and speed. A lot of this is dependent on the code
itself.

### OS

Ideally I'd like to do a microkernel RTOS. There's a slight performance loss
with microkernels, due to context switching, and considering the clock speed of
the STM32, a monolithic RTOS may well be more suitable.

However, microkernels offer suitable performance, and good security. It may also
be that using a microkernel is more power-efficient as well.

Language for the RTOS should be Rust. C or Assembly may be required for certain
low-level components.

Taking the above into consideration, a microkernel would suit this firmware
well.

In terms of mapping the code into the memory regions that the cover display
uses, Planet have very kindly provided me (@shymega) with details of the regions.

I am not sure if I can share this on the repo yet, but I will query that.

## Ideas

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

    No other data channels exist for Mediatek <-> STM32 communications.

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
