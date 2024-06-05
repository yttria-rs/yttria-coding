# yttria-math

Yttria sets out to be an end-to-end wireless communications DSP library for developing custom and
novel radio solutions, particularly regarding Software Defined Radios. Yttria-math is the subcrate
of the Yttria framework responsible for the mathematical heavy lifting involved. Many of the
functions of Yttria-math are directly analogous to those found in Numpy or Scipy, since those
libraries are particularly well suited for the task.

One of the goals of this crate is to make DSP easy with as little extra consideration as possible.
The result is that no custom structs have been made for the purpose of DSP, instead simply offering
extension traits on any iterable that contains a number (so determined by the `num` crate), and is
`send + sync + copy + clone` so that a lot of the heavy lifting can be done using `rayon`.

## Why Yttria?

Yittrium Iron Garnets (YIGs) are a powerful technology that have been around for many years, but are
becoming very powerful and highly relevant in miniaturized SDR applications due to their good filter
characteristics and their extremely high agility. This project hopes to extend that power by making
the software side as powerful and agile as the hardware is becoming.

## Todo

- [x] Gray codes
- [ ] Gold codes
    - [ ] Linear Feedback Shift Registers
- [ ] Barker codes
- [ ] Error Correction Codes
    - [ ] Turbo codes
    - [ ] LPDC
    - [ ] Polar codes
