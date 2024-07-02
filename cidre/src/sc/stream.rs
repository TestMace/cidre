use crate::{
    api, arc, blocks, cf, cg, cm, cv, define_cls, define_obj_type, dispatch, ns, objc, sc,
};

/// Denotes the status of frame sample buffer.
#[doc(alias = "SCFrameStatus")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum FrameStatus {
    Complete,
    Idle,
    Blank,
    Suspended,
    Started,
    Stopped,
}

define_obj_type!(
    /// Keys you use to retrieve metadata from a frame the system captures.
    #[doc(alias = "SCStreamFrameInfo")]
    pub FrameInfo(ns::String)
);

impl FrameInfo {
    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] that denotes the frames [`sc::FrameStatus`]
    #[doc(alias = "SCStreamFrameInfoStatus")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn status() -> &'static Self {
        unsafe { SCStreamFrameInfoStatus }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the mach absolute
    /// time when the event occurred. For a frame event, this is when the frame was displayed by the window server.
    #[doc(alias = "SCStreamFrameInfoDisplayTime")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn display_time() -> &'static Self {
        unsafe { SCStreamFrameInfoDisplayTime }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the display resolution
    /// associated with the frame. Display resolution is the pixel to point scaling factor.
    /// It should be in the range of \[1, 4\].
    #[doc(alias = "SCStreamFrameInfoScaleFactor")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn scale_factor() -> &'static Self {
        unsafe { SCStreamFrameInfoScaleFactor }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the content scale
    /// associated with the frame. Content scale is the scaling factor from original content
    /// size to its size in surface.
    #[doc(alias = "SCStreamFrameInfoContentScale")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn content_scale() -> &'static Self {
        unsafe { SCStreamFrameInfoContentScale }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the content rect
    /// associated with the frame. Content rect is the size and location of content in points in surface.
    #[doc(alias = "SCStreamFrameInfoContentRect")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn content_rect() -> &'static Self {
        unsafe { SCStreamFrameInfoContentRect }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for an array of rectangles
    /// that is the union of both rectangles that were redrawn and rectangles that were moved.
    /// This is an array of [`cg::Rect`] in [`ns::Value`]. The [`cg::Rect`]s elements are specified in pixels.
    #[doc(alias = "SCStreamFrameInfoDirtyRects")]
    #[inline]
    #[api::available(macos = 12.3)]
    pub fn dirty_rects() -> &'static Self {
        unsafe { SCStreamFrameInfoDirtyRects }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the onscreen location
    /// of the captured content
    #[doc(alias = "SCStreamFrameInfoScreenRect")]
    #[inline]
    #[api::available(macos = 13.1)]
    pub fn screen_rect() -> &'static Self {
        unsafe { SCStreamFrameInfoScreenRect }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the bounding rect
    /// associated with the frame. Bounding rect is the size and location of smallest bounding box
    /// containing all captured windows in points and in surface coordinates.
    #[doc(alias = "SCStreamFrameInfoBoundingRect")]
    #[inline]
    #[api::available(macos = 14.0)]
    pub fn bounding_rect() -> &'static Self {
        unsafe { SCStreamFrameInfoBoundingRect }
    }

    /// The key for the [`cf::Dictionary`] attached to the [`cm::SampleBuf`] for the content rect associated
    /// with the frame while in presenter overlay.  In presenter overlay small, this content rect is the size
    /// and location of smallest bounding box containing all captured windows plus small overlay window
    /// in points and in surface coordinates. If presenter overlay large, this content rect is the size and
    /// location of shared content in points and in surface coordinates.
    #[doc(alias = "SCStreamFrameInfoPresenterOverlayContentRect")]
    #[inline]
    #[api::available(macos = 14.2)]
    pub fn presenter_overlay_content_rect() -> &'static Self {
        unsafe { SCStreamFrameInfoPresenterOverlayContentRect }
    }
}

#[doc(alias = "SCStreamOutputType")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum OutputType {
    #[doc(alias = "SCStreamOutputTypeScreen")]
    Screen,
    #[doc(alias = "SCStreamOutputTypeAudio")]
    Audio,
    #[doc(alias = "SCStreamOutputTypeMicrophone")]
    Mic,
}

/// Denotes the setting that can be set to determine when to show the presenter overlay
/// alert for any stream
#[doc(alias = "SCPresenterOverlayAlertSetting")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PresenterOverlayAlertSetting {
    /// Allow the system to determine when to show the presenter overlay privacy alert.
    System,

    /// Never show the presenter overlay privacy alert.
    Never,

    /// Always show the presenter overlay privacy alert.
    Always,
}

#[doc(alias = "SCCaptureResolutionType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CaptureResolution {
    Automatic,
    Best,
    Nominal,
}

define_obj_type!(
    #[doc(alias = "SCStreamConfiguration")]
    pub Cfg(ns::Id),
    SC_STREAM_CONFIGURATION
);

/// Client can use sc::StreamCfgPreset to create sc::StreamCfg with suggested values of properties for various use cases
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CfgPreset {
    CaptureHdrStreamLocalDisplay,
    CaptureHdrStreamCanoncalDisplay,
    CaptureHdrScreenshotLocalDisplay,
    CaptureHdrScreenshotCanoncalDisplay,
}

impl Cfg {
    /// ```
    /// use cidre::{sc, cm, cv};
    ///
    /// let mut cfg = sc::StreamCfg::new();
    ///
    /// cfg.set_width(200);
    /// assert_eq!(200, cfg.width());
    /// cfg.set_height(300);
    /// assert_eq!(300, cfg.height());
    ///
    /// cfg.set_minimum_frame_interval(cm::Time::new(1, 60));
    /// cfg.set_pixel_format(cv::PixelFormat::_32_BGRA);
    /// cfg.set_scales_to_fit(false);
    /// cfg.set_shows_cursor(false);
    ///
    /// ```
    #[objc::msg_send2(width)]
    pub fn width(&self) -> usize;

    #[objc::msg_send2(setWidth:)]
    pub fn set_width(&mut self, val: usize);

    #[objc::msg_send2(height)]
    pub fn height(&self) -> usize;

    #[objc::msg_send2(setHeight:)]
    pub fn set_height(&mut self, val: usize);

    #[objc::msg_send2(minimumFrameInterval)]
    pub fn minimum_frame_interval(&self) -> cm::Time;

    #[objc::msg_send2(setMinimumFrameInterval:)]
    pub fn set_minimum_frame_interval(&mut self, val: cm::Time);

    /// 'BGRA': Packed Little Endian ARGB8888
    /// 'l10r': Packed Little Endian ARGB2101010
    /// '420v': 2-plane "video" range YCbCr 4:2:0
    /// '420f': 2-plane "full" range YCbCr 4:2:0
    #[objc::msg_send2(pixelFormat)]
    pub fn pixel_format(&self) -> cv::PixelFormat;

    #[objc::msg_send2(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: cv::PixelFormat);

    #[objc::msg_send2(scalesToFit)]
    pub fn scales_to_fit(&self) -> bool;

    #[objc::msg_send2(setScalesToFit:)]
    pub fn set_scales_to_fit(&mut self, val: bool);

    #[objc::msg_send2(preservesAspectRatio)]
    #[api::available(macos = 14.0)]
    pub fn preserves_aspect_ratio(&self) -> bool;

    #[objc::msg_send2(setPreservesAspectRatio:)]
    #[api::available(macos = 14.0)]
    pub fn set_preserves_aspect_ratio(&self, val: bool) -> bool;

    #[objc::msg_send2(showsCursor)]
    pub fn shows_cursor(&self) -> bool;

    #[objc::msg_send2(setShowsCursor:)]
    pub fn set_shows_cursor(&mut self, val: bool);

    /// SCStreamProperty that specifies whether to draw a circle around the cursor
    /// click, default is false. This property will not be affected by shows_cursor.
    /// This property currently applies when pixelFormat is set to BGRA.
    #[objc::msg_send2(showMouseClicks)]
    #[api::available(macos = 15.0)]
    pub fn show_mouse_clicks(&self) -> bool;

    /// SCStreamProperty that specifies whether to draw a circle around the cursor
    /// click, default is false. This property will not be affected by shows_cursor.
    /// This property currently applies when pixelFormat is set to BGRA.
    #[objc::msg_send2(setShowMouseClicks:)]
    #[api::available(macos = 15.0)]
    pub fn set_show_mouse_clicks(&mut self, val: bool);

    #[objc::msg_send2(backgroundColor)]
    pub fn background_color(&self) -> cg::Color;

    #[objc::msg_send2(setBackgroundColor:)]
    pub fn set_background_color(&mut self, val: cg::Color);

    #[objc::msg_send2(sourceRect)]
    pub fn src_rect(&self) -> cg::Rect;

    #[objc::msg_send2(setSourceRect:)]
    pub fn set_src_rect(&mut self, val: cg::Rect);

    #[objc::msg_send2(destinationRect)]
    pub fn dst_rect(&self) -> cg::Rect;

    #[objc::msg_send2(setDestinationRect:)]
    pub fn set_dst_rect(&self, val: cg::Rect);

    #[objc::msg_send2(queueDepth)]
    pub fn queue_depth(&self) -> isize;

    #[objc::msg_send2(setQueueDepth:)]
    pub fn set_queue_depth(&mut self, val: isize);

    /// Specifies the YCbCr matrix applied to the output surface.
    /// The value must be one of the strings specified
    /// [in](https://developer.apple.com/documentation/coregraphics/quartz_display_services/display_stream_ycbcr_to_rgb_conversion_matrix_options)
    /// Should only be used if your pixel format is 420v or 420f.
    #[objc::msg_send2(colorMatrix)]
    pub fn color_matrix(&self) -> &cf::String;

    #[objc::msg_send2(setColorMatrix:)]
    pub fn set_color_matrix(&self, val: &cf::String);

    /// The color space of the output buffer.  If not set the output buffer uses the same color
    /// space as the display. The value must be one of the strings specified
    /// [in](https://developer.apple.com/documentation/coregraphics/cgcolorspace/color_space_names)
    #[objc::msg_send2(colorSpaceName)]
    pub fn color_space_name(&self) -> &cf::String;

    #[objc::msg_send2(setColorSpaceName:)]
    pub fn set_color_space_name(&mut self, val: &cf::String);

    /// Specifies whether the audio will be captured.  By default audio is not captured.
    #[objc::msg_send2(capturesAudio)]
    #[api::available(macos = 13.0)]
    pub fn captures_audio(&self) -> bool;

    #[objc::msg_send2(setCapturesAudio:)]
    #[api::available(macos = 13.0)]
    pub fn set_captures_audio(&mut self, val: bool);

    /// The sample rate for audio. Default is set to 48000.
    #[objc::msg_send2(sampleRate)]
    #[api::available(macos = 13.0)]
    pub fn sample_rate(&self) -> i64;

    #[objc::msg_send2(setSampleRate:)]
    #[api::available(macos = 13.0)]
    pub fn set_sample_rate(&mut self, val: i64);

    /// Channel count. Default is set to two.
    #[objc::msg_send2(channelCount)]
    #[api::available(macos = 13.0)]
    pub fn channel_count(&self) -> i64;

    #[objc::msg_send2(setChannelCount:)]
    #[api::available(macos = 13.0)]
    pub fn set_channel_count(&mut self, val: i64);

    /// Whether to exclude audio from current process. Default is set to false.
    #[objc::msg_send2(excludesCurrentProcessAudio)]
    #[api::available(macos = 13.0)]
    pub fn excludes_current_process_audio(&self) -> bool;

    #[objc::msg_send2(setExcludesCurrentProcessAudio:)]
    #[api::available(macos = 13.0)]
    pub fn set_excludes_current_process_audio(&mut self, val: bool);

    /// Ignore framing on windows in the display sharing case (will ignore shadows).
    #[objc::msg_send2(ignoreShadowsDisplay)]
    #[api::available(macos = 14.0)]
    pub fn ignore_shadows_display(&self) -> bool;

    #[objc::msg_send2(setIgnoreShadowsDisplay:)]
    #[api::available(macos = 14.0)]
    pub fn set_ignore_shadows_display(&mut self, val: bool);

    /// Ignore framing on windows in the single window sharing case (will ignore shadows).
    #[objc::msg_send2(ignoreShadowsSingleWindow)]
    #[api::available(macos = 14.0)]
    pub fn ignore_shadows_single_window(&self) -> bool;

    #[objc::msg_send2(setIgnoreShadowsSingleWindow:)]
    #[api::available(macos = 14.0)]
    pub fn set_ignore_shadows_single_window(&mut self, val: bool);

    #[objc::msg_send2(captureResolution)]
    #[api::available(macos = 14.0)]
    pub fn capture_resolution(&self) -> sc::CaptureResolution;

    #[objc::msg_send2(setCaptureResolution:)]
    #[api::available(macos = 14.0)]
    pub fn set_capture_resolution(&mut self, val: sc::CaptureResolution);

    #[objc::msg_send2(capturesShadowsOnly)]
    #[api::available(macos = 14.0)]
    pub fn captures_shadows_only(&self) -> bool;

    #[objc::msg_send2(setCapturesShadowsOnly:)]
    #[api::available(macos = 14.0)]
    pub fn set_captures_shadows_only(&mut self, val: bool);

    /// Ensure partially transparent areas on windows are backed by
    /// a solid white color so that the resulting image is fully opaque.
    #[objc::msg_send2(shouldBeOpaque)]
    #[api::available(macos = 14.0)]
    pub fn should_be_opaque(&self) -> bool;

    #[objc::msg_send2(setShouldBeOpaque:)]
    #[api::available(macos = 14.0)]
    pub fn set_should_be_opaque(&mut self, val: bool);

    #[objc::msg_send2(ignoreGlobalClipDisplay)]
    #[api::available(macos = 14.0)]
    pub fn ignore_global_clip_display(&self) -> bool;

    #[objc::msg_send2(setIgnoreGlobalClipDisplay:)]
    #[api::available(macos = 14.0)]
    pub fn set_ignore_global_clip_display(&mut self, val: bool);

    /// Ignore framing on windows in the single window sharing case (will ignore shadows).
    #[objc::msg_send2(ignoreGlobalClipSingleWindow)]
    #[api::available(macos = 14.0)]
    pub fn ignore_global_clip_single_window(&self) -> bool;

    #[objc::msg_send2(setIgnoreGlobalClipSingleWindow:)]
    #[api::available(macos = 14.0)]
    pub fn set_ignore_global_clip_single_window(&mut self, val: bool);

    /// Informs the system if a privacy alert should be shown when using presenter overlay
    /// for a stream. Defaults to 'sc::PresenterOverlayAlertSetting::System';
    #[objc::msg_send2(presenterOverlayPrivacyAlertSetting)]
    #[api::available(macos = 14.0)]
    pub fn presenter_overlay_privacy_alert_setting(&self) -> PresenterOverlayAlertSetting;

    #[objc::msg_send2(setPresenterOverlayPrivacyAlertSetting:)]
    #[api::available(macos = 14.0)]
    pub fn set_presenter_overlay_privacy_alert_setting(
        &mut self,
        val: PresenterOverlayAlertSetting,
    );

    /// Show the child windows in display bound windows and applications sharing.
    /// Child windows are included by default.
    #[objc::msg_send2(includeChildWindows)]
    #[api::available(macos = 14.2)]
    pub fn include_child_windows(&self) -> bool;

    #[objc::msg_send2(setIncludeChildWindows:)]
    #[api::available(macos = 14.2)]
    pub fn set_include_child_windows(&mut self, val: bool);

    #[objc::msg_send2(captureMicrophone)]
    #[api::available(macos = 15.0)]
    pub fn capture_mic(&self) -> bool;

    #[objc::msg_send2(setCaptureMicrophone:)]
    #[api::available(macos = 15.0)]
    pub fn set_capture_mic(&mut self, val: bool);

    #[objc::msg_send2(microphoneCaptureDeviceID)]
    #[api::available(macos = 15.0)]
    pub fn mic_capture_device_id(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send2(setMicrophoneCaptureDeviceID:)]
    #[api::available(macos = 15.0)]
    pub fn set_mic_capture_device_id(&mut self, val: Option<&ns::String>);

    #[objc::msg_send2(captureDynamicRange)]
    #[api::available(macos = 15.0)]
    pub fn capture_dynamic_range(&self) -> CaptureDynamicRange;

    #[objc::msg_send2(setCaptureDynamicRange:)]
    #[api::available(macos = 15.0)]
    pub fn set_capture_dynamic_range(&mut self, val: CaptureDynamicRange);

    #[objc::msg_send2(streamConfigurationWithPreset:)]
    #[api::available(macos = 15.0)]
    pub fn with_preset(preset: CfgPreset) -> arc::R<Self>;
}

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_STREAM_CONFIGURATION: &'static objc::Class<Cfg>;
    static SC_CONTENT_FILTER: &'static objc::Class<ContentFilter>;
    static SC_STREAM: &'static objc::Class<Stream>;
}

#[link(name = "ScreenCaptureKit", kind = "framework")]
#[api::weak]
extern "C" {
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoStatus: &'static FrameInfo;
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoDisplayTime: &'static FrameInfo;
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoScaleFactor: &'static FrameInfo;
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoContentScale: &'static FrameInfo;
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoContentRect: &'static FrameInfo;
    #[api::available(macos = 12.3)]
    static SCStreamFrameInfoDirtyRects: &'static FrameInfo;
    #[api::available(macos = 13.1)]
    static SCStreamFrameInfoScreenRect: &'static FrameInfo;
    #[api::available(macos = 14.0)]
    static SCStreamFrameInfoBoundingRect: &'static FrameInfo;
    #[api::available(macos = 14.2)]
    static SCStreamFrameInfoPresenterOverlayContentRect: &'static FrameInfo;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CaptureDynamicRange {
    Sdr,
    HdrLocalDisplay,
    HdrCanonicalDisplay,
}

define_obj_type!(pub ContentFilter(ns::Id));

impl arc::A<ContentFilter> {
    #[objc::msg_send2(initWithDesktopIndependentWindow:)]
    pub fn init_with_desktop_independent_window(
        self,
        window: &sc::Window,
    ) -> arc::Retained<ContentFilter>;

    #[objc::msg_send2(initWithDisplay:excludingWindows:)]
    pub fn init_with_display_excluding_windows(
        self,
        display: &sc::Display,
        windows: &ns::Array<sc::Window>,
    ) -> arc::Retained<ContentFilter>;
}

impl ContentFilter {
    define_cls!(SC_CONTENT_FILTER);

    /// Will create a sc::ContentFilter that captures just the independent window passed in.
    pub fn with_desktop_independent_window(window: &sc::Window) -> arc::R<ContentFilter> {
        Self::alloc().init_with_desktop_independent_window(window)
    }

    pub fn with_display_excluding_windows(
        display: &sc::Display,
        windows: &ns::Array<sc::Window>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_display_excluding_windows(display, windows)
    }

    #[objc::msg_send2(style)]
    #[api::available(macos = 14.0)]
    pub fn style(&self) -> sc::ShareableContentStyle;

    #[objc::msg_send2(pointPixelScale)]
    #[api::available(macos = 14.0)]
    pub fn point_pixel_scale(&self) -> f32;

    #[objc::msg_send2(contentRect)]
    #[api::available(macos = 14.0)]
    pub fn content_rect(&self) -> cg::Rect;

    #[objc::msg_send2(includeMenuBar)]
    #[api::available(macos = 14.2)]
    pub fn include_menu_bar(&self) -> bool;

    /// To include menu bar as part of the capture. This property has no effect for the
    /// desktop independent window filter. For content filters created with initWithDisplay:excluding,
    /// the default value is 'true'. Display excluding content filters contains the desktop
    /// and dock. For content filters created with initWithDisplay:including, the default
    /// value is 'false'. Display including content filters do not contain the desktop and dock
    #[objc::msg_send(setIncludeMenuBar:)]
    #[api::available(macos = 14.2)]
    pub fn set_include_menu_bar(&mut self, val: bool);
}

define_obj_type!(
    /// An object that represents a stream of shareable content.
    #[doc(alias = "SCStream")]
    pub Stream(ns::Id)
);

#[objc::obj_trait]
pub trait Output: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(stream:didOutputSampleBuffer:ofType:)]
    fn stream_did_output_sample_buf(
        &mut self,
        stream: &Stream,
        sample_bufer: &mut cm::SampleBuf,
        kind: OutputType,
    );
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(stream:didStopWithError:)]
    fn stream_did_stop_with_err(&mut self, stream: &Stream, error: &ns::Error);

    #[objc::optional]
    #[objc::msg_send(userDidStopStream:)]
    fn user_did_stop_stream(&mut self, stream: &Stream);

    #[objc::optional]
    #[objc::msg_send(outputVideoEffectDidStartForStream:)]
    fn output_video_effect_did_start_for_stream(&mut self, stream: &Stream);

    #[objc::optional]
    #[objc::msg_send(outputVideoEffectDidStopForStream:)]
    fn output_video_effect_did_stop_for_stream(&mut self, stream: &Stream);
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

impl arc::A<Stream> {
    #[objc::msg_send(initWithFilter:configuration:delegate:)]
    pub fn init_with_filter_configuration_delegate<D: Delegate>(
        self,
        filter: &ContentFilter,
        configuration: &Cfg,
        delegate: Option<&D>,
    ) -> arc::R<Stream>;
}

impl Stream {
    define_cls!(SC_STREAM);

    pub fn with_delegate<T, D: Delegate>(
        filter: &ContentFilter,
        configuration: &Cfg,
        delegate: &D,
    ) -> arc::R<Self> {
        Self::alloc().init_with_filter_configuration_delegate::<D>(
            filter,
            configuration,
            Some(delegate),
        )
    }

    pub fn new(filter: &ContentFilter, configuration: &Cfg) -> arc::R<Self> {
        Self::alloc().init_with_filter_configuration_delegate::<AnyDelegate>(
            filter,
            configuration,
            None,
        )
    }

    #[objc::msg_send(addStreamOutput:type:sampleHandlerQueue:error:)]
    fn add_stream_output_type_sample_handler_queue_err<D: Output>(
        &self,
        output: &D,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: *mut Option<&ns::Error>,
    ) -> bool;

    pub fn add_stream_output<'ar, D: Output>(
        &self,
        output: &D,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.add_stream_output_type_sample_handler_queue_err(
            output,
            output_type,
            queue,
            &mut error,
        ) {
            return Ok(());
        }
        Err(error.unwrap())
    }

    #[objc::msg_send2(startCaptureWithCompletionHandler:)]
    pub fn start_with_ch(&self, ch: Option<&mut blocks::ErrCompletionHandler>);

    #[objc::msg_send2(stopCaptureWithCompletionHandler:)]
    pub fn stop_with_ch(&self, ch: Option<&mut blocks::ErrCompletionHandler>);

    pub async fn start(&self) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        self.start_with_ch(Some(&mut block));
        future.await
    }

    pub async fn stop(&self) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        self.stop_with_ch(Some(&mut block));
        future.await
    }

    #[objc::msg_send2(addRecordingOutput:error:)]
    #[api::available(macos = 15.0)]
    pub unsafe fn add_recording_output_err<'ar>(
        &mut self,
        val: &sc::RecordingOutput,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[api::available(macos = 15.0)]
    pub fn add_recording_output<'ar>(
        &mut self,
        val: &sc::RecordingOutput,
    ) -> Result<(), &'ar ns::Error> {
        ns::if_false(|err| unsafe { self.add_recording_output_err(val, err) })
    }

    #[objc::msg_send2(removeRecordingOutput:error:)]
    #[api::available(macos = 15.0)]
    pub unsafe fn remove_recording_output_err<'ar>(
        &mut self,
        val: &sc::RecordingOutput,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[api::available(macos = 15.0)]
    pub fn remove_recording_output<'ar>(
        &mut self,
        val: &sc::RecordingOutput,
    ) -> Result<(), &'ar ns::Error> {
        ns::if_false(|err| unsafe { self.remove_recording_output_err(val, err) })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        cm, cv, define_obj_type, dispatch, ns, objc, sc, sc::stream::Output, sc::stream::OutputImpl,
    };

    define_obj_type!(
        FrameCounter + sc::stream::OutputImpl,
        usize,
        FRAME_COUNTER_CLS
    );

    impl FrameCounter {
        // fn counter(&self) -> usize {
        //     *self.inner()
        // }
    }

    impl Output for FrameCounter {}

    #[objc::add_methods]
    impl OutputImpl for FrameCounter {}

    #[test]
    fn basics() {
        let mut cfg = sc::StreamCfg::new();

        cfg.set_width(200);
        assert_eq!(200, cfg.width());
        cfg.set_height(300);
        assert_eq!(300, cfg.height());

        cfg.set_minimum_frame_interval(cm::Time::new(1, 60));
        cfg.set_pixel_format(cv::PixelFormat::_32_BGRA);
        cfg.set_scales_to_fit(false);
        cfg.set_shows_cursor(true);
    }

    #[tokio::test]
    async fn start_fails() {
        let q = dispatch::Queue::serial_with_ar_pool();
        let content = sc::ShareableContent::current().await.expect("content");
        let ref display = content.displays()[0];
        let mut cfg = sc::StreamCfg::new();
        cfg.set_width(display.width() as usize * 2);
        cfg.set_height(display.height() as usize * 2);

        let windows = ns::Array::new();
        let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
        let stream = sc::Stream::new(&filter, &cfg);
        let delegate = FrameCounter::with(0);
        stream
            .add_stream_output(delegate.as_ref(), sc::OutputType::Screen, Some(&q))
            .unwrap();
        stream.start().await.expect("started");
        stream.start().await.expect_err("already started");

        tokio::time::sleep(Duration::from_secs(2)).await;

        stream.stop().await.expect("stopped");
        stream.stop().await.expect_err("already stopped");
        // println!(
        //     "------- {:?} {:?}",
        //     d.obj.as_type_ref(),
        //     d.delegate.counter()
        // );

        // assert!(d.delegate.counter() > 10, "{:?}", d.delegate.counter);
    }
}
