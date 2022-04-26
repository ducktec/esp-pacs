#[doc = "Register `CH5CARRIER_DUTY` reader"]
pub struct R(crate::R<CH5CARRIER_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5CARRIER_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5CARRIER_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5CARRIER_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5CARRIER_DUTY` writer"]
pub struct W(crate::W<CH5CARRIER_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5CARRIER_DUTY_SPEC>;
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
impl From<crate::W<CH5CARRIER_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5CARRIER_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_LOW` reader - This register is used to configure carrier wave's low level value for channel5."]
pub struct CARRIER_LOW_R(crate::FieldReader<u16, u16>);
impl CARRIER_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CARRIER_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_LOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_LOW` writer - This register is used to configure carrier wave's low level value for channel5."]
pub struct CARRIER_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CARRIER_HIGH` reader - This register is used to configure carrier wave's high level value for channel5."]
pub struct CARRIER_HIGH_R(crate::FieldReader<u16, u16>);
impl CARRIER_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CARRIER_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_HIGH` writer - This register is used to configure carrier wave's high level value for channel5."]
pub struct CARRIER_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel5."]
    #[inline(always)]
    pub fn carrier_low(&self) -> CARRIER_LOW_R {
        CARRIER_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel5."]
    #[inline(always)]
    pub fn carrier_high(&self) -> CARRIER_HIGH_R {
        CARRIER_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure carrier wave's low level value for channel5."]
    #[inline(always)]
    pub fn carrier_low(&mut self) -> CARRIER_LOW_W {
        CARRIER_LOW_W { w: self }
    }
    #[doc = "Bits 16:31 - This register is used to configure carrier wave's high level value for channel5."]
    #[inline(always)]
    pub fn carrier_high(&mut self) -> CARRIER_HIGH_W {
        CARRIER_HIGH_W { w: self }
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
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5carrier_duty]
(index.html) module"]
pub struct CH5CARRIER_DUTY_SPEC;
impl crate::RegisterSpec for CH5CARRIER_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5carrier_duty::R]
(R) reader structure"]
impl crate::Readable for CH5CARRIER_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5carrier_duty::W]
(W) writer structure"]
impl crate::Writable for CH5CARRIER_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH5CARRIER_DUTY to value 0x0040_0040"]
impl crate::Resettable for CH5CARRIER_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0040
    }
}
