#[doc = "Register `LCD_CLOCK` reader"]
pub struct R(crate::R<LCD_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CLOCK` writer"]
pub struct W(crate::W<LCD_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CLOCK_SPEC>;
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
impl From<crate::W<LCD_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CLKCNT_N` reader - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub struct LCD_CLKCNT_N_R(crate::FieldReader<u8, u8>);
impl LCD_CLKCNT_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CLKCNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLKCNT_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLKCNT_N` writer - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub struct LCD_CLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `LCD_CLK_EQU_SYSCLK` reader - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub struct LCD_CLK_EQU_SYSCLK_R(crate::FieldReader<bool, bool>);
impl LCD_CLK_EQU_SYSCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CLK_EQU_SYSCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLK_EQU_SYSCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLK_EQU_SYSCLK` writer - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub struct LCD_CLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLK_EQU_SYSCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `LCD_CK_IDLE_EDGE` reader - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub struct LCD_CK_IDLE_EDGE_R(crate::FieldReader<bool, bool>);
impl LCD_CK_IDLE_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CK_IDLE_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CK_IDLE_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CK_IDLE_EDGE` writer - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub struct LCD_CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CK_IDLE_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `LCD_CK_OUT_EDGE` reader - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
pub struct LCD_CK_OUT_EDGE_R(crate::FieldReader<bool, bool>);
impl LCD_CK_OUT_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_CK_OUT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CK_OUT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CK_OUT_EDGE` writer - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
pub struct LCD_CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CK_OUT_EDGE_W<'a> {
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
#[doc = "Field `LCD_CLKM_DIV_NUM` reader - Integral LCD clock divider value"]
pub struct LCD_CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl LCD_CLKM_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLKM_DIV_NUM` writer - Integral LCD clock divider value"]
pub struct LCD_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | ((value as u32 & 0xff) << 9);
        self.w
    }
}
#[doc = "Field `LCD_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub struct LCD_CLKM_DIV_B_R(crate::FieldReader<u8, u8>);
impl LCD_CLKM_DIV_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CLKM_DIV_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLKM_DIV_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub struct LCD_CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Field `LCD_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub struct LCD_CLKM_DIV_A_R(crate::FieldReader<u8, u8>);
impl LCD_CLKM_DIV_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CLKM_DIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLKM_DIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub struct LCD_CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | ((value as u32 & 0x3f) << 23);
        self.w
    }
}
#[doc = "Field `LCD_CLK_SEL` reader - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub struct LCD_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl LCD_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CLK_SEL` writer - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub struct LCD_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&self) -> LCD_CLKCNT_N_R {
        LCD_CLKCNT_N_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&self) -> LCD_CLK_EQU_SYSCLK_R {
        LCD_CLK_EQU_SYSCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&self) -> LCD_CK_IDLE_EDGE_R {
        LCD_CK_IDLE_EDGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&self) -> LCD_CK_OUT_EDGE_R {
        LCD_CK_OUT_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&self) -> LCD_CLKM_DIV_NUM_R {
        LCD_CLKM_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&self) -> LCD_CLKM_DIV_B_R {
        LCD_CLKM_DIV_B_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&self) -> LCD_CLKM_DIV_A_R {
        LCD_CLKM_DIV_A_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&self) -> LCD_CLK_SEL_R {
        LCD_CLK_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&mut self) -> LCD_CLKCNT_N_W {
        LCD_CLKCNT_N_W { w: self }
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&mut self) -> LCD_CLK_EQU_SYSCLK_W {
        LCD_CLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&mut self) -> LCD_CK_IDLE_EDGE_W {
        LCD_CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 8 - 1: LCD_PCLK high in first half clock cycle. 0: LCD_PCLK low in first half clock cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&mut self) -> LCD_CK_OUT_EDGE_W {
        LCD_CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&mut self) -> LCD_CLKM_DIV_NUM_W {
        LCD_CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&mut self) -> LCD_CLKM_DIV_B_W {
        LCD_CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&mut self) -> LCD_CLKM_DIV_A_W {
        LCD_CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&mut self) -> LCD_CLK_SEL_W {
        LCD_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD clock register\n\nThis register you can [`read`]
(crate::generic::Reg::read), [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write), [`modify`]
(crate::generic::Reg::modify). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_clock]
(index.html) module"]
pub struct LCD_CLOCK_SPEC;
impl crate::RegisterSpec for LCD_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_clock::R]
(R) reader structure"]
impl crate::Readable for LCD_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_clock::W]
(W) writer structure"]
impl crate::Writable for LCD_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_CLOCK to value 0x0843"]
impl crate::Resettable for LCD_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0843
    }
}
