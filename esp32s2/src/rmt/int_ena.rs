#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `CH(0-3)_TX_END_INT_ENA` reader - The interrupt enabled bit for CH%s_TX_END_INT."]
pub struct CH_TX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH_TX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_TX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_TX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-3)_TX_END_INT_ENA` writer - The interrupt enabled bit for CH%s_TX_END_INT."]
pub struct CH_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_END_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `CH(0-3)_RX_END_INT_ENA` reader - The interrupt enabled bit for CH%s_RX_END_INT."]
pub struct CH_RX_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH_RX_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_RX_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_RX_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-3)_RX_END_INT_ENA` writer - The interrupt enabled bit for CH%s_RX_END_INT."]
pub struct CH_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_RX_END_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `CH(0-3)_ERR_INT_ENA` reader - The interrupt enabled bit for CH%s_ERR_INT."]
pub struct CH_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-3)_ERR_INT_ENA` writer - The interrupt enabled bit for CH%s_ERR_INT."]
pub struct CH_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `CH(0-3)_TX_THR_EVENT_INT_ENA` reader - The interrupt enabled bit for CH%s_TX_THR_EVENT_INT."]
pub struct CH_TX_THR_EVENT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH_TX_THR_EVENT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_TX_THR_EVENT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_TX_THR_EVENT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-3)_TX_THR_EVENT_INT_ENA` writer - The interrupt enabled bit for CH%s_TX_THR_EVENT_INT."]
pub struct CH_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `CH(0-3)_TX_LOOP_INT_ENA` reader - The interrupt enabled bit for CH%s_TX_LOOP_INT."]
pub struct CH_TX_LOOP_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CH_TX_LOOP_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_TX_LOOP_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_TX_LOOP_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CH(0-3)_TX_LOOP_INT_ENA` writer - The interrupt enabled bit for CH%s_TX_LOOP_INT."]
pub struct CH_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_ena(&self, n: usize) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_ena(&self, n: usize) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_err_int_ena(&self, n: usize) -> CH_ERR_INT_ENA_R {
        CH_ERR_INT_ENA_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&self) -> CH_ERR_INT_ENA_R {
        CH_ERR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&self) -> CH_ERR_INT_ENA_R {
        CH_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&self) -> CH_ERR_INT_ENA_R {
        CH_ERR_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&self) -> CH_ERR_INT_ENA_R {
        CH_ERR_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_ena(&self, n: usize) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_ena(&self, n: usize) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_ena(&mut self, n: usize) -> CH_TX_END_INT_ENA_W {
        CH_TX_END_INT_ENA_W {
            w: self,
            offset: n * 3,
        }
    }
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W {
        CH_TX_END_INT_ENA_W { w: self, offset: 0 }
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W {
        CH_TX_END_INT_ENA_W { w: self, offset: 3 }
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W {
        CH_TX_END_INT_ENA_W { w: self, offset: 6 }
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W {
        CH_TX_END_INT_ENA_W { w: self, offset: 9 }
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_ena(&mut self, n: usize) -> CH_RX_END_INT_ENA_W {
        CH_RX_END_INT_ENA_W {
            w: self,
            offset: n * 3 + 1,
        }
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W {
        CH_RX_END_INT_ENA_W { w: self, offset: 1 }
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W {
        CH_RX_END_INT_ENA_W { w: self, offset: 4 }
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W {
        CH_RX_END_INT_ENA_W { w: self, offset: 7 }
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W {
        CH_RX_END_INT_ENA_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_err_int_ena(&mut self, n: usize) -> CH_ERR_INT_ENA_W {
        CH_ERR_INT_ENA_W {
            w: self,
            offset: n * 3 + 2,
        }
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err_int_ena(&mut self) -> CH_ERR_INT_ENA_W {
        CH_ERR_INT_ENA_W { w: self, offset: 2 }
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err_int_ena(&mut self) -> CH_ERR_INT_ENA_W {
        CH_ERR_INT_ENA_W { w: self, offset: 5 }
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err_int_ena(&mut self) -> CH_ERR_INT_ENA_W {
        CH_ERR_INT_ENA_W { w: self, offset: 8 }
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err_int_ena(&mut self) -> CH_ERR_INT_ENA_W {
        CH_ERR_INT_ENA_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_ena(&mut self, n: usize) -> CH_TX_THR_EVENT_INT_ENA_W {
        CH_TX_THR_EVENT_INT_ENA_W {
            w: self,
            offset: n + 12,
        }
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W {
        CH_TX_THR_EVENT_INT_ENA_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W {
        CH_TX_THR_EVENT_INT_ENA_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W {
        CH_TX_THR_EVENT_INT_ENA_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W {
        CH_TX_THR_EVENT_INT_ENA_W {
            w: self,
            offset: 15,
        }
    }
    #[doc = "The interrupt enabled bit for CH(0-3)_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_ena(&mut self, n: usize) -> CH_TX_LOOP_INT_ENA_W {
        CH_TX_LOOP_INT_ENA_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W {
        CH_TX_LOOP_INT_ENA_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W {
        CH_TX_LOOP_INT_ENA_W {
            w: self,
            offset: 17,
        }
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W {
        CH_TX_LOOP_INT_ENA_W {
            w: self,
            offset: 18,
        }
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W {
        CH_TX_LOOP_INT_ENA_W {
            w: self,
            offset: 19,
        }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena]
(index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R]
(R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W]
(W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
