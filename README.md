# pihex
Arbitrary place hexadecimal digits viewer of pi written in Rust.
The library is based on the Bailey-Borwein-Plouffe formula (BBP formula).

```
 $ pihex
0: 243f 6a88 85a3 08d3 1319 8a2e 0370 7344
 $ pihex 1
1: 43f6 a888 5a30 8d31 3198 a2e0 3707 344a
 $ pihex 2
2: 3f6a 8885 a308 d313 198a 2e03 7073 44a4
 $ pihex 4
4: 6a88 85a3 08d3 1319 8a2e 0370 7344 a409
 $ pihex 8
8: 85a3 08d3 1319 8a2e 0370 7344 a409 3822
 $ pihex 128
128: 9216 d5d9 8979 fb1b d131 0ba6 98df b5ac
 $ pihex 65536
65536: 3004 3414 c926 7212 d7fb 8a3f fc7c 7002
 $ pihex 1000000
1000000: 6c65 e52c b459 3500 50e4 bb17 8f4c 67a0
```

## Author
itchyny (https://github.com/itchyny)

## License
This software is released under the MIT License, see LICENSE.

## References
- David H. Bailey, Peter Borwein, and Simon Plouffe, On the Rapid Computation of Various Polylogarithmic Constants, Mathematics of Computation 66, 903-913, 1997.
- David H. Bailey, The BBP Algorithm for Pi, September 17, 2006.