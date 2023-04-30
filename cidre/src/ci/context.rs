use crate::{arc, cg, ci, define_obj_type, ns, objc};

define_obj_type!(Context(ns::Id), CI_CONTEXT);

impl arc::A<Context> {
    #[objc::msg_send(initWithOptions:)]
    pub fn init_with_options(
        self,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Context>>;
}

impl Context {
    #[inline]
    pub fn with_options(
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_options(options)
    }

    #[objc::msg_send(writePNGRepresentationOfImage:toURL:format:colorSpace:options:error:)]
    fn write_png_to_url_format_colorspace_options_err<'ar>(
        &self,
        image: &ci::Image,
        url: &ns::URL,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
        error: &mut Option<&'ar ns::Error>,
    ) -> bool;

    pub fn write_png_to_url<'ar>(
        &self,
        image: &ci::Image,
        url: &ns::URL,
        format: ci::Format,
        color_space: &cg::ColorSpace,
        options: &ns::Dictionary<ns::String, ns::Id>,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        let res = self.write_png_to_url_format_colorspace_options_err(
            image,
            url,
            format,
            color_space,
            options,
            &mut error,
        );

        if res {
            Ok(())
        } else {
            Err(error.unwrap())
        }
    }
}

#[link(name = "ci", kind = "static")]
extern "C" {
    static CI_CONTEXT: &'static objc::Class<Context>;
}
