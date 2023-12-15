# C Example: dhrystone

`make`

The popular MCU benchmarking tool Dhrystone, easily running on this core and its variants. Defaults to 20 million iterations. Uses `-fno-inline` to match `vexriscv` testing methodology.

Prints:
```
Microseconds for one run through Dhrystone: 18
Dhrystones per Second:                      54945
```

You can calculate the DMIPS/s by dividing by 1757:

```
54945 / 1757 = 31.272 DMIPS/s
31.272 / 57.12MHz = 0.547 DMIPS/MHz
```

For additional information, see [Dhrystone Howto](https://wiki.cdot.senecacollege.ca/wiki/Dhrystone_howto) and [SiFive Performance Tuning](https://www.sifive.com/blog/dhrystone-performance-tuning-on-the-freedom-platform).

## License

As stated in [`LICENSE`](LICENSE), there is no explicit license defined. Dhrysone was originally written in ADA by Reinhold P. Weicker and translated to C by Rick Richardson.

The source obtained from the following site: https://fossies.org/linux/privat/old/dhrystone-2.1.tar.gz

This code is likely considered public domain at this point.