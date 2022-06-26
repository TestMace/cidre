use std::intrinsics::transmute;

use ns::Id;

use crate::{at::audio::StreamBasicDescription, cf, define_obj_type, ns};

use super::{channel_layout::ChannelLayout, ChannelCount};

#[repr(usize)]
pub enum CommonFormat {
    /// A format other than one of the common ones below.
    Other = 0,

    /// Native-endian floats (this is the standard format).
    PCMFloat32 = 1,
    /// Native-endian doubles.
    PCMFloat64 = 2,
    /// Signed 16-bit native-endian integers.
    PCMInt16 = 3,
    /// Signed 32-bit native-endian integers.
    PCMInt32 = 4,
}

define_obj_type!(Format(ns::Id));

/// AVAudioFormat wraps a Core Audio AudioStreamBasicDescription struct, with convenience
/// initializers and accessors for common formats, including Core Audio's standard deinterleaved
/// 32-bit floating point.
///
/// Instances of this class are immutable.
impl Format {
    /// If the format specifies more than 2 channels, this method fails (returns None).
    pub fn with_asbd<'a>(asbd: &StreamBasicDescription) -> Option<cf::Retained<'a, Format>> {
        unsafe { AVAudioFormat_initWithStreamDescription(asbd) }
    }

    /// the channel layout. Can be None only if asbd specifies 1 or 2 channels.
    pub fn with_asbd_and_channel_layout<'a>(
        asbd: &StreamBasicDescription,
        layout: Option<&ChannelLayout>,
    ) -> Option<cf::Retained<'a, Format>> {
        unsafe { AVAudioFormat_initWithStreamDescription_channelLayout(asbd, layout) }
    }

    pub fn standard_with_sample_rate_and_channels<'a>(
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<cf::Retained<'a, Format>> {
        unsafe { AVAudioFormat_initStandardFormatWithSampleRate_channels(sample_rate, channels) }
    }

    /// Initialize to deinterleaved float with the specified sample rate and channel layout.
    pub fn standard_with_sample_rate_and_channel_layout<'a>(
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> cf::Retained<'a, Format> {
        unsafe { AVAudioFormat_initStandardFormatWithSampleRate_channelLayout(sample_rate, layout) }
    }

    /// Initialize to float with the specified sample rate, channel layout and interleavedness.
    pub fn with_common_format_sample_rate_interleaved_channel_layout<'a>(
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> cf::Retained<'a, Format> {
        unsafe {
            AVAudioFormat_initWithCommonFormat_sampleRate_interleaved_channelLayout(
                format,
                sample_rate,
                interleaved,
                channel_layout,
            )
        }
    }

    pub fn with_settings<'a>(settings: &cf::Dictionary) -> Option<cf::Retained<'a, Format>> {
        unsafe { AVAudioFormat_initWithSettings(settings) }
    }

    pub fn settings(&self) -> &cf::DictionaryOf<cf::String, ns::Id> {
        unsafe {
            transmute(rsel_settings(self))
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {

    fn AVAudioFormat_initWithStreamDescription<'a>(
        asbd: &StreamBasicDescription,
    ) -> Option<cf::Retained<'a, Format>>;

    fn AVAudioFormat_initWithStreamDescription_channelLayout<'a>(
        asbd: &StreamBasicDescription,
        layout: Option<&ChannelLayout>,
    ) -> Option<cf::Retained<'a, Format>>;

    fn AVAudioFormat_initStandardFormatWithSampleRate_channels<'a>(
        sample_rate: f64,
        channels: ChannelCount,
    ) -> Option<cf::Retained<'a, Format>>;

    fn AVAudioFormat_initStandardFormatWithSampleRate_channelLayout<'a>(
        sample_rate: f64,
        layout: &ChannelLayout,
    ) -> cf::Retained<'a, Format>;

    fn AVAudioFormat_initWithCommonFormat_sampleRate_interleaved_channelLayout<'a>(
        format: CommonFormat,
        sample_rate: f64,
        interleaved: bool,
        channel_layout: &ChannelLayout,
    ) -> cf::Retained<'a, Format>;

    fn AVAudioFormat_initWithSettings<'a>(
        settings: &cf::Dictionary,
    ) -> Option<cf::Retained<'a, Format>>;
    

    fn rsel_settings(id: &Id) -> &cf::Dictionary;
}
