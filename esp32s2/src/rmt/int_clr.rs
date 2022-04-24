#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `CH(0-3)_TX_END_INT_CLR` writer - Set this bit to clear the CH%s_TX_END_INT interrupt."]
pub struct CH_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_END_INT_CLR_W<'a> {
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
#[doc = "Fields `CH(0-3)_RX_END_INT_CLR` writer - Set this bit to clear the CH%s_RX_END_INT interrupt."]
pub struct CH_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_RX_END_INT_CLR_W<'a> {
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
#[doc = "Fields `CH(0-3)_ERR_INT_CLR` writer - Set this bit to clear the CH%s_ERR_INT interrupt."]
pub struct CH_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_ERR_INT_CLR_W<'a> {
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
#[doc = "Fields `CH(0-3)_TX_THR_EVENT_INT_CLR` writer - Set this bit to clear the CH%s_TX_THR_EVENT_INT interrupt."]
pub struct CH_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_THR_EVENT_INT_CLR_W<'a> {
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
#[doc = "Fields `CH(0-3)_TX_LOOP_INT_CLR` writer - Set this bit to clear the CH%s_TX_LOOP_INT interrupt."]
pub struct CH_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_TX_LOOP_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Set this bit to clear the CH(0-3)_TX_END_INT interrupt."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_clr(&mut self, n: usize) -> CH_TX_END_INT_CLR_W {
        CH_TX_END_INT_CLR_W {
            w: self,
            offset: n * 3,
        }
    }
    #[doc = "Bit 0 - Set this bit to clear the CH0_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W {
        CH_TX_END_INT_CLR_W { w: self, offset: 0 }
    }
    #[doc = "Bit 3 - Set this bit to clear the CH1_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W {
        CH_TX_END_INT_CLR_W { w: self, offset: 3 }
    }
    #[doc = "Bit 6 - Set this bit to clear the CH2_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W {
        CH_TX_END_INT_CLR_W { w: self, offset: 6 }
    }
    #[doc = "Bit 9 - Set this bit to clear the CH3_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W {
        CH_TX_END_INT_CLR_W { w: self, offset: 9 }
    }
    #[doc = "Set this bit to clear the CH(0-3)_RX_END_INT interrupt."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_clr(&mut self, n: usize) -> CH_RX_END_INT_CLR_W {
        CH_RX_END_INT_CLR_W {
            w: self,
            offset: n * 3 + 1,
        }
    }
    #[doc = "Bit 1 - Set this bit to clear the CH0_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W {
        CH_RX_END_INT_CLR_W { w: self, offset: 1 }
    }
    #[doc = "Bit 4 - Set this bit to clear the CH1_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W {
        CH_RX_END_INT_CLR_W { w: self, offset: 4 }
    }
    #[doc = "Bit 7 - Set this bit to clear the CH2_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W {
        CH_RX_END_INT_CLR_W { w: self, offset: 7 }
    }
    #[doc = "Bit 10 - Set this bit to clear the CH3_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W {
        CH_RX_END_INT_CLR_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Set this bit to clear the CH(0-3)_ERR_INT interrupt."]
    #[inline(always)]
    pub unsafe fn ch_err_int_clr(&mut self, n: usize) -> CH_ERR_INT_CLR_W {
        CH_ERR_INT_CLR_W {
            w: self,
            offset: n * 3 + 2,
        }
    }
    #[doc = "Bit 2 - Set this bit to clear the CH0_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch0_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W {
        CH_ERR_INT_CLR_W { w: self, offset: 2 }
    }
    #[doc = "Bit 5 - Set this bit to clear the CH1_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch1_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W {
        CH_ERR_INT_CLR_W { w: self, offset: 5 }
    }
    #[doc = "Bit 8 - Set this bit to clear the CH2_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch2_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W {
        CH_ERR_INT_CLR_W { w: self, offset: 8 }
    }
    #[doc = "Bit 11 - Set this bit to clear the CH3_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch3_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W {
        CH_ERR_INT_CLR_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Set this bit to clear the CH(0-3)_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_clr(&mut self, n: usize) -> CH_TX_THR_EVENT_INT_CLR_W {
        CH_TX_THR_EVENT_INT_CLR_W {
            w: self,
            offset: n + 12,
        }
    }
    #[doc = "Bit 12 - Set this bit to clear the CH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W {
        CH_TX_THR_EVENT_INT_CLR_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Set this bit to clear the CH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W {
        CH_TX_THR_EVENT_INT_CLR_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Set this bit to clear the CH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W {
        CH_TX_THR_EVENT_INT_CLR_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Set this bit to clear the CH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W {
        CH_TX_THR_EVENT_INT_CLR_W {
            w: self,
            offset: 15,
        }
    }
    #[doc = "Set this bit to clear the CH(0-3)_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_clr(&mut self, n: usize) -> CH_TX_LOOP_INT_CLR_W {
        CH_TX_LOOP_INT_CLR_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - Set this bit to clear the CH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W {
        CH_TX_LOOP_INT_CLR_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bit 17 - Set this bit to clear the CH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W {
        CH_TX_LOOP_INT_CLR_W {
            w: self,
            offset: 17,
        }
    }
    #[doc = "Bit 18 - Set this bit to clear the CH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W {
        CH_TX_LOOP_INT_CLR_W {
            w: self,
            offset: 18,
        }
    }
    #[doc = "Bit 19 - Set this bit to clear the CH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W {
        CH_TX_LOOP_INT_CLR_W {
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
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr]
(index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W]
(W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
