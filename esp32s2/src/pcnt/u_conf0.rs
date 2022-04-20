#[doc = "Register `U%s_CONF0` reader"]
pub struct R(crate::R<U_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U%s_CONF0` writer"]
pub struct W(crate::W<U_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U_CONF0_SPEC>;
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
impl From<crate::W<U_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_THRES_U0` reader - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub struct FILTER_THRES_U0_R(crate::FieldReader<u16, u16>);
impl FILTER_THRES_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FILTER_THRES_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_THRES_U0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_THRES_U0` writer - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub struct FILTER_THRES_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_THRES_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `FILTER_EN_U0` reader - This is the enable bit for unit %s's input filter."]
pub struct FILTER_EN_U0_R(crate::FieldReader<bool, bool>);
impl FILTER_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_EN_U0` writer - This is the enable bit for unit %s's input filter."]
pub struct FILTER_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `THR_ZERO_EN_U0` reader - This is the enable bit for unit %s's zero comparator."]
pub struct THR_ZERO_EN_U0_R(crate::FieldReader<bool, bool>);
impl THR_ZERO_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_ZERO_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_ZERO_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_ZERO_EN_U0` writer - This is the enable bit for unit %s's zero comparator."]
pub struct THR_ZERO_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_ZERO_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `THR_H_LIM_EN_U0` reader - This is the enable bit for unit %s's thr_h_lim comparator."]
pub struct THR_H_LIM_EN_U0_R(crate::FieldReader<bool, bool>);
impl THR_H_LIM_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_H_LIM_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_H_LIM_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_H_LIM_EN_U0` writer - This is the enable bit for unit %s's thr_h_lim comparator."]
pub struct THR_H_LIM_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_H_LIM_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `THR_L_LIM_EN_U0` reader - This is the enable bit for unit %s's thr_l_lim comparator."]
pub struct THR_L_LIM_EN_U0_R(crate::FieldReader<bool, bool>);
impl THR_L_LIM_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_L_LIM_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_L_LIM_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_L_LIM_EN_U0` writer - This is the enable bit for unit %s's thr_l_lim comparator."]
pub struct THR_L_LIM_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_L_LIM_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `THR_THRES0_EN_U0` reader - This is the enable bit for unit %s's thres0 comparator."]
pub struct THR_THRES0_EN_U0_R(crate::FieldReader<bool, bool>);
impl THR_THRES0_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_THRES0_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_THRES0_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_THRES0_EN_U0` writer - This is the enable bit for unit %s's thres0 comparator."]
pub struct THR_THRES0_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_THRES0_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `THR_THRES1_EN_U0` reader - This is the enable bit for unit %s's thres1 comparator."]
pub struct THR_THRES1_EN_U0_R(crate::FieldReader<bool, bool>);
impl THR_THRES1_EN_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_THRES1_EN_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_THRES1_EN_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_THRES1_EN_U0` writer - This is the enable bit for unit %s's thres1 comparator."]
pub struct THR_THRES1_EN_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_THRES1_EN_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `CH0_NEG_MODE_U0` reader - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
pub struct CH0_NEG_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH0_NEG_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_NEG_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_NEG_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_NEG_MODE_U0` writer - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
pub struct CH0_NEG_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_NEG_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `CH0_POS_MODE_U0` reader - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
pub struct CH0_POS_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH0_POS_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_POS_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_POS_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_POS_MODE_U0` writer - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
pub struct CH0_POS_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_POS_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `CH0_HCTRL_MODE_U0` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH0_HCTRL_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH0_HCTRL_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_HCTRL_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_HCTRL_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_HCTRL_MODE_U0` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH0_HCTRL_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_HCTRL_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `CH0_LCTRL_MODE_U0` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH0_LCTRL_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH0_LCTRL_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_LCTRL_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_LCTRL_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_LCTRL_MODE_U0` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH0_LCTRL_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_LCTRL_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `CH1_NEG_MODE_U0` reader - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
pub struct CH1_NEG_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH1_NEG_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_NEG_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_NEG_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_NEG_MODE_U0` writer - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
pub struct CH1_NEG_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_NEG_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `CH1_POS_MODE_U0` reader - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
pub struct CH1_POS_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH1_POS_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_POS_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_POS_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_POS_MODE_U0` writer - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
pub struct CH1_POS_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_POS_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `CH1_HCTRL_MODE_U0` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH1_HCTRL_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH1_HCTRL_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_HCTRL_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_HCTRL_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_HCTRL_MODE_U0` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH1_HCTRL_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_HCTRL_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `CH1_LCTRL_MODE_U0` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH1_LCTRL_MODE_U0_R(crate::FieldReader<u8, u8>);
impl CH1_LCTRL_MODE_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_LCTRL_MODE_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_LCTRL_MODE_U0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_LCTRL_MODE_U0` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
pub struct CH1_LCTRL_MODE_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_LCTRL_MODE_U0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres_u0(&self) -> FILTER_THRES_U0_R {
        FILTER_THRES_U0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u0(&self) -> FILTER_EN_U0_R {
        FILTER_EN_U0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u0(&self) -> THR_ZERO_EN_U0_R {
        THR_ZERO_EN_U0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator."]
    #[inline(always)]
    pub fn thr_h_lim_en_u0(&self) -> THR_H_LIM_EN_U0_R {
        THR_H_LIM_EN_U0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator."]
    #[inline(always)]
    pub fn thr_l_lim_en_u0(&self) -> THR_L_LIM_EN_U0_R {
        THR_L_LIM_EN_U0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u0(&self) -> THR_THRES0_EN_U0_R {
        THR_THRES0_EN_U0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u0(&self) -> THR_THRES1_EN_U0_R {
        THR_THRES1_EN_U0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch0_neg_mode_u0(&self) -> CH0_NEG_MODE_U0_R {
        CH0_NEG_MODE_U0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch0_pos_mode_u0(&self) -> CH0_POS_MODE_U0_R {
        CH0_POS_MODE_U0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u0(&self) -> CH0_HCTRL_MODE_U0_R {
        CH0_HCTRL_MODE_U0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u0(&self) -> CH0_LCTRL_MODE_U0_R {
        CH0_LCTRL_MODE_U0_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch1_neg_mode_u0(&self) -> CH1_NEG_MODE_U0_R {
        CH1_NEG_MODE_U0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch1_pos_mode_u0(&self) -> CH1_POS_MODE_U0_R {
        CH1_POS_MODE_U0_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u0(&self) -> CH1_HCTRL_MODE_U0_R {
        CH1_HCTRL_MODE_U0_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u0(&self) -> CH1_LCTRL_MODE_U0_R {
        CH1_LCTRL_MODE_U0_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres_u0(&mut self) -> FILTER_THRES_U0_W {
        FILTER_THRES_U0_W { w: self }
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u0(&mut self) -> FILTER_EN_U0_W {
        FILTER_EN_U0_W { w: self }
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u0(&mut self) -> THR_ZERO_EN_U0_W {
        THR_ZERO_EN_U0_W { w: self }
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator."]
    #[inline(always)]
    pub fn thr_h_lim_en_u0(&mut self) -> THR_H_LIM_EN_U0_W {
        THR_H_LIM_EN_U0_W { w: self }
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator."]
    #[inline(always)]
    pub fn thr_l_lim_en_u0(&mut self) -> THR_L_LIM_EN_U0_W {
        THR_L_LIM_EN_U0_W { w: self }
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u0(&mut self) -> THR_THRES0_EN_U0_W {
        THR_THRES0_EN_U0_W { w: self }
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u0(&mut self) -> THR_THRES1_EN_U0_W {
        THR_THRES1_EN_U0_W { w: self }
    }
    #[doc = "Bits 16:17 - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch0_neg_mode_u0(&mut self) -> CH0_NEG_MODE_U0_W {
        CH0_NEG_MODE_U0_W { w: self }
    }
    #[doc = "Bits 18:19 - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter. 2: Decrease the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch0_pos_mode_u0(&mut self) -> CH0_POS_MODE_U0_W {
        CH0_POS_MODE_U0_W { w: self }
    }
    #[doc = "Bits 20:21 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u0(&mut self) -> CH0_HCTRL_MODE_U0_W {
        CH0_HCTRL_MODE_U0_W { w: self }
    }
    #[doc = "Bits 22:23 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u0(&mut self) -> CH0_LCTRL_MODE_U0_W {
        CH0_LCTRL_MODE_U0_W { w: self }
    }
    #[doc = "Bits 24:25 - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch1_neg_mode_u0(&mut self) -> CH1_NEG_MODE_U0_W {
        CH1_NEG_MODE_U0_W { w: self }
    }
    #[doc = "Bits 26:27 - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter. 2: Decrement the counter. 0, 3: No effect on counter."]
    #[inline(always)]
    pub fn ch1_pos_mode_u0(&mut self) -> CH1_POS_MODE_U0_W {
        CH1_POS_MODE_U0_W { w: self }
    }
    #[doc = "Bits 28:29 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u0(&mut self) -> CH1_HCTRL_MODE_U0_W {
        CH1_HCTRL_MODE_U0_W { w: self }
    }
    #[doc = "Bits 30:31 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification. 1: Invert behavior (increase -> decrease, decrease -> increase). 2, 3: Inhibit counter modification."]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u0(&mut self) -> CH1_LCTRL_MODE_U0_W {
        CH1_LCTRL_MODE_U0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 0 for unit %s\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_conf0]
(index.html) module"]
pub struct U_CONF0_SPEC;
impl crate::RegisterSpec for U_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_conf0::R]
(R) reader structure"]
impl crate::Readable for U_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u_conf0::W]
(W) writer structure"]
impl crate::Writable for U_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U%s_CONF0 to value 0x3c10"]
impl crate::Resettable for U_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c10
    }
}
