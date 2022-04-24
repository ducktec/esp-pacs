#[doc = "Register `CH2CONF1` reader"]
pub struct R(crate::R<CH2CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CONF1` writer"]
pub struct W(crate::W<CH2CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CONF1_SPEC>;
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
impl From<crate::W<CH2CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN` reader - reg_rx_en_ch2."]
pub struct RX_EN_R(crate::FieldReader<bool, bool>);
impl RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN` writer - reg_rx_en_ch2."]
pub struct RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_W<'a> {
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
#[doc = "Field `MEM_WR_RST` writer - reg_mem_wr_rst_ch2."]
pub struct MEM_WR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_RST_W<'a> {
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
#[doc = "Field `APB_MEM_RST` writer - reg_apb_mem_rst_ch2."]
pub struct APB_MEM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_W<'a> {
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
#[doc = "Field `MEM_OWNER` reader - reg_mem_owner_ch2."]
pub struct MEM_OWNER_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER` writer - reg_mem_owner_ch2."]
pub struct MEM_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_OWNER_W<'a> {
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
#[doc = "Field `RX_FILTER_EN` reader - reg_rx_filter_en_ch2."]
pub struct RX_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl RX_FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_EN` writer - reg_rx_filter_en_ch2."]
pub struct RX_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_EN_W<'a> {
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
#[doc = "Field `RX_FILTER_THRES` reader - reg_rx_filter_thres_ch2."]
pub struct RX_FILTER_THRES_R(crate::FieldReader<u8, u8>);
impl RX_FILTER_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_THRES` writer - reg_rx_filter_thres_ch2."]
pub struct RX_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | ((value as u32 & 0xff) << 5);
        self.w
    }
}
#[doc = "Field `MEM_RX_WRAP_EN` reader - reg_mem_rx_wrap_en_ch2."]
pub struct MEM_RX_WRAP_EN_R(crate::FieldReader<bool, bool>);
impl MEM_RX_WRAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_RX_WRAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RX_WRAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RX_WRAP_EN` writer - reg_mem_rx_wrap_en_ch2."]
pub struct MEM_RX_WRAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RX_WRAP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `AFIFO_RST` writer - reg_afifo_rst_ch2."]
pub struct AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `CONF_UPDATE` writer - reg_conf_update_ch2."]
pub struct CONF_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reg_rx_en_ch2."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch2."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch2."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch2."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch2."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&self) -> MEM_RX_WRAP_EN_R {
        MEM_RX_WRAP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rx_en_ch2."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W {
        RX_EN_W { w: self }
    }
    #[doc = "Bit 1 - reg_mem_wr_rst_ch2."]
    #[inline(always)]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W {
        MEM_WR_RST_W { w: self }
    }
    #[doc = "Bit 2 - reg_apb_mem_rst_ch2."]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W {
        APB_MEM_RST_W { w: self }
    }
    #[doc = "Bit 3 - reg_mem_owner_ch2."]
    #[inline(always)]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W {
        MEM_OWNER_W { w: self }
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch2."]
    #[inline(always)]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W {
        RX_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch2."]
    #[inline(always)]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W {
        RX_FILTER_THRES_W { w: self }
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch2."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&mut self) -> MEM_RX_WRAP_EN_W {
        MEM_RX_WRAP_EN_W { w: self }
    }
    #[doc = "Bit 14 - reg_afifo_rst_ch2."]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W {
        AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 15 - reg_conf_update_ch2."]
    #[inline(always)]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W {
        CONF_UPDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH2CONF1_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2conf1]
(index.html) module"]
pub struct CH2CONF1_SPEC;
impl crate::RegisterSpec for CH2CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2conf1::R]
(R) reader structure"]
impl crate::Readable for CH2CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2conf1::W]
(W) writer structure"]
impl crate::Writable for CH2CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CONF1 to value 0x01e8"]
impl crate::Resettable for CH2CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01e8
    }
}
