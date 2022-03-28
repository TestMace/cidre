use crate::{define_mtl, define_obj_type, objc::Id};

define_obj_type!(CounterSampleBuffer(Id));

impl CounterSampleBuffer {
    define_mtl!(device, label);
}

#[repr(C)]
pub struct CounterResultStatistic {
    pub tessellation_input_patches: u64,
    pub vertex_invocations: u64,
    pub post_tessellation_vertex_invocations: u64,
    pub clipper_invocations: u64,
    pub clipper_primitives_out: u64,
    pub fragment_invocations: u64,
    pub fragments_passed: u64,
    pub compute_kernel_invocations: u64,
}

define_obj_type!(Counter(Id));

define_obj_type!(CounterSet(Id));

define_obj_type!(Descriptor(Id));

impl Descriptor {
    define_mtl!(label, set_label, storage_mode);
}
