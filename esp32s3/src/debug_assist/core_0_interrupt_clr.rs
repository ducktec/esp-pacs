#[doc = "Register `CORE_0_INTERRUPT_CLR` reader"]
pub struct R(crate::R<CORE_0_INTERRUPT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTERRUPT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTERRUPT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTERRUPT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTERRUPT_CLR` writer"]
pub struct W(crate::W<CORE_0_INTERRUPT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTERRUPT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_INTERRUPT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTERRUPT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` reader - Core0 dram0 area0 read monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_0_RD_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_RD_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_RD_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_RD_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - Core0 dram0 area0 read monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_0_RD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_RD_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` reader - Core0 dram0 area0 write monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_0_WR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_WR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_WR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_WR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - Core0 dram0 area0 write monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_0_WR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_0_WR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` reader - Core0 dram0 area1 read monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_1_RD_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_RD_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_RD_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_RD_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` writer - Core0 dram0 area1 read monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_1_RD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_RD_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` reader - Core0 dram0 area1 write monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_1_WR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_WR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_WR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_WR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` writer - Core0 dram0 area1 write monitor interrupt clr"]
pub struct CORE_0_AREA_DRAM0_1_WR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_DRAM0_1_WR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` reader - Core0 PIF area0 read monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_0_RD_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_RD_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_RD_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_RD_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` writer - Core0 PIF area0 read monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_0_RD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_RD_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` reader - Core0 PIF area0 write monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_0_WR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_WR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_WR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_WR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` writer - Core0 PIF area0 write monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_0_WR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_WR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` reader - Core0 PIF area1 read monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_1_RD_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_RD_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_RD_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_RD_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` writer - Core0 PIF area1 read monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_1_RD_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_RD_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` reader - Core0 PIF area1 write monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_1_WR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_WR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_WR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_WR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` writer - Core0 PIF area1 write monitor interrupt clr"]
pub struct CORE_0_AREA_PIF_1_WR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_1_WR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` reader - Core0 stackpoint overflow monitor interrupt clr"]
pub struct CORE_0_SP_SPILL_MIN_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MIN_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MIN_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MIN_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - Core0 stackpoint overflow monitor interrupt clr"]
pub struct CORE_0_SP_SPILL_MIN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MIN_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` reader - Core0 stackpoint underflow monitor interrupt clr"]
pub struct CORE_0_SP_SPILL_MAX_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MAX_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MAX_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MAX_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - Core0 stackpoint underflow monitor interrupt clr"]
pub struct CORE_0_SP_SPILL_MAX_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_SP_SPILL_MAX_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` reader - IBUS busy monitor interrupt clr"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` reader - DBUS busy monitor interrupt clr"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(&self) -> CORE_0_AREA_DRAM0_0_RD_CLR_R {
        CORE_0_AREA_DRAM0_0_RD_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(&self) -> CORE_0_AREA_DRAM0_0_WR_CLR_R {
        CORE_0_AREA_DRAM0_0_WR_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_clr(&self) -> CORE_0_AREA_DRAM0_1_RD_CLR_R {
        CORE_0_AREA_DRAM0_1_RD_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_clr(&self) -> CORE_0_AREA_DRAM0_1_WR_CLR_R {
        CORE_0_AREA_DRAM0_1_WR_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_clr(&self) -> CORE_0_AREA_PIF_0_RD_CLR_R {
        CORE_0_AREA_PIF_0_RD_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_clr(&self) -> CORE_0_AREA_PIF_0_WR_CLR_R {
        CORE_0_AREA_PIF_0_WR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_clr(&self) -> CORE_0_AREA_PIF_1_RD_CLR_R {
        CORE_0_AREA_PIF_1_RD_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_clr(&self) -> CORE_0_AREA_PIF_1_WR_CLR_R {
        CORE_0_AREA_PIF_1_WR_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&self) -> CORE_0_SP_SPILL_MIN_CLR_R {
        CORE_0_SP_SPILL_MIN_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&self) -> CORE_0_SP_SPILL_MAX_CLR_R {
        CORE_0_SP_SPILL_MAX_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_clr(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_clr(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_0_RD_CLR_W {
        CORE_0_AREA_DRAM0_0_RD_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_0_WR_CLR_W {
        CORE_0_AREA_DRAM0_0_WR_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_1_RD_CLR_W {
        CORE_0_AREA_DRAM0_1_RD_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_1_WR_CLR_W {
        CORE_0_AREA_DRAM0_1_WR_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_clr(&mut self) -> CORE_0_AREA_PIF_0_RD_CLR_W {
        CORE_0_AREA_PIF_0_RD_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_clr(&mut self) -> CORE_0_AREA_PIF_0_WR_CLR_W {
        CORE_0_AREA_PIF_0_WR_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_clr(&mut self) -> CORE_0_AREA_PIF_1_RD_CLR_W {
        CORE_0_AREA_PIF_1_RD_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_clr(&mut self) -> CORE_0_AREA_PIF_1_WR_CLR_W {
        CORE_0_AREA_PIF_1_WR_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Core0 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W {
        CORE_0_SP_SPILL_MIN_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Core0 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W {
        CORE_0_SP_SPILL_MAX_CLR_W { w: self }
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_clr(&mut self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W { w: self }
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_clr(&mut self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor interrupt clr register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_interrupt_clr]
(index.html) module"]
pub struct CORE_0_INTERRUPT_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTERRUPT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_interrupt_clr::R]
(R) reader structure"]
impl crate::Readable for CORE_0_INTERRUPT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_interrupt_clr::W]
(W) writer structure"]
impl crate::Writable for CORE_0_INTERRUPT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTERRUPT_CLR to value 0"]
impl crate::Resettable for CORE_0_INTERRUPT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
