#[doc = "Register `SYS_CONF` reader"]
pub struct R(crate::R<SYS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CONF` writer"]
pub struct W(crate::W<SYS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CONF_SPEC>;
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
impl From<crate::W<SYS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_FIFO_MASK` reader - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub struct APB_FIFO_MASK_R(crate::FieldReader<bool, bool>);
impl APB_FIFO_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_FIFO_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_FIFO_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_FIFO_MASK` writer - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub struct APB_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_FIFO_MASK_W<'a> {
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
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit to enable the clock for RMT memory."]
pub struct MEM_CLK_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl MEM_CLK_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_CLK_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_CLK_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit to enable the clock for RMT memory."]
pub struct MEM_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLK_FORCE_ON_W<'a> {
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
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to power down RMT memory."]
pub struct MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to power down RMT memory."]
pub struct MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `MEM_FORCE_PU` reader - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub struct MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PU` writer - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub struct MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PU_W<'a> {
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
#[doc = "Field `SCLK_DIV_NUM` reader - the integral part of the fractional divisor"]
pub struct SCLK_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_NUM` writer - the integral part of the fractional divisor"]
pub struct SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `SCLK_DIV_A` reader - the numerator of the fractional part of the fractional divisor"]
pub struct SCLK_DIV_A_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_A` writer - the numerator of the fractional part of the fractional divisor"]
pub struct SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `SCLK_DIV_B` reader - the denominator of the fractional part of the fractional divisor"]
pub struct SCLK_DIV_B_R(crate::FieldReader<u8, u8>);
impl SCLK_DIV_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_DIV_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_DIV_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_DIV_B` writer - the denominator of the fractional part of the fractional divisor"]
pub struct SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `SCLK_SEL` reader - choose the clock source of rmt_sclk. 1:CLK_80Mhz;2:CLK_8MHz; 2:XTAL"]
pub struct SCLK_SEL_R(crate::FieldReader<u8, u8>);
impl SCLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_SEL` writer - choose the clock source of rmt_sclk. 1:CLK_80Mhz;2:CLK_8MHz; 2:XTAL"]
pub struct SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `SCLK_ACTIVE` reader - rmt_sclk switch"]
pub struct SCLK_ACTIVE_R(crate::FieldReader<bool, bool>);
impl SCLK_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_ACTIVE` writer - rmt_sclk switch"]
pub struct SCLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to power down RMT memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - the integral part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:17 - the numerator of the fractional part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - the denominator of the fractional part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - choose the clock source of rmt_sclk. 1:CLK_80Mhz;2:CLK_8MHz; 2:XTAL"]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - rmt_sclk switch"]
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W {
        APB_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W {
        MEM_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to power down RMT memory."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 3 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
        MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bits 4:11 - the integral part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W {
        SCLK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 12:17 - the numerator of the fractional part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W {
        SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 18:23 - the denominator of the fractional part of the fractional divisor"]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W {
        SCLK_DIV_B_W { w: self }
    }
    #[doc = "Bits 24:25 - choose the clock source of rmt_sclk. 1:CLK_80Mhz;2:CLK_8MHz; 2:XTAL"]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W {
        SCLK_SEL_W { w: self }
    }
    #[doc = "Bit 26 - rmt_sclk switch"]
    #[inline(always)]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W {
        SCLK_ACTIVE_W { w: self }
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT apb configuration register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_conf]
(index.html) module"]
pub struct SYS_CONF_SPEC;
impl crate::RegisterSpec for SYS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_conf::R]
(R) reader structure"]
impl crate::Readable for SYS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_conf::W]
(W) writer structure"]
impl crate::Writable for SYS_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CONF to value 0x0500_0010"]
impl crate::Resettable for SYS_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500_0010
    }
}
