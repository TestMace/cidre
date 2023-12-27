use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MLCTensorData")]
    pub TensorData(ns::Id)
);

impl TensorData {
    define_cls!(MLC_TENSOR_DATA);

    #[objc::msg_send(bytes)]
    pub fn bytes(&self) -> *const u8;

    #[objc::msg_send(bytes)]
    pub fn bytes_mut(&mut self) -> *mut u8;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::cls_msg_send(dataWithImmutableBytesNoCopy:length:)]
    pub fn with_bytes_no_copy_ar(bytes: *const u8, length: usize) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_bytes_no_copy(bytes: *const u8, length: usize) -> arc::R<Self>;

    #[inline]
    pub fn with_slice_no_copy_ar<T: Sized>(slice: &[T]) -> arc::Rar<Self> {
        Self::with_bytes_no_copy_ar(slice.as_ptr() as _, std::mem::size_of_val(slice))
    }

    #[objc::cls_rar_retain]
    pub fn with_slice_no_copy<T>(slice: &[T]) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR_DATA: &'static objc::Class<TensorData>;
}
