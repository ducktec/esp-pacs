#[doc = "Register `PRO_CACHE_WRAP_AROUND_CTRL` reader"]
pub struct R(crate::R<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_WRAP_AROUND_CTRL` writer"]
pub struct W(crate::W<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>;
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
impl From<crate::W<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_WRAP_AROUND_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_FLASH_WRAP_AROUND` reader - The bit is used to enable wrap around mode when read data from flash."]
pub struct PRO_CACHE_FLASH_WRAP_AROUND_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_FLASH_WRAP_AROUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_FLASH_WRAP_AROUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_FLASH_WRAP_AROUND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_FLASH_WRAP_AROUND` writer - The bit is used to enable wrap around mode when read data from flash."]
pub struct PRO_CACHE_FLASH_WRAP_AROUND_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_FLASH_WRAP_AROUND_W<'a> {
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
#[doc = "Field `PRO_CACHE_SRAM_RD_WRAP_AROUND` reader - The bit is used to enable wrap around mode when read data from spiram."]
pub struct PRO_CACHE_SRAM_RD_WRAP_AROUND_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_SRAM_RD_WRAP_AROUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_SRAM_RD_WRAP_AROUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_SRAM_RD_WRAP_AROUND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_SRAM_RD_WRAP_AROUND` writer - The bit is used to enable wrap around mode when read data from spiram."]
pub struct PRO_CACHE_SRAM_RD_WRAP_AROUND_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_SRAM_RD_WRAP_AROUND_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The bit is used to enable wrap around mode when read data from flash."]
    #[inline(always)]
    pub fn pro_cache_flash_wrap_around(&self) -> PRO_CACHE_FLASH_WRAP_AROUND_R {
        PRO_CACHE_FLASH_WRAP_AROUND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable wrap around mode when read data from spiram."]
    #[inline(always)]
    pub fn pro_cache_sram_rd_wrap_around(&self) -> PRO_CACHE_SRAM_RD_WRAP_AROUND_R {
        PRO_CACHE_SRAM_RD_WRAP_AROUND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable wrap around mode when read data from flash."]
    #[inline(always)]
    pub fn pro_cache_flash_wrap_around(&mut self) -> PRO_CACHE_FLASH_WRAP_AROUND_W {
        PRO_CACHE_FLASH_WRAP_AROUND_W { w: self }
    }
    #[doc = "Bit 1 - The bit is used to enable wrap around mode when read data from spiram."]
    #[inline(always)]
    pub fn pro_cache_sram_rd_wrap_around(&mut self) -> PRO_CACHE_SRAM_RD_WRAP_AROUND_W {
        PRO_CACHE_SRAM_RD_WRAP_AROUND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_wrap_around_ctrl]
(index.html) module"]
pub struct PRO_CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_wrap_around_ctrl::R]
(R) reader structure"]
impl crate::Readable for PRO_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_wrap_around_ctrl::W]
(W) writer structure"]
impl crate::Writable for PRO_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for PRO_CACHE_WRAP_AROUND_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
