#[doc = "Register `CH%s_TX_CONF0` reader"]
pub struct R(crate::R<CH_TX_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_TX_CONF0` writer"]
pub struct W(crate::W<CH_TX_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TX_CONF0_SPEC>;
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
impl From<crate::W<CH_TX_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TX_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START` writer - reg_tx_start_ch0."]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
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
#[doc = "Field `MEM_RD_RST` writer - reg_mem_rd_rst_ch0."]
pub struct MEM_RD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RD_RST_W<'a> {
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
#[doc = "Field `APB_MEM_RST` writer - reg_apb_mem_rst_ch0."]
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
#[doc = "Field `TX_CONTI_MODE` reader - reg_tx_conti_mode_ch0."]
pub struct TX_CONTI_MODE_R(crate::FieldReader<bool, bool>);
impl TX_CONTI_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CONTI_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CONTI_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CONTI_MODE` writer - reg_tx_conti_mode_ch0."]
pub struct TX_CONTI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CONTI_MODE_W<'a> {
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
#[doc = "Field `MEM_TX_WRAP_EN` reader - reg_mem_tx_wrap_en_ch0."]
pub struct MEM_TX_WRAP_EN_R(crate::FieldReader<bool, bool>);
impl MEM_TX_WRAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TX_WRAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TX_WRAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TX_WRAP_EN` writer - reg_mem_tx_wrap_en_ch0."]
pub struct MEM_TX_WRAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TX_WRAP_EN_W<'a> {
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
#[doc = "Field `IDLE_OUT_LV` reader - reg_idle_out_lv_ch0."]
pub struct IDLE_OUT_LV_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_LV` writer - reg_idle_out_lv_ch0."]
pub struct IDLE_OUT_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_LV_W<'a> {
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
#[doc = "Field `IDLE_OUT_EN` reader - reg_idle_out_en_ch0."]
pub struct IDLE_OUT_EN_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_EN` writer - reg_idle_out_en_ch0."]
pub struct IDLE_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_EN_W<'a> {
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
#[doc = "Field `TX_STOP` reader - reg_tx_stop_ch0."]
pub struct TX_STOP_R(crate::FieldReader<bool, bool>);
impl TX_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_STOP` writer - reg_tx_stop_ch0."]
pub struct TX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STOP_W<'a> {
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
#[doc = "Field `DIV_CNT` reader - reg_div_cnt_ch0."]
pub struct DIV_CNT_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT` writer - reg_div_cnt_ch0."]
pub struct DIV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MEM_SIZE` reader - reg_mem_size_ch0."]
pub struct MEM_SIZE_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE` writer - reg_mem_size_ch0."]
pub struct MEM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
#[doc = "Field `CARRIER_EFF_EN` reader - reg_carrier_eff_en_ch0."]
pub struct CARRIER_EFF_EN_R(crate::FieldReader<bool, bool>);
impl CARRIER_EFF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EFF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EFF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EFF_EN` writer - reg_carrier_eff_en_ch0."]
pub struct CARRIER_EFF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EFF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `CARRIER_EN` reader - reg_carrier_en_ch0."]
pub struct CARRIER_EN_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN` writer - reg_carrier_en_ch0."]
pub struct CARRIER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `CARRIER_OUT_LV` reader - reg_carrier_out_lv_ch0."]
pub struct CARRIER_OUT_LV_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV` writer - reg_carrier_out_lv_ch0."]
pub struct CARRIER_OUT_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `AFIFO_RST` writer - reg_afifo_rst_ch0."]
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `CONF_UPDATE` writer - reg_reg_conf_update_ch0."]
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - reg_tx_conti_mode_ch0."]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_mem_tx_wrap_en_ch0."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_idle_out_lv_ch0."]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_idle_out_en_ch0."]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_tx_stop_ch0."]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - reg_div_cnt_ch0."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - reg_mem_size_ch0."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - reg_carrier_eff_en_ch0."]
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_carrier_en_ch0."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_carrier_out_lv_ch0."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_tx_start_ch0."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 1 - reg_mem_rd_rst_ch0."]
    #[inline(always)]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W {
        MEM_RD_RST_W { w: self }
    }
    #[doc = "Bit 2 - reg_apb_mem_rst_ch0."]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W {
        APB_MEM_RST_W { w: self }
    }
    #[doc = "Bit 3 - reg_tx_conti_mode_ch0."]
    #[inline(always)]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W {
        TX_CONTI_MODE_W { w: self }
    }
    #[doc = "Bit 4 - reg_mem_tx_wrap_en_ch0."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W {
        MEM_TX_WRAP_EN_W { w: self }
    }
    #[doc = "Bit 5 - reg_idle_out_lv_ch0."]
    #[inline(always)]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W {
        IDLE_OUT_LV_W { w: self }
    }
    #[doc = "Bit 6 - reg_idle_out_en_ch0."]
    #[inline(always)]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W {
        IDLE_OUT_EN_W { w: self }
    }
    #[doc = "Bit 7 - reg_tx_stop_ch0."]
    #[inline(always)]
    pub fn tx_stop(&mut self) -> TX_STOP_W {
        TX_STOP_W { w: self }
    }
    #[doc = "Bits 8:15 - reg_div_cnt_ch0."]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W {
        DIV_CNT_W { w: self }
    }
    #[doc = "Bits 16:18 - reg_mem_size_ch0."]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W {
        MEM_SIZE_W { w: self }
    }
    #[doc = "Bit 20 - reg_carrier_eff_en_ch0."]
    #[inline(always)]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W {
        CARRIER_EFF_EN_W { w: self }
    }
    #[doc = "Bit 21 - reg_carrier_en_ch0."]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W {
        CARRIER_EN_W { w: self }
    }
    #[doc = "Bit 22 - reg_carrier_out_lv_ch0."]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W {
        CARRIER_OUT_LV_W { w: self }
    }
    #[doc = "Bit 23 - reg_afifo_rst_ch0."]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W {
        AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 24 - reg_reg_conf_update_ch0."]
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
#[doc = "RMT_CH%sCONF%s_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_conf0]
(index.html) module"]
pub struct CH_TX_CONF0_SPEC;
impl crate::RegisterSpec for CH_TX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_conf0::R]
(R) reader structure"]
impl crate::Readable for CH_TX_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_tx_conf0::W]
(W) writer structure"]
impl crate::Writable for CH_TX_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_TX_CONF0 to value 0x0071_0200"]
impl crate::Resettable for CH_TX_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0071_0200
    }
}
