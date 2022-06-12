use super::*;

pub enum MTLCounterSampleBuffer {}

foreign_obj_type! {
    type CType = MTLCounterSampleBuffer;
    pub struct CounterSampleBuffer;
    pub struct CounterSampleBufferRef;
}

impl CounterSampleBufferRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, sampleCount] }
    }
}
