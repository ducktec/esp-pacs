#[doc = "Register `CH%sCONF1` reader"]
pub struct R(crate::R<CHCONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sCONF1` writer"]
pub struct W(crate::W<CHCONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCONF1_SPEC>;
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
impl From<crate::W<CHCONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START` reader - Set this bit to start sending data on CHANNEL%s."]
pub struct TX_START_R(crate::FieldReader<bool, bool>);
impl TX_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START` writer - Set this bit to start sending data on CHANNEL%s."]
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
#[doc = "Field `RX_EN` reader - Set this bit to enable receiver to receive data on CHANNEL%s."]
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
#[doc = "Field `RX_EN` writer - Set this bit to enable receiver to receive data on CHANNEL%s."]
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `MEM_WR_RST` writer - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `MEM_RD_RST` writer - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `APB_MEM_RST` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `MEM_OWNER` reader - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
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
#[doc = "Field `MEM_OWNER` writer - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `TX_CONTI_MODE` reader - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
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
#[doc = "Field `TX_CONTI_MODE` writer - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RX_FILTER_EN` reader - This is the receive filter's enable bit for CHANNEL%s."]
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
#[doc = "Field `RX_FILTER_EN` writer - This is the receive filter's enable bit for CHANNEL%s."]
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RX_FILTER_THRES` reader - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
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
#[doc = "Field `RX_FILTER_THRES` writer - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub struct RX_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CHK_RX_CARRIER_EN` reader - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
pub struct CHK_RX_CARRIER_EN_R(crate::FieldReader<bool, bool>);
impl CHK_RX_CARRIER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHK_RX_CARRIER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHK_RX_CARRIER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHK_RX_CARRIER_EN` writer - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
pub struct CHK_RX_CARRIER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHK_RX_CARRIER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `REF_ALWAYS_ON` reader - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
pub struct REF_ALWAYS_ON_R(crate::FieldReader<bool, bool>);
impl REF_ALWAYS_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REF_ALWAYS_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_ALWAYS_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_ALWAYS_ON` writer - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
pub struct REF_ALWAYS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_ALWAYS_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_LV` reader - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
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
#[doc = "Field `IDLE_OUT_LV` writer - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_EN` reader - This is the output enable-control bit for CHANNEL%s in IDLE state."]
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
#[doc = "Field `IDLE_OUT_EN` writer - This is the output enable-control bit for CHANNEL%s in IDLE state."]
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `TX_STOP` reader - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
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
#[doc = "Field `TX_STOP` writer - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
    #[inline(always)]
    pub fn chk_rx_carrier_en(&self) -> CHK_RX_CARRIER_EN_R {
        CHK_RX_CARRIER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
    #[inline(always)]
    pub fn ref_always_on(&self) -> REF_ALWAYS_ON_R {
        REF_ALWAYS_ON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W {
        RX_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
    #[inline(always)]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W {
        MEM_WR_RST_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
    #[inline(always)]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W {
        MEM_RD_RST_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W {
        APB_MEM_RST_W { w: self }
    }
    #[doc = "Bit 5 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
    #[inline(always)]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W {
        MEM_OWNER_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W {
        TX_CONTI_MODE_W { w: self }
    }
    #[doc = "Bit 7 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W {
        RX_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W {
        RX_FILTER_THRES_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
    #[inline(always)]
    pub fn chk_rx_carrier_en(&mut self) -> CHK_RX_CARRIER_EN_W {
        CHK_RX_CARRIER_EN_W { w: self }
    }
    #[doc = "Bit 17 - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
    #[inline(always)]
    pub fn ref_always_on(&mut self) -> REF_ALWAYS_ON_W {
        REF_ALWAYS_ON_W { w: self }
    }
    #[doc = "Bit 18 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W {
        IDLE_OUT_LV_W { w: self }
    }
    #[doc = "Bit 19 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W {
        IDLE_OUT_EN_W { w: self }
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop(&mut self) -> TX_STOP_W {
        TX_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 1\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chconf1]
(index.html) module"]
pub struct CHCONF1_SPEC;
impl crate::RegisterSpec for CHCONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chconf1::R]
(R) reader structure"]
impl crate::Readable for CHCONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chconf1::W]
(W) writer structure"]
impl crate::Writable for CHCONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%sCONF1 to value 0x0f20"]
impl crate::Resettable for CHCONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f20
    }
}
