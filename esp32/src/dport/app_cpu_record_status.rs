#[doc = "Register `APP_CPU_RECORD_STATUS` reader"]
pub struct R(crate::R<APP_CPU_RECORD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APP_CPU_RECORDING` reader - "]
pub struct APP_CPU_RECORDING_R(crate::FieldReader<bool, bool>);
impl APP_CPU_RECORDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CPU_RECORDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CPU_RECORDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_recording(&self) -> APP_CPU_RECORDING_R {
        APP_CPU_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_status]
(index.html) module"]
pub struct APP_CPU_RECORD_STATUS_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_status::R]
(R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_STATUS to value 0"]
impl crate::Resettable for APP_CPU_RECORD_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
