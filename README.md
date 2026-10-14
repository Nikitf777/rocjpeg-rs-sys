# rocjpeg-rs-sys

Low-level FFI bindings to [AMD rocJPEG](https://rocm.docs.amd.com/projects/rocJPEG/), the
hardware-accelerated JPEG decode library (VCN) on AMD GPUs. This is a `-sys` crate: it
exposes the C API of `librocjpeg` one-to-one with no safe wrappers.

Bindgen-free: the bindings are hand-written against `rocjpeg.h` (rocJPEG 1.4), so the crate
has no dependencies and needs no `libclang` at build time.

## Requirements

- ROCm with rocJPEG installed (`/opt/rocm/lib/librocjpeg.so`), or any location you point
  the build script to.
- A supported AMD GPU with a VCN JPEG decoder for actual decoding.

## Build configuration

| Environment variable | Meaning |
| --- | --- |
| `ROCJPEG_LIB_DIR` | Directory containing `librocjpeg.so` (highest priority) |
| `ROCM_PATH` | ROCm root; `<ROCM_PATH>/lib`, `<ROCM_PATH>/lib64`, `<ROCM_PATH>/bin` are searched (default: `/opt/rocm`) |

## Usage

```toml
[dependencies]
rocjpeg-rs-sys = "0.1"
```

The API is entirely `unsafe`. Destination buffers for [`rocJpegDecode`] are HIP device
memory — allocate them with `hipMalloc` (e.g. via a HIP runtime crate) and copy results
back with `hipMemcpy`.

```rust
use rocjpeg_rs_sys::*;
use std::ffi::CStr;

fn main() {
    unsafe {
        let status = ROCJPEG_STATUS_BAD_JPEG;
        let name = CStr::from_ptr(rocJpegGetErrorName(status));
        println!("{}", name.to_str().unwrap());
    }
}
```

See the crate docs for the full API and the [rocJPEG samples](https://github.com/ROCm/rocm-systems/tree/develop/projects/rocjpeg/samples)
for complete decode examples.

## License

MIT. See [LICENSE](LICENSE).
