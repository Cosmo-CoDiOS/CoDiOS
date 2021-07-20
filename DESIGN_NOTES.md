Design notes for CoDirs
==============================================================

## Implementation notes

### Important firmware notes

- This firmware is designed to be a drop-in replacement. It should work with the
  existing protocol, and not need any modifications on the Android OS, or other
  OSes. Previous iterations of the design did not aim for a drop-in replacement,
  but this has now been changed.

### Microcontroller

The chip used for the cover display is: STM32L4R9AII6.

The crate - [stm32-rs][]- has support for the aforementioned chip.

This chip runs at 120Mhz clock speed, has 2Mbyte flash, and 640Kbyte
RAM. It has a FPU.

The cover display also has:

- 4MB external RAM (is this used?)
- 32MB external flash (currently used for CoDi resources)

### OS

In terms of the OS for CoDirs, the intention is for a microkernel-based RTOS
(real-time operating system).

Primarily, the OS will be written in Rust. I anticipate some partial usage of C
or Assembly.

Primarily goals of the OS:

- Speed.
- Secure.
- Fault-tolerant.
- Software-enabled debugging.
- Small memory footprint.

I have also attained from Planet, details of the memory regions used on the CoDi
chip and external memory.

#### Recovery

Currently, it would appear the stock CoDi, when it encounters a non-recoverable error just simply restarts. There is also *no way* to debug CoDi without soldering a data line directly onto the chip - or so I've been told.

Therefore, it is my aim to incoporate the following enhancements to CoDirs:

- Logging.
    Logging should be initialised as soon as possible, and stored in a buffer.
    The buffer should be 'flushed' to the OS on the MTK chip as soon as a log comes in.
- Recovery
    - The recovery process should include, in execution order:
        - Shutting down OS to minimal state, preparing for memory dump and recovery message.
        - Dump logs to MTK chip.
        - Dump memory to MTK
        - Display recovery icon, and await restart.

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

Of course, if the metadata were changed, it should be on a different URL,
because older versions of the Assistant/CoDi updater software (see: codi-app)
would break!

[stm32-rs]: https://github.com/stm32-rs/stm32-rs
