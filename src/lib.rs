//! Low-level FFI bindings to AMD's rocJPEG library (ROCm), version 1.4.
//!
//! rocJPEG decodes JPEG images using the hardware-accelerated JPEG decoder
//! (VCN) on AMD GPUs. This crate exposes the raw C API of `librocjpeg`
//! one-to-one; see the [rocJPEG documentation](https://rocm.docs.amd.com/projects/rocJPEG/)
//! for semantics of each function.
//!
//! Decoded image buffers (`RocJpegImage`) are HIP device memory: allocate them
//! with `hipMalloc` (or a HIP runtime crate) and copy results back with
//! `hipMemcpy`.
//!
//! # Example
//!
//! ```no_run
//! use rocjpeg_rs_sys::*;
//! use std::ffi::c_int;
//!
//! unsafe {
//!     let mut stream: RocJpegStreamHandle = std::ptr::null_mut();
//!     assert_eq!(rocJpegStreamCreate(&mut stream), ROCJPEG_STATUS_SUCCESS);
//!
//!     let jpeg: &[u8] = &[];
//!     assert_eq!(
//!         rocJpegStreamParse(jpeg.as_ptr(), jpeg.len(), stream),
//!         ROCJPEG_STATUS_SUCCESS
//!     );
//!
//!     let mut num_components: u8 = 0;
//!     let mut subsampling: RocJpegChromaSubsampling = 0;
//!     let mut widths = [0u32; ROCJPEG_MAX_COMPONENT];
//!     let mut heights = [0u32; ROCJPEG_MAX_COMPONENT];
//!
//!     let handle: RocJpegHandle = std::ptr::null_mut();
//!     rocJpegGetImageInfo(
//!         handle,
//!         stream,
//!         &mut num_components,
//!         &mut subsampling,
//!         widths.as_mut_ptr(),
//!         heights.as_mut_ptr(),
//!     );
//!
//!     rocJpegStreamDestroy(stream);
//! }
//! ```

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int, c_void};

/// Maximum number of channels rocJPEG supports.
pub const ROCJPEG_MAX_COMPONENT: usize = 4;

/// Major version of the rocJPEG headers these bindings were generated from.
pub const ROCJPEG_VERSION_MAJOR: u32 = 1;
/// Minor version of the rocJPEG headers these bindings were generated from.
pub const ROCJPEG_VERSION_MINOR: u32 = 4;
/// Patch version of the rocJPEG headers these bindings were generated from.
pub const ROCJPEG_VERSION_PATCH: u32 = 0;

/// Status codes returned by rocJPEG functions. See [`rocJpegGetErrorName`].
pub type RocJpegStatus = c_int;

/// The operation completed successfully.
pub const ROCJPEG_STATUS_SUCCESS: RocJpegStatus = 0;
/// The rocJPEG library is not initialized.
pub const ROCJPEG_STATUS_NOT_INITIALIZED: RocJpegStatus = -1;
/// An invalid parameter was passed to a function.
pub const ROCJPEG_STATUS_INVALID_PARAMETER: RocJpegStatus = -2;
/// The input JPEG data is corrupted or invalid.
pub const ROCJPEG_STATUS_BAD_JPEG: RocJpegStatus = -3;
/// The JPEG format is not supported.
pub const ROCJPEG_STATUS_JPEG_NOT_SUPPORTED: RocJpegStatus = -4;
/// Out of memory error.
pub const ROCJPEG_STATUS_OUTOF_MEMORY: RocJpegStatus = -5;
/// The execution of a function failed.
pub const ROCJPEG_STATUS_EXECUTION_FAILED: RocJpegStatus = -6;
/// The architecture is not supported.
pub const ROCJPEG_STATUS_ARCH_MISMATCH: RocJpegStatus = -7;
/// Internal error occurred.
pub const ROCJPEG_STATUS_INTERNAL_ERROR: RocJpegStatus = -8;
/// The requested implementation is not supported.
pub const ROCJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED: RocJpegStatus = -9;
/// Hardware JPEG decoder is not supported.
pub const ROCJPEG_STATUS_HW_JPEG_DECODER_NOT_SUPPORTED: RocJpegStatus = -10;
/// Runtime error occurred.
pub const ROCJPEG_STATUS_RUNTIME_ERROR: RocJpegStatus = -11;
/// The requested feature is not implemented.
pub const ROCJPEG_STATUS_NOT_IMPLEMENTED: RocJpegStatus = -12;
/// Maximum status code value.
pub const ROCJPEG_STATUS_MAX_VALUE: RocJpegStatus = -13;

/// Chroma subsampling of a JPEG image. See [`rocJpegGetImageInfo`].
pub type RocJpegChromaSubsampling = c_int;

/// Full chroma resolution (4:4:4).
pub const ROCJPEG_CSS_444: RocJpegChromaSubsampling = 0;
/// Chroma resolution reduced by half vertically (4:4:0).
pub const ROCJPEG_CSS_440: RocJpegChromaSubsampling = 1;
/// Chroma resolution reduced by half horizontally (4:2:2).
pub const ROCJPEG_CSS_422: RocJpegChromaSubsampling = 2;
/// Chroma resolution reduced by half both horizontally and vertically (4:2:0).
pub const ROCJPEG_CSS_420: RocJpegChromaSubsampling = 3;
/// Chroma resolution reduced by a quarter horizontally (4:1:1).
pub const ROCJPEG_CSS_411: RocJpegChromaSubsampling = 4;
/// No chroma information (4:0:0).
pub const ROCJPEG_CSS_400: RocJpegChromaSubsampling = 5;
/// Unknown chroma subsampling.
pub const ROCJPEG_CSS_UNKNOWN: RocJpegChromaSubsampling = -1;

/// Output format for decoded images. See [`RocJpegDecodeParams`].
pub type RocJpegOutputFormat = c_int;

/// Return native unchanged decoded YUV image from the VCN JPEG decoder:
/// - for [`ROCJPEG_CSS_444`] and [`ROCJPEG_CSS_440`] write Y, U, and V to first,
///   second, and third channel of [`RocJpegImage`]
/// - for [`ROCJPEG_CSS_422`] write YUYV (packed) to first channel
/// - for [`ROCJPEG_CSS_420`] write Y to first channel and UV (interleaved) to
///   second channel
/// - for [`ROCJPEG_CSS_400`] write Y to first channel
pub const ROCJPEG_OUTPUT_NATIVE: RocJpegOutputFormat = 0;
/// Extract Y, U, and V channels from the decoded YUV image and write into
/// first, second, and third channel of [`RocJpegImage`]. For
/// [`ROCJPEG_CSS_400`] only Y is written to the first channel.
pub const ROCJPEG_OUTPUT_YUV_PLANAR: RocJpegOutputFormat = 1;
/// Return luma component (Y) and write to first channel of [`RocJpegImage`].
pub const ROCJPEG_OUTPUT_Y: RocJpegOutputFormat = 2;
/// Convert to interleaved RGB using the VCN JPEG decoder (on MI300+) or HIP
/// kernels and write to first channel of [`RocJpegImage`].
pub const ROCJPEG_OUTPUT_RGB: RocJpegOutputFormat = 3;
/// Convert to RGB planar using the VCN JPEG decoder (on MI300+) or HIP kernels
/// and write to first, second, and third channel of [`RocJpegImage`].
pub const ROCJPEG_OUTPUT_RGB_PLANAR: RocJpegOutputFormat = 4;
/// Maximum allowed value for the output format.
pub const ROCJPEG_OUTPUT_FORMAT_MAX: RocJpegOutputFormat = 5;

/// Backend used for JPEG decoding. See [`rocJpegCreate`].
pub type RocJpegBackend = c_int;

/// Hardware backend option.
pub const ROCJPEG_BACKEND_HARDWARE: RocJpegBackend = 0;
/// Hybrid backend option.
pub const ROCJPEG_BACKEND_HYBRID: RocJpegBackend = 1;

/// A handle representing a `RocJpegStream` instance, holding parsed JPEG
/// stream parameters.
pub type RocJpegStreamHandle = *mut c_void;

/// A handle representing a rocJPEG decoder instance.
pub type RocJpegHandle = *mut c_void;

/// Pointers to decoded image channels and the pitch (stride) of each channel.
///
/// All channel pointers refer to HIP device memory.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RocJpegImage {
    /// Pointers to the image channels.
    pub channel: [*mut u8; ROCJPEG_MAX_COMPONENT],
    /// Pitch (stride) of each channel.
    pub pitch: [u32; ROCJPEG_MAX_COMPONENT],
}

/// Region of interest copied into the output buffers.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RocJpegCropRectangle {
    /// Left coordinate of the crop rectangle.
    pub left: i16,
    /// Top coordinate of the crop rectangle.
    pub top: i16,
    /// Right coordinate of the crop rectangle.
    pub right: i16,
    /// Bottom coordinate of the crop rectangle.
    pub bottom: i16,
}

/// Target dimensions for resizing ((future use), both should be even).
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RocJpegTargetDimension {
    /// Target width of the picture to be resized.
    pub width: u32,
    /// Target height of the picture to be resized.
    pub height: u32,
}

/// Parameters for decoding a JPEG image.
///
/// If both the crop rectangle and target dimensions are defined, cropping is
/// done first, followed by resizing the resulting ROI to the target dimension.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RocJpegDecodeParams {
    /// Output data format. See [`RocJpegOutputFormat`] constants.
    pub output_format: RocJpegOutputFormat,
    /// Region of interest (ROI) copied into the [`RocJpegImage`] output buffers.
    pub crop_rectangle: RocJpegCropRectangle,
    /// (future use) Target width and height the picture is resized to.
    /// If specified, allocate the [`RocJpegImage`] buffers based on these
    /// dimensions.
    pub target_dimension: RocJpegTargetDimension,
}

extern "C" {
    /// Creates a [`RocJpegStreamHandle`] for JPEG stream processing.
    ///
    /// Returns [`ROCJPEG_STATUS_INVALID_PARAMETER`] if `jpeg_stream_handle` is
    /// null, [`ROCJPEG_STATUS_OUTOF_MEMORY`] if allocation fails.
    pub fn rocJpegStreamCreate(jpeg_stream_handle: *mut RocJpegStreamHandle) -> RocJpegStatus;

    /// Parses a JPEG stream of `length` bytes at `data` and stores the parsed
    /// parameters in `jpeg_stream_handle`.
    pub fn rocJpegStreamParse(
        data: *const u8,
        length: usize,
        jpeg_stream_handle: RocJpegStreamHandle,
    ) -> RocJpegStatus;

    /// Destroys a [`RocJpegStreamHandle`] and releases associated resources.
    pub fn rocJpegStreamDestroy(jpeg_stream_handle: RocJpegStreamHandle) -> RocJpegStatus;

    /// Creates a [`RocJpegHandle`] for JPEG decoding using the specified
    /// backend and device ID.
    ///
    /// Returns [`ROCJPEG_STATUS_INVALID_PARAMETER`] if `handle` is null,
    /// [`ROCJPEG_STATUS_NOT_INITIALIZED`] if decoder initialization fails.
    pub fn rocJpegCreate(
        backend: RocJpegBackend,
        device_id: c_int,
        handle: *mut RocJpegHandle,
    ) -> RocJpegStatus;

    /// Destroys a [`RocJpegHandle`] and releases associated resources.
    pub fn rocJpegDestroy(handle: RocJpegHandle) -> RocJpegStatus;

    /// Retrieves the number of components, chroma subsampling, and per-component
    /// dimensions of the JPEG image represented by `jpeg_stream_handle`.
    ///
    /// `widths` and `heights` must each point to an array of at least
    /// [`ROCJPEG_MAX_COMPONENT`] elements.
    pub fn rocJpegGetImageInfo(
        handle: RocJpegHandle,
        jpeg_stream_handle: RocJpegStreamHandle,
        num_components: *mut u8,
        subsampling: *mut RocJpegChromaSubsampling,
        widths: *mut u32,
        heights: *mut u32,
    ) -> RocJpegStatus;

    /// Decodes a JPEG image into the HIP device buffers described by
    /// `destination`, using the parameters in `decode_params`.
    pub fn rocJpegDecode(
        handle: RocJpegHandle,
        jpeg_stream_handle: RocJpegStreamHandle,
        decode_params: *const RocJpegDecodeParams,
        destination: *mut RocJpegImage,
    ) -> RocJpegStatus;

    /// Decodes a batch of `batch_size` JPEG images into the `destinations`
    /// array of HIP device buffers.
    pub fn rocJpegDecodeBatched(
        handle: RocJpegHandle,
        jpeg_stream_handles: *mut RocJpegStreamHandle,
        batch_size: c_int,
        decode_params: *const RocJpegDecodeParams,
        destinations: *mut RocJpegImage,
    ) -> RocJpegStatus;

    /// Returns the name of the error associated with the given
    /// [`RocJpegStatus`].
    pub fn rocJpegGetErrorName(rocjpeg_status: RocJpegStatus) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn struct_layouts_match_c_abi() {
        assert_eq!(std::mem::size_of::<RocJpegImage>(), 48);
        assert_eq!(std::mem::align_of::<RocJpegImage>(), 8);
        assert_eq!(std::mem::size_of::<RocJpegCropRectangle>(), 8);
        assert_eq!(std::mem::size_of::<RocJpegTargetDimension>(), 8);
        assert_eq!(std::mem::size_of::<RocJpegDecodeParams>(), 20);
        assert_eq!(std::mem::align_of::<RocJpegDecodeParams>(), 4);
    }

    #[test]
    fn error_name_reports_success() {
        unsafe {
            let name = CStr::from_ptr(rocJpegGetErrorName(ROCJPEG_STATUS_SUCCESS));
            assert_eq!(name.to_str().unwrap(), "ROCJPEG_STATUS_SUCCESS");
        }
    }
}
