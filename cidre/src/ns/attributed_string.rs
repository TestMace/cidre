use crate::{arc, cf, define_obj_type, define_opts, ns, objc};

define_obj_type!(
    #[doc(alias = "NSAttributedStringKey")]
    pub AttrStringKey(ns::String)
);

impl AttrStringKey {
    pub fn from_ns_string(string: &ns::String) -> arc::R<Self> {
        unsafe { std::mem::transmute(string.retained()) }
    }
}

define_obj_type!(
    #[doc(alias = "NSAttributedStringFormattingContextKey")]
    pub AttrStringFormatCtxKey(ns::String)
);

define_obj_type!(
    #[doc(alias = "NSAttributedString")]
    pub AttrString(ns::Id),
    NS_ATTRIBUTED_STRING
);

impl arc::A<AttrString> {
    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, str: &ns::String) -> arc::R<AttrString>;

    #[objc::msg_send(initWithAttributedString:)]
    pub fn init_with_attr_string(self, str: &ns::AttrString) -> arc::R<AttrString>;
}

impl AttrString {
    /// For performance reasons, this property returns the current backing store of
    /// the attributed string object. If you want to maintain a snapshot of this as
    /// you manipulate the returned string, you should make a copy of the appropriate substring.
    #[objc::msg_send(string)]
    pub fn string(&self) -> &ns::String;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(attributesAtIndex:effectiveRange:)]
    pub unsafe fn attrs_at_throws_ar(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> arc::Rar<ns::Dictionary<AttrStringKey, ns::Id>>;

    #[objc::rar_retain]
    pub unsafe fn attrs_at_throws(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> arc::R<ns::Dictionary<AttrStringKey, ns::Id>>;

    pub fn attrs_at<'ar>(
        &self,
        index: usize,
        effective_range: Option<&ns::Range>,
    ) -> Result<arc::R<ns::Dictionary<AttrStringKey, ns::Id>>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.attrs_at_throws(index, effective_range) })
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(str)
    }

    #[inline]
    pub fn with_attr_string(str: &ns::AttrString) -> arc::R<Self> {
        Self::alloc().init_with_attr_string(str)
    }

    #[inline]
    pub fn as_cf(&self) -> &cf::AttrString {
        unsafe { std::mem::transmute(self) }
    }

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::R<AttrString>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::R<AttrStringMut>;
}

define_obj_type!(
    #[doc(alias = "NSMutableAttributedString")]
    pub AttrStringMut(AttrString),
    NS_MUTABLE_ATTRIBUTED_STRING
);

impl AttrStringMut {
    #[objc::msg_send(mutableString)]
    pub fn string_mut(&mut self) -> &ns::StringMut;

    #[objc::msg_send(addAttribute:value:range:)]
    pub unsafe fn add_attr_throws(
        &mut self,
        name: &ns::AttrStringKey,
        val: &ns::Id,
        range: ns::Range,
    );

    pub fn add_attr<'ear>(
        &mut self,
        name: &ns::AttrStringKey,
        val: &ns::Id,
        range: ns::Range,
    ) -> Result<(), &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.add_attr_throws(name, val, range) })
    }

    #[objc::msg_send(addAttributes:range:)]
    pub unsafe fn add_attrs_throws(
        &mut self,
        attrs: &ns::Dictionary<ns::AttrStringKey, ns::Id>,
        range: ns::Range,
    );

    pub fn add_attrs<'ear>(
        &mut self,
        attrs: &ns::Dictionary<ns::AttrStringKey, ns::Id>,
        range: ns::Range,
    ) -> Result<(), &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.add_attrs_throws(attrs, range) })
    }

    #[objc::msg_send(removeAttribute:range:)]
    pub unsafe fn remove_attr_throws(&mut self, name: &ns::AttrStringKey, range: ns::Range);

    pub fn remove_attr<'ear>(
        &mut self,
        name: &ns::AttrStringKey,
        range: ns::Range,
    ) -> Result<(), &'ear ns::Exception> {
        ns::try_catch(|| unsafe { self.remove_attr_throws(name, range) })
    }

    #[objc::msg_send(appendAttributedString:)]
    pub fn append_attr_string(&mut self, val: &ns::AttrString);
}

define_opts!(
    #[doc(alias = "NSInlinePresentationIntent")]
    pub InlinePresentationIntent(usize)
);

impl InlinePresentationIntent {
    pub const EMPHASIZED: Self = Self(1 << 0);
    pub const STRONGLY_EMPHASIZED: Self = Self(1 << 1);
    pub const CODE: Self = Self(1 << 2);
    pub const STRIKETHROUGH: Self = Self(1 << 5);
    pub const SOFT_BREAK: Self = Self(1 << 6);
    pub const LINE_BREAK: Self = Self(1 << 7);
    pub const INLINE_HTML: Self = Self(1 << 8);
    pub const BLOCK_HTML: Self = Self(1 << 9);
}

extern "C" {
    static NS_ATTRIBUTED_STRING: &'static objc::Class<AttrString>;
    static NS_MUTABLE_ATTRIBUTED_STRING: &'static objc::Class<AttrStringMut>;
}

#[cfg(test)]
mod tests {
    use crate::{ct, ns, objc::Obj};

    #[test]
    fn basics() {
        let str = ns::String::with_str("Hello");
        let astr = ns::AttrString::with_string(&str);

        assert_eq!(astr.string(), &str);
        let attrs = astr.attrs_at(0, None).unwrap();
        assert!(attrs.is_empty());

        let copy = astr.copy_mut();
        let copy = copy.copy();
        let copy = ns::AttrString::with_attr_string(&copy);

        assert_eq!(copy.len(), 5);

        let mut mcopy = copy.copy_mut();

        // let mut mstr = mcopy.string_mut();

        let color_key = ct::StringAttrName::foreground_color().as_ns_attr_string_key();
        let color_val = ns::Color::with_rgba(0.5, 0.5, 1.0, 1.0);

        let attrs = ns::Dictionary::with_keys_values(&[color_key], &[color_val.as_id_ref()]);

        mcopy.add_attrs(&attrs, ns::Range::new(0, 5)).expect("ok");
        mcopy
            .add_attr(color_key, &color_val, ns::Range::new(1000, 10))
            .expect_err("ok");
        mcopy
            .add_attr(color_key, &color_val, ns::Range::new(0, 3))
            .expect("ok");

        mcopy
            .remove_attr(color_key, ns::Range::new(100, 200))
            .expect_err("should be out of bounds");

        mcopy
            .remove_attr(color_key, ns::Range::new(0, 5))
            .expect("failed to remove valid range");
        // TODO: investigate
        // astr.attrs_at(1000, Some(&ns::Range::new(1000, 1000)))
        // .expect_err("Should be Exception");
    }
}
