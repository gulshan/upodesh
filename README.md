# upodesh
`upodesh` (In Bangla, `উপদেশ` is a synonym of the word `পরামর্শ` which means `suggestion`) is a Bangla word suggestion library.

This implementation uses an approach based on the Trie data structure which is substantially faster than the Regular Expression based approach. This is a Rust port of the Go project [`libavrophonetic`](https://github.com/mugli/libavrophonetic/) of Mehdi Hasan Khan.

## Benchmarks
`upodesh` is around **~5× faster** than a heavily optimized regex based search approach previously used in OpenBangla Keyboard. And in cases where the old implementation struggled with large regex patterns, upodesh is a staggering **~80×** faster! 

### 📊 Summary of the Benchmark
This benchmark was performed on a Apple MacBook Air M1:

| Word   | `upodesh` Time | `regex` Time | Speedup         |
| --------- | -------------- | ------------ | --------------- |
| `a`       | 39.190 µs      | 193.50 µs    | **\~4.94× faster**  |
| `arO`     | 45.942 µs      | 247.68 µs    | **\~5.39× faster**  |
| `bistari` | 4.4495 µs      | 355.04 µs    | **\~79.79× faster** |



## Acknowledgement
* [Mehdi Hasan Khan](https://github.com/mugli) and [Tahmid Sadik](https://github.com/tahmidsadik/) for their [`libavrophonetic`](https://github.com/mugli/libavrophonetic/) project.
