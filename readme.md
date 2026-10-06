## hoshi - a smol fetch tool written in rust

![demo](image.png)

## Benchmark

Simple execution-time benchmark of the release binary (`./target/release/hoshi`) on performance mode.

- **Runs:** 50
- **Average:** 17.74 ms
- **Median:** 17.81 ms
- **Min:** 16.55 ms
- **Max:** 18.78 ms

### System Specs

- **OS:** NixOS 26.05 (Yarara)
- **Kernel:** 7.2.6
- **CPU:** AMD Ryzen 7 7735HS with Radeon Graphics
- **Cores/Threads:** 8 cores / 16 threads
- **Memory:** 16 GiB
- **CPU Governor:** performance

Benchmarked with:

```bash
for i in $(seq 1 50); do
  start=$EPOCHREALTIME
  ./target/release/hoshi >/dev/null 2>&1
  end=$EPOCHREALTIME
  awk -v s="$start" -v e="$end" 'BEGIN { printf "%.3f\n", (e-s)*1000 }'
done
```
