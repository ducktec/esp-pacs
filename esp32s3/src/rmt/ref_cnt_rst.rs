#[doc = "Register `REF_CNT_RST` writer"]
pub struct W(crate::W<REF_CNT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_CNT_RST_SPEC>;
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
impl From<crate::W<REF_CNT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_CNT_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `CH(0-7)` writer - This register is used to reset the clock divider of CHANNEL0."]
pub struct CH_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CH_W<'a> {
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
    #[doc = "This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub unsafe fn ch(&mut self, n: usize) -> CH_W {
        CH_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch0(&mut self) -> CH_W {
        CH_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch1(&mut self) -> CH_W {
        CH_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch2(&mut self) -> CH_W {
        CH_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch3(&mut self) -> CH_W {
        CH_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch0(&mut self) -> CH_W {
        CH_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch1(&mut self) -> CH_W {
        CH_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch2(&mut self) -> CH_W {
        CH_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch3(&mut self) -> CH_W {
        CH_W { w: self, offset: 7 }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT clock divider reset register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cnt_rst]
(index.html) module"]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ref_cnt_rst::W]
(W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
