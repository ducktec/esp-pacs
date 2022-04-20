#[doc = "Register `CACHE_SYNC_INT_CTRL` reader"]
pub struct R(crate::R<CACHE_SYNC_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SYNC_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SYNC_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SYNC_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SYNC_INT_CTRL` writer"]
pub struct W(crate::W<CACHE_SYNC_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SYNC_INT_CTRL_SPEC>;
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
impl From<crate::W<CACHE_SYNC_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SYNC_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_SYNC_INT_ST` reader - The bit is used to indicate the interrupt by icache sync done."]
pub struct PRO_ICACHE_SYNC_INT_ST_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_SYNC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_SYNC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_SYNC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_SYNC_INT_ENA` reader - The bit is used to enable the interrupt by icache sync done."]
pub struct PRO_ICACHE_SYNC_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_ICACHE_SYNC_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ICACHE_SYNC_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ICACHE_SYNC_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ICACHE_SYNC_INT_ENA` writer - The bit is used to enable the interrupt by icache sync done."]
pub struct PRO_ICACHE_SYNC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_SYNC_INT_ENA_W<'a> {
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
#[doc = "Field `PRO_ICACHE_SYNC_INT_CLR` writer - The bit is used to clear the interrupt by icache sync done."]
pub struct PRO_ICACHE_SYNC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ICACHE_SYNC_INT_CLR_W<'a> {
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
#[doc = "Field `PRO_DCACHE_SYNC_INT_ST` reader - The bit is used to indicate the interrupt by dcache sync done."]
pub struct PRO_DCACHE_SYNC_INT_ST_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_SYNC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_SYNC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_SYNC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_SYNC_INT_ENA` reader - The bit is used to enable the interrupt by dcache sync done."]
pub struct PRO_DCACHE_SYNC_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_DCACHE_SYNC_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DCACHE_SYNC_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DCACHE_SYNC_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DCACHE_SYNC_INT_ENA` writer - The bit is used to enable the interrupt by dcache sync done."]
pub struct PRO_DCACHE_SYNC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_SYNC_INT_ENA_W<'a> {
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
#[doc = "Field `PRO_DCACHE_SYNC_INT_CLR` writer - The bit is used to clear the interrupt by dcache sync done."]
pub struct PRO_DCACHE_SYNC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DCACHE_SYNC_INT_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The bit is used to indicate the interrupt by icache sync done."]
    #[inline(always)]
    pub fn pro_icache_sync_int_st(&self) -> PRO_ICACHE_SYNC_INT_ST_R {
        PRO_ICACHE_SYNC_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache sync done."]
    #[inline(always)]
    pub fn pro_icache_sync_int_ena(&self) -> PRO_ICACHE_SYNC_INT_ENA_R {
        PRO_ICACHE_SYNC_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate the interrupt by dcache sync done."]
    #[inline(always)]
    pub fn pro_dcache_sync_int_st(&self) -> PRO_DCACHE_SYNC_INT_ST_R {
        PRO_DCACHE_SYNC_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache sync done."]
    #[inline(always)]
    pub fn pro_dcache_sync_int_ena(&self) -> PRO_DCACHE_SYNC_INT_ENA_R {
        PRO_DCACHE_SYNC_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache sync done."]
    #[inline(always)]
    pub fn pro_icache_sync_int_ena(&mut self) -> PRO_ICACHE_SYNC_INT_ENA_W {
        PRO_ICACHE_SYNC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to clear the interrupt by icache sync done."]
    #[inline(always)]
    pub fn pro_icache_sync_int_clr(&mut self) -> PRO_ICACHE_SYNC_INT_CLR_W {
        PRO_ICACHE_SYNC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache sync done."]
    #[inline(always)]
    pub fn pro_dcache_sync_int_ena(&mut self) -> PRO_DCACHE_SYNC_INT_ENA_W {
        PRO_DCACHE_SYNC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The bit is used to clear the interrupt by dcache sync done."]
    #[inline(always)]
    pub fn pro_dcache_sync_int_clr(&mut self) -> PRO_DCACHE_SYNC_INT_CLR_W {
        PRO_DCACHE_SYNC_INT_CLR_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_sync_int_ctrl]
(index.html) module"]
pub struct CACHE_SYNC_INT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_sync_int_ctrl::R]
(R) reader structure"]
impl crate::Readable for CACHE_SYNC_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_sync_int_ctrl::W]
(W) writer structure"]
impl crate::Writable for CACHE_SYNC_INT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_SYNC_INT_CTRL to value 0"]
impl crate::Resettable for CACHE_SYNC_INT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
