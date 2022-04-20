#[doc = "Register `CONTINUE` writer"]
pub struct W(crate::W<CONTINUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTINUE_SPEC>;
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
impl From<crate::W<CONTINUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTINUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUE` writer - Set this bit to continue GCM operation."]
pub struct CONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Set this bit to continue GCM operation."]
    #[inline(always)]
    pub fn continue_(&mut self) -> CONTINUE_W {
        CONTINUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES continue register\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continue_]
(index.html) module"]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [continue_::W]
(W) writer structure"]
impl crate::Writable for CONTINUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
