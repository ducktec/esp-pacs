#[doc = "Register `SRAM_CMD` reader"]
pub struct R(crate::R<SRAM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CMD` writer"]
pub struct W(crate::W<SRAM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CMD_SPEC>;
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
impl From<crate::W<SRAM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_DIO` reader - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub struct SRAM_DIO_R(crate::FieldReader<bool, bool>);
impl SRAM_DIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_DIO` writer - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub struct SRAM_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_DIO_W<'a> {
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
#[doc = "Field `SRAM_QIO` reader - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub struct SRAM_QIO_R(crate::FieldReader<bool, bool>);
impl SRAM_QIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_QIO` writer - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub struct SRAM_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_QIO_W<'a> {
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
#[doc = "Field `SRAM_RSTIO` reader - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
pub struct SRAM_RSTIO_R(crate::FieldReader<bool, bool>);
impl SRAM_RSTIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_RSTIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_RSTIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_RSTIO` writer - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
pub struct SRAM_RSTIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_RSTIO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_dio(&self) -> SRAM_DIO_R {
        SRAM_DIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_qio(&self) -> SRAM_QIO_R {
        SRAM_QIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
    #[inline(always)]
    pub fn sram_rstio(&self) -> SRAM_RSTIO_R {
        SRAM_RSTIO_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_dio(&mut self) -> SRAM_DIO_W {
        SRAM_DIO_W { w: self }
    }
    #[doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_qio(&mut self) -> SRAM_QIO_W {
        SRAM_QIO_W { w: self }
    }
    #[doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
    #[inline(always)]
    pub fn sram_rstio(&mut self) -> SRAM_RSTIO_W {
        SRAM_RSTIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cmd]
(index.html) module"]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cmd::R]
(R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cmd::W]
(W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0"]
impl crate::Resettable for SRAM_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
