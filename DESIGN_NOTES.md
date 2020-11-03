Design notes for CoDi-rs
=========================

## Implementation notes

- Ground-up implementation, Tock, or Drone? Latter two depend on
  existing and stable support for (as of yet unknown) microcontroller.

## Ideas

- Publish/subscribe model

    Power users can select which notifications they want CoDi to
    display, or select_all_ notifications, and exclude some.

- Serial baud rate to be increased

    Currently, my suspicion about phone OS<-> CoDi communication is
    that the baud rate is limited. My presumption is that this is to
    ensure integrity. I would like to be certain that it _is_ the baud
    rate. Given the slowness of uploading new firmware, and
    camera/video lag, that would tie in. Whenever this is the case,
    remains to be seen.

    To ensure data integrity, my suggestion is to use simple checksums
    to ensure that communication is intact. I would personally only
    apply this to firmware updates, along with negotiating a slightly
    faster than current baud rate, to enhance speed and user
    satisfaction, whilst also maintaining integrity.

- Rust-based RTOS

    Using Rust for the new RTOS gives us the benefits of memory-safety
    and static type enforcement, amongst the other features of Rust,
    whilst also maintaining optimisation and security.
