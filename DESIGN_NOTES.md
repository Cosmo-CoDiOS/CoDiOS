Design notes for CoDiOS
=======================

## Implementation notes

### Important firmware notes

*This firmware's design has been changed!*

Given stock CoDi's shortcomings, and the new approach CoDiOS will use,
drop-in compatability with the stock Android OS on the Cosmo
Communicator will no longer available.

CoDiOS will be using MsgPack bidirectionally with the MTK processor
and ST32 chip.

All communication will still be performed over serial.

The bootloader of the CoDi processor will be modified to accept both stock CoDi
firmware (traditional flashing), and CoDiOS. A special header will be sent
before the updates, which advises the bootloader on the type of firmware being
sent over serial. CRC checks will be stringent, and successful flashing will be
verified by the bootloader once its complete, and sent back to the flashing
tool.

### Microcontroller

The chip used for the cover display is a STM32 microprocessor, specifically, a
STM32L4R9AII6.

The [stm32-rs][] Rust crate has support for the aforementioned chip.

This chip runs at 120Mhz clock speed, has 2Mbyte flash, and 640Kbyte RAM, plus a
single-precision FPU.

The chip also presents 4MB external RAM (awaiting information if in use), and
32MB external flash (used for resources; images, animations, translations -
CoDiOS might be using sprites)

In terms of communication between the CoDiOS chip (referred to as `ST32`
henceforth), and the Cosmo OS SoC (referred to as `MTK`, henceforth), Planet
originally intended for it to be done over a faster data line, but this was not
possible. Instead, communications are done over UART.

### OS

In terms of the design for CoDiOS, the intention is for a minimal, fast, and
efficient RTOS (real-time operating system).

Primarily, the OS will be written in Rust. I anticipate some partial usage of C
or Assembly. which must remain safe, efficient, and secure.

Primary goals of the OS:

- Speed.
- Secure.
- Fault-tolerant.
- Software-enabled debugging.
- Small memory footprint.

I have also attained from Planet, details of the memory regions used on the CoDi
chip and external memory. They are unfortunately formatted in IAR format, and I
haven't yet figured out to translate them to GNU Linker format for the Rust
compiler to use (it does use LLVM behind the scenes, though). I have not
included them in this repo until I have permission to do so, if at all.

#### Recovery

Currently, it would appear the stock CoDi, when it encounters a non-recoverable
error just simply restarts. There is also *no way* to debug CoDi without
soldering a data line directly onto the chip. Of course, this is a sub-optimal
method, and should be enhanced.

Therefore, it is my aim to incorporate the following enhancements to CoDiOS:

- Logging.
    - Logging should be initialised as soon as possible, and stored in a buffer.
    - The buffer should be 'flushed' to the OS on the MTK chip as soon as a log comes in.
- Recovery
    - The recovery process should include, in execution order:
        - Shutting down OS to minimal state, preparing for memory dump and
          recovery message.
        - Display warning icon, state current stage of recovery - then display
          recovery icon.
        - Dump logs to MTK chip over serial - msgpack array.
        - Dump memory to MTK over serial - split packets, with special header
          signifying memory dump.
        - Display recovery icon, and restart after waiting for 15 seconds, or
          manual restart by pressing the silver button.

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
