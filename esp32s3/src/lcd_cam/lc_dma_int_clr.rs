#[doc = "Register `LC_DMA_INT_CLR` writer"]
pub struct W(crate::W<LC_DMA_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_DMA_INT_CLR_SPEC>;
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
impl From<crate::W<LC_DMA_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_DMA_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_VSYNC_INT_CLR` writer - The clear bit for LCD frame end interrupt."]
pub struct LCD_VSYNC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_VSYNC_INT_CLR_W<'a> {
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
#[doc = "Field `LCD_TRANS_DONE_INT_CLR` writer - The clear bit for lcd transfer end interrupt."]
pub struct LCD_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_TRANS_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `CAM_VSYNC_INT_CLR` writer - The clear bit for Camera frame end interrupt."]
pub struct CAM_VSYNC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_VSYNC_INT_CLR_W<'a> {
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
#[doc = "Field `CAM_HS_INT_CLR` writer - The clear bit for Camera line interrupt."]
pub struct CAM_HS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAM_HS_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - The clear bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_clr(&mut self) -> LCD_VSYNC_INT_CLR_W {
        LCD_VSYNC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The clear bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_clr(&mut self) -> LCD_TRANS_DONE_INT_CLR_W {
        LCD_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - The clear bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_clr(&mut self) -> CAM_VSYNC_INT_CLR_W {
        CAM_VSYNC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - The clear bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_clr(&mut self) -> CAM_HS_INT_CLR_W {
        CAM_HS_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_camera DMA inturrupt clear register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_dma_int_clr]
(index.html) module"]
pub struct LC_DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lc_dma_int_clr::W]
(W) writer structure"]
impl crate::Writable for LC_DMA_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LC_DMA_INT_CLR to value 0"]
impl crate::Resettable for LC_DMA_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
