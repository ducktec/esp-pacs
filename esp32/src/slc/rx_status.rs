#[doc = "Register `RX_STATUS` reader"]
pub struct R(crate::R<RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_RX_FULL` reader - "]
pub struct SLC0_RX_FULL_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_EMPTY` reader - "]
pub struct SLC0_RX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_FULL` reader - "]
pub struct SLC1_RX_FULL_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_EMPTY` reader - "]
pub struct SLC1_RX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_rx_full(&self) -> SLC0_RX_FULL_R {
        SLC0_RX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_rx_empty(&self) -> SLC0_RX_EMPTY_R {
        SLC0_RX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_full(&self) -> SLC1_RX_FULL_R {
        SLC1_RX_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_empty(&self) -> SLC1_RX_EMPTY_R {
        SLC1_RX_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_status]
(index.html) module"]
pub struct RX_STATUS_SPEC;
impl crate::RegisterSpec for RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_status::R]
(R) reader structure"]
impl crate::Readable for RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_STATUS to value 0x0002_0002"]
impl crate::Resettable for RX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0002
    }
}
