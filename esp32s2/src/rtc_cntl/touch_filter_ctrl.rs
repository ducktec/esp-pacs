#[doc = "Register `TOUCH_FILTER_CTRL` reader"]
pub struct R(crate::R<TOUCH_FILTER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_FILTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_FILTER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_FILTER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_FILTER_CTRL` writer"]
pub struct W(crate::W<TOUCH_FILTER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_FILTER_CTRL_SPEC>;
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
impl From<crate::W<TOUCH_FILTER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_FILTER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_SMOOTH_LVL` reader - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8."]
pub struct TOUCH_SMOOTH_LVL_R(crate::FieldReader<u8, u8>);
impl TOUCH_SMOOTH_LVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_SMOOTH_LVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SMOOTH_LVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SMOOTH_LVL` writer - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8."]
pub struct TOUCH_SMOOTH_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SMOOTH_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u32 & 3) << 9);
        self.w
    }
}
#[doc = "Field `TOUCH_JITTER_STEP` reader - Touch jitter step. Range: 0 – 15."]
pub struct TOUCH_JITTER_STEP_R(crate::FieldReader<u8, u8>);
impl TOUCH_JITTER_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_JITTER_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_JITTER_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_JITTER_STEP` writer - Touch jitter step. Range: 0 – 15."]
pub struct TOUCH_JITTER_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_JITTER_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` reader - Negative threshold counter limit."]
pub struct TOUCH_NEG_NOISE_LIMIT_R(crate::FieldReader<u8, u8>);
impl TOUCH_NEG_NOISE_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_NEG_NOISE_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_NEG_NOISE_LIMIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` writer - Negative threshold counter limit."]
pub struct TOUCH_NEG_NOISE_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_NEG_NOISE_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | ((value as u32 & 0x0f) << 15);
        self.w
    }
}
#[doc = "Field `TOUCH_NEG_NOISE_THRES` reader - Negative noise threshold."]
pub struct TOUCH_NEG_NOISE_THRES_R(crate::FieldReader<u8, u8>);
impl TOUCH_NEG_NOISE_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_NEG_NOISE_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_NEG_NOISE_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_NEG_NOISE_THRES` writer - Negative noise threshold."]
pub struct TOUCH_NEG_NOISE_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_NEG_NOISE_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 19)) | ((value as u32 & 3) << 19);
        self.w
    }
}
#[doc = "Field `TOUCH_NOISE_THRES` reader - Active noise threshold."]
pub struct TOUCH_NOISE_THRES_R(crate::FieldReader<u8, u8>);
impl TOUCH_NOISE_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_NOISE_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_NOISE_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_NOISE_THRES` writer - Active noise threshold."]
pub struct TOUCH_NOISE_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_NOISE_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 21)) | ((value as u32 & 3) << 21);
        self.w
    }
}
#[doc = "Field `TOUCH_HYSTERESIS` reader - Touch hysteresis."]
pub struct TOUCH_HYSTERESIS_R(crate::FieldReader<u8, u8>);
impl TOUCH_HYSTERESIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_HYSTERESIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_HYSTERESIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_HYSTERESIS` writer - Touch hysteresis."]
pub struct TOUCH_HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_HYSTERESIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 23)) | ((value as u32 & 3) << 23);
        self.w
    }
}
#[doc = "Field `TOUCH_DEBOUNCE` reader - Debounce counter."]
pub struct TOUCH_DEBOUNCE_R(crate::FieldReader<u8, u8>);
impl TOUCH_DEBOUNCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DEBOUNCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DEBOUNCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DEBOUNCE` writer - Debounce counter."]
pub struct TOUCH_DEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DEBOUNCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 25)) | ((value as u32 & 7) << 25);
        self.w
    }
}
#[doc = "Field `TOUCH_FILTER_MODE` reader - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter."]
pub struct TOUCH_FILTER_MODE_R(crate::FieldReader<u8, u8>);
impl TOUCH_FILTER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_FILTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_FILTER_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_FILTER_MODE` writer - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter."]
pub struct TOUCH_FILTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_FILTER_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 28)) | ((value as u32 & 7) << 28);
        self.w
    }
}
#[doc = "Field `TOUCH_FILTER_EN` reader - Enable touch filter."]
pub struct TOUCH_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_FILTER_EN` writer - Enable touch filter."]
pub struct TOUCH_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_FILTER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:10 - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8."]
    #[inline(always)]
    pub fn touch_smooth_lvl(&self) -> TOUCH_SMOOTH_LVL_R {
        TOUCH_SMOOTH_LVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Touch jitter step. Range: 0 – 15."]
    #[inline(always)]
    pub fn touch_jitter_step(&self) -> TOUCH_JITTER_STEP_R {
        TOUCH_JITTER_STEP_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - Negative threshold counter limit."]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&self) -> TOUCH_NEG_NOISE_LIMIT_R {
        TOUCH_NEG_NOISE_LIMIT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:20 - Negative noise threshold."]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&self) -> TOUCH_NEG_NOISE_THRES_R {
        TOUCH_NEG_NOISE_THRES_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - Active noise threshold."]
    #[inline(always)]
    pub fn touch_noise_thres(&self) -> TOUCH_NOISE_THRES_R {
        TOUCH_NOISE_THRES_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Touch hysteresis."]
    #[inline(always)]
    pub fn touch_hysteresis(&self) -> TOUCH_HYSTERESIS_R {
        TOUCH_HYSTERESIS_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Debounce counter."]
    #[inline(always)]
    pub fn touch_debounce(&self) -> TOUCH_DEBOUNCE_R {
        TOUCH_DEBOUNCE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter."]
    #[inline(always)]
    pub fn touch_filter_mode(&self) -> TOUCH_FILTER_MODE_R {
        TOUCH_FILTER_MODE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Enable touch filter."]
    #[inline(always)]
    pub fn touch_filter_en(&self) -> TOUCH_FILTER_EN_R {
        TOUCH_FILTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 9:10 - 0: Raw data. 1: IIR1/2. 2: IIR1/4. 3: IIR1/8."]
    #[inline(always)]
    pub fn touch_smooth_lvl(&mut self) -> TOUCH_SMOOTH_LVL_W {
        TOUCH_SMOOTH_LVL_W { w: self }
    }
    #[doc = "Bits 11:14 - Touch jitter step. Range: 0 – 15."]
    #[inline(always)]
    pub fn touch_jitter_step(&mut self) -> TOUCH_JITTER_STEP_W {
        TOUCH_JITTER_STEP_W { w: self }
    }
    #[doc = "Bits 15:18 - Negative threshold counter limit."]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&mut self) -> TOUCH_NEG_NOISE_LIMIT_W {
        TOUCH_NEG_NOISE_LIMIT_W { w: self }
    }
    #[doc = "Bits 19:20 - Negative noise threshold."]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&mut self) -> TOUCH_NEG_NOISE_THRES_W {
        TOUCH_NEG_NOISE_THRES_W { w: self }
    }
    #[doc = "Bits 21:22 - Active noise threshold."]
    #[inline(always)]
    pub fn touch_noise_thres(&mut self) -> TOUCH_NOISE_THRES_W {
        TOUCH_NOISE_THRES_W { w: self }
    }
    #[doc = "Bits 23:24 - Touch hysteresis."]
    #[inline(always)]
    pub fn touch_hysteresis(&mut self) -> TOUCH_HYSTERESIS_W {
        TOUCH_HYSTERESIS_W { w: self }
    }
    #[doc = "Bits 25:27 - Debounce counter."]
    #[inline(always)]
    pub fn touch_debounce(&mut self) -> TOUCH_DEBOUNCE_W {
        TOUCH_DEBOUNCE_W { w: self }
    }
    #[doc = "Bits 28:30 - Set filter mode. 0: IIR 1/2; 1: IIR 1/4; 2: IIR 1/8; 3: IIR 1/16; 4: IIR 1/32; 5: IIR 1/64; 6: IIR 1/128; 7: Jitter."]
    #[inline(always)]
    pub fn touch_filter_mode(&mut self) -> TOUCH_FILTER_MODE_W {
        TOUCH_FILTER_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Enable touch filter."]
    #[inline(always)]
    pub fn touch_filter_en(&mut self) -> TOUCH_FILTER_EN_W {
        TOUCH_FILTER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure touch filter settings\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_filter_ctrl]
(index.html) module"]
pub struct TOUCH_FILTER_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_filter_ctrl::R]
(R) reader structure"]
impl crate::Readable for TOUCH_FILTER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_filter_ctrl::W]
(W) writer structure"]
impl crate::Writable for TOUCH_FILTER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_FILTER_CTRL to value 0x96aa_8800"]
impl crate::Resettable for TOUCH_FILTER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x96aa_8800
    }
}
