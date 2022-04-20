#[doc = "Register `TOUCH_CTRL2` reader"]
pub struct R(crate::R<TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CTRL2` writer"]
pub struct W(crate::W<TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CTRL2_SPEC>;
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
impl From<crate::W<TOUCH_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_DRANGE` reader - TOUCH_DRANGE"]
pub struct TOUCH_DRANGE_R(crate::FieldReader<u8, u8>);
impl TOUCH_DRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DRANGE` writer - TOUCH_DRANGE"]
pub struct TOUCH_DRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `TOUCH_DREFL` reader - TOUCH_DREFL"]
pub struct TOUCH_DREFL_R(crate::FieldReader<u8, u8>);
impl TOUCH_DREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DREFL` writer - TOUCH_DREFL"]
pub struct TOUCH_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `TOUCH_DREFH` reader - TOUCH_DREFH"]
pub struct TOUCH_DREFH_R(crate::FieldReader<u8, u8>);
impl TOUCH_DREFH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DREFH` writer - TOUCH_DREFH"]
pub struct TOUCH_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `TOUCH_XPD_BIAS` reader - TOUCH_XPD_BIAS"]
pub struct TOUCH_XPD_BIAS_R(crate::FieldReader<bool, bool>);
impl TOUCH_XPD_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_XPD_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_XPD_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_XPD_BIAS` writer - TOUCH_XPD_BIAS"]
pub struct TOUCH_XPD_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_XPD_BIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `TOUCH_REFC` reader - TOUCH pad0 reference cap"]
pub struct TOUCH_REFC_R(crate::FieldReader<u8, u8>);
impl TOUCH_REFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_REFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_REFC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_REFC` writer - TOUCH pad0 reference cap"]
pub struct TOUCH_REFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_REFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 9)) | ((value as u32 & 7) << 9);
        self.w
    }
}
#[doc = "Field `TOUCH_DBIAS` reader - 1:use self bias 0:use bandgap bias"]
pub struct TOUCH_DBIAS_R(crate::FieldReader<bool, bool>);
impl TOUCH_DBIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_DBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DBIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DBIAS` writer - 1:use self bias 0:use bandgap bias"]
pub struct TOUCH_DBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DBIAS_W<'a> {
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
#[doc = "Field `TOUCH_SLP_TIMER_EN` reader - touch timer enable bit"]
pub struct TOUCH_SLP_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_SLP_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SLP_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_TIMER_EN` writer - touch timer enable bit"]
pub struct TOUCH_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_TIMER_EN_W<'a> {
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
#[doc = "Field `TOUCH_START_FSM_EN` reader - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm"]
pub struct TOUCH_START_FSM_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_FSM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_FSM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_FSM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_FSM_EN` writer - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm"]
pub struct TOUCH_START_FSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_FSM_EN_W<'a> {
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
#[doc = "Field `TOUCH_START_EN` reader - 1: start touch fsm"]
pub struct TOUCH_START_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_EN` writer - 1: start touch fsm"]
pub struct TOUCH_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_EN_W<'a> {
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
#[doc = "Field `TOUCH_START_FORCE` reader - 1: to start touch fsm by SW"]
pub struct TOUCH_START_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_FORCE` writer - 1: to start touch fsm by SW"]
pub struct TOUCH_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `TOUCH_XPD_WAIT` reader - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub struct TOUCH_XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl TOUCH_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_XPD_WAIT` writer - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub struct TOUCH_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 17)) | ((value as u32 & 0xff) << 17);
        self.w
    }
}
#[doc = "Field `TOUCH_SLP_CYC_DIV` reader - when a touch pad is active sleep cycle could be divided by this number"]
pub struct TOUCH_SLP_CYC_DIV_R(crate::FieldReader<u8, u8>);
impl TOUCH_SLP_CYC_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_SLP_CYC_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_CYC_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_CYC_DIV` writer - when a touch pad is active sleep cycle could be divided by this number"]
pub struct TOUCH_SLP_CYC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_CYC_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 25)) | ((value as u32 & 3) << 25);
        self.w
    }
}
#[doc = "Field `TOUCH_TIMER_FORCE_DONE` reader - force touch timer done"]
pub struct TOUCH_TIMER_FORCE_DONE_R(crate::FieldReader<u8, u8>);
impl TOUCH_TIMER_FORCE_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_TIMER_FORCE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_TIMER_FORCE_DONE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_TIMER_FORCE_DONE` writer - force touch timer done"]
pub struct TOUCH_TIMER_FORCE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_TIMER_FORCE_DONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 27)) | ((value as u32 & 3) << 27);
        self.w
    }
}
#[doc = "Field `TOUCH_RESET` reader - reset upgrade touch"]
pub struct TOUCH_RESET_R(crate::FieldReader<bool, bool>);
impl TOUCH_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_RESET` writer - reset upgrade touch"]
pub struct TOUCH_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `TOUCH_CLK_FO` reader - touch clock force on"]
pub struct TOUCH_CLK_FO_R(crate::FieldReader<bool, bool>);
impl TOUCH_CLK_FO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_CLK_FO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_CLK_FO` writer - touch clock force on"]
pub struct TOUCH_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_CLK_FO_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `TOUCH_CLKGATE_EN` reader - touch clock enable"]
pub struct TOUCH_CLKGATE_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_CLKGATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_CLKGATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_CLKGATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_CLKGATE_EN` writer - touch clock enable"]
pub struct TOUCH_CLKGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_CLKGATE_EN_W<'a> {
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
    #[doc = "Bits 2:3 - TOUCH_DRANGE"]
    #[inline(always)]
    pub fn touch_drange(&self) -> TOUCH_DRANGE_R {
        TOUCH_DRANGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TOUCH_DREFL"]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TOUCH_DREFH"]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - TOUCH_XPD_BIAS"]
    #[inline(always)]
    pub fn touch_xpd_bias(&self) -> TOUCH_XPD_BIAS_R {
        TOUCH_XPD_BIAS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - TOUCH pad0 reference cap"]
    #[inline(always)]
    pub fn touch_refc(&self) -> TOUCH_REFC_R {
        TOUCH_REFC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 1:use self bias 0:use bandgap bias"]
    #[inline(always)]
    pub fn touch_dbias(&self) -> TOUCH_DBIAS_R {
        TOUCH_DBIAS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&self) -> TOUCH_SLP_TIMER_EN_R {
        TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&self) -> TOUCH_START_FSM_EN_R {
        TOUCH_START_FSM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: start touch fsm"]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: to start touch fsm by SW"]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:26 - when a touch pad is active sleep cycle could be divided by this number"]
    #[inline(always)]
    pub fn touch_slp_cyc_div(&self) -> TOUCH_SLP_CYC_DIV_R {
        TOUCH_SLP_CYC_DIV_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - force touch timer done"]
    #[inline(always)]
    pub fn touch_timer_force_done(&self) -> TOUCH_TIMER_FORCE_DONE_R {
        TOUCH_TIMER_FORCE_DONE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - reset upgrade touch"]
    #[inline(always)]
    pub fn touch_reset(&self) -> TOUCH_RESET_R {
        TOUCH_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - touch clock force on"]
    #[inline(always)]
    pub fn touch_clk_fo(&self) -> TOUCH_CLK_FO_R {
        TOUCH_CLK_FO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - touch clock enable"]
    #[inline(always)]
    pub fn touch_clkgate_en(&self) -> TOUCH_CLKGATE_EN_R {
        TOUCH_CLKGATE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - TOUCH_DRANGE"]
    #[inline(always)]
    pub fn touch_drange(&mut self) -> TOUCH_DRANGE_W {
        TOUCH_DRANGE_W { w: self }
    }
    #[doc = "Bits 4:5 - TOUCH_DREFL"]
    #[inline(always)]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W {
        TOUCH_DREFL_W { w: self }
    }
    #[doc = "Bits 6:7 - TOUCH_DREFH"]
    #[inline(always)]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W {
        TOUCH_DREFH_W { w: self }
    }
    #[doc = "Bit 8 - TOUCH_XPD_BIAS"]
    #[inline(always)]
    pub fn touch_xpd_bias(&mut self) -> TOUCH_XPD_BIAS_W {
        TOUCH_XPD_BIAS_W { w: self }
    }
    #[doc = "Bits 9:11 - TOUCH pad0 reference cap"]
    #[inline(always)]
    pub fn touch_refc(&mut self) -> TOUCH_REFC_W {
        TOUCH_REFC_W { w: self }
    }
    #[doc = "Bit 12 - 1:use self bias 0:use bandgap bias"]
    #[inline(always)]
    pub fn touch_dbias(&mut self) -> TOUCH_DBIAS_W {
        TOUCH_DBIAS_W { w: self }
    }
    #[doc = "Bit 13 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&mut self) -> TOUCH_SLP_TIMER_EN_W {
        TOUCH_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 14 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&mut self) -> TOUCH_START_FSM_EN_W {
        TOUCH_START_FSM_EN_W { w: self }
    }
    #[doc = "Bit 15 - 1: start touch fsm"]
    #[inline(always)]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W {
        TOUCH_START_EN_W { w: self }
    }
    #[doc = "Bit 16 - 1: to start touch fsm by SW"]
    #[inline(always)]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W {
        TOUCH_START_FORCE_W { w: self }
    }
    #[doc = "Bits 17:24 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W {
        TOUCH_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 25:26 - when a touch pad is active sleep cycle could be divided by this number"]
    #[inline(always)]
    pub fn touch_slp_cyc_div(&mut self) -> TOUCH_SLP_CYC_DIV_W {
        TOUCH_SLP_CYC_DIV_W { w: self }
    }
    #[doc = "Bits 27:28 - force touch timer done"]
    #[inline(always)]
    pub fn touch_timer_force_done(&mut self) -> TOUCH_TIMER_FORCE_DONE_W {
        TOUCH_TIMER_FORCE_DONE_W { w: self }
    }
    #[doc = "Bit 29 - reset upgrade touch"]
    #[inline(always)]
    pub fn touch_reset(&mut self) -> TOUCH_RESET_W {
        TOUCH_RESET_W { w: self }
    }
    #[doc = "Bit 30 - touch clock force on"]
    #[inline(always)]
    pub fn touch_clk_fo(&mut self) -> TOUCH_CLK_FO_W {
        TOUCH_CLK_FO_W { w: self }
    }
    #[doc = "Bit 31 - touch clock enable"]
    #[inline(always)]
    pub fn touch_clkgate_en(&mut self) -> TOUCH_CLKGATE_EN_W {
        TOUCH_CLKGATE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_ctrl2]
(index.html) module"]
pub struct TOUCH_CTRL2_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_ctrl2::R]
(R) reader structure"]
impl crate::Readable for TOUCH_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_ctrl2::W]
(W) writer structure"]
impl crate::Writable for TOUCH_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_CTRL2 to value 0x0008_40cc"]
impl crate::Resettable for TOUCH_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_40cc
    }
}
