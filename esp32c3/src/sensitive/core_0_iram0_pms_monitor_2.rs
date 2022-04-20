#[doc = "Register `CORE_0_IRAM0_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR` reader - core_0_iram0_pms_monitor_violate_intr"]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR` reader - core_0_iram0_pms_monitor_violate_status_wr"]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE` reader - core_0_iram0_pms_monitor_violate_status_loadstore"]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - core_0_iram0_pms_monitor_violate_status_world"]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader<u8, u8>);
impl CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - core_0_iram0_pms_monitor_violate_status_addr"]
pub struct CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader<u32, u32>);
impl CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - core_0_iram0_pms_monitor_violate_intr"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_intr(&self) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - core_0_iram0_pms_monitor_violate_status_wr"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_wr(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - core_0_iram0_pms_monitor_violate_status_loadstore"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_loadstore(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - core_0_iram0_pms_monitor_violate_status_world"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_world(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:28 - core_0_iram0_pms_monitor_violate_status_addr"]
    #[inline(always)]
    pub fn core_0_iram0_pms_monitor_violate_status_addr(
        &self,
    ) -> CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(((self.bits >> 5) & 0x00ff_ffff) as u32)
    }
}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2_REG\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_iram0_pms_monitor_2]
(index.html) module"]
pub struct CORE_0_IRAM0_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_iram0_pms_monitor_2::R]
(R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_IRAM0_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_0_IRAM0_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
