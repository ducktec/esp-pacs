#[doc = "Register `SHROM_MPU_TABLE22` reader"]
pub struct R(crate::R<SHROM_MPU_TABLE22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHROM_MPU_TABLE22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHROM_MPU_TABLE22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHROM_MPU_TABLE22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHROM_MPU_TABLE22` writer"]
pub struct W(crate::W<SHROM_MPU_TABLE22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHROM_MPU_TABLE22_SPEC>;
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
impl From<crate::W<SHROM_MPU_TABLE22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHROM_MPU_TABLE22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHROM_MPU_TABLE22` reader - "]
pub struct SHROM_MPU_TABLE22_R(crate::FieldReader<u8, u8>);
impl SHROM_MPU_TABLE22_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHROM_MPU_TABLE22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHROM_MPU_TABLE22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHROM_MPU_TABLE22` writer - "]
pub struct SHROM_MPU_TABLE22_W<'a> {
    w: &'a mut W,
}
impl<'a> SHROM_MPU_TABLE22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table22(&self) -> SHROM_MPU_TABLE22_R {
        SHROM_MPU_TABLE22_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table22(&mut self) -> SHROM_MPU_TABLE22_W {
        SHROM_MPU_TABLE22_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shrom_mpu_table22]
(index.html) module"]
pub struct SHROM_MPU_TABLE22_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shrom_mpu_table22::R]
(R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table22::W]
(W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE22_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHROM_MPU_TABLE22 to value 0x01"]
impl crate::Resettable for SHROM_MPU_TABLE22_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
