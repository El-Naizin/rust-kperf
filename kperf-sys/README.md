# Kperf-rs

Native rust bindings to the private macOS frameworks: kperf and kperfdata.
The library binds to the frameworks with the libc library.
This framework allows access to the PMU, and it's usage requires the process
to be run with super-user privileges.

If when building or running this code on a Mac, you have a linker error, it means either
the kperf or kperfdata private frameworks changed, or some functions were not well tested.

I could only test the code on an Apple M2 2022 macbook air, on macOS Ventura 13.5.2

Issues are welcome.

Still a WIP.

## Credit

The rust code was written from the reverse-engineering efforts of two posts I saw online:
- https://lemire.me/blog/2023/03/21/counting-cycles-and-instructions-on-arm-based-apple-systems/
- https://twitter.com/ibireme/status/1476802948160442368
