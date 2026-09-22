# Memory measurement

Decode benchmarks measure time, not memory. CI containers are memory
constrained, so peak memory usage on large inputs is measured explicitly and
reported per input size.

## Running the measurement

```bash
cargo run --release -p devkit-xdr --example mem_profile
```

Pass comma-separated input sizes in bytes to override the defaults
(`1024,65536,1048576`):

```bash
cargo run --release -p devkit-xdr --example mem_profile -- 1024,262144,1048576
```

The example prints CSV:

```text
size_bytes,base64_bytes,decoded_json_bytes,current_bytes,peak_bytes,delta_current_bytes
1024,1376,2060,5595136,5595136,290816
262144,349536,524300,6590464,6848512,0
```

| Column | Meaning |
| ------ | ------- |
| `size_bytes` | Size of the `ScVal::Bytes` payload used to build the input. |
| `base64_bytes` | Length of the base64 XDR input that is decoded. |
| `decoded_json_bytes` | Length of the decoded JSON, proving the decode ran. |
| `current_bytes` | Resident memory after decoding (or `n/a`). |
| `peak_bytes` | Peak resident memory observed by the process (or `n/a`). |
| `delta_current_bytes` | Current-memory growth across the decode (or `n/a`). |

## Methodology

1. Build a `ScVal::Bytes` payload of the requested size and encode it to base64
   XDR.
2. Sample memory before the call.
3. Decode with `devkit_xdr::inspect_scval_base64`.
4. Sample memory after the call and serialise the result to JSON.

Each size is measured in the same process. Peak memory is monotonic, so later
rows report the running maximum; use the current-memory column, or a fresh
process, for isolated per-input figures.

## Platform APIs and fallback

| Platform | Current | Peak | Source |
| -------- | ------- | ---- | ------ |
| Linux | `VmRSS` | `VmHWM` | `/proc/self/status` |
| Windows | `WorkingSetSize` | `PeakWorkingSetSize` | `GetProcessMemoryInfo` |
| Other | `n/a` | `n/a` | documented fallback |

On unsupported platforms the example still builds and runs and prints a note to
stderr; memory columns are reported as `n/a`.

## Limits

- Figures are process-wide: they include the allocator, the toolchain runtime,
  and the decoded JSON value, not just XDR buffers.
- Decoding to JSON expands bytes substantially (each byte becomes a decimal
  array element), so JSON size dominates the large-input measurements.
- Numbers are representative, not exact: run in `--release` and compare runs on
  the same platform to detect regressions.
- Measurement is not cross-platform normalised; compare Linux to Linux.
