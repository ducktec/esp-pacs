#[doc = "Register `CH3STATUS` reader"]
pub struct R(crate::R<CH3STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_WADDR_EX` reader - reg_mem_waddr_ex_ch3."]
pub struct MEM_WADDR_EX_R(crate::FieldReader<u16, u16>);
impl MEM_WADDR_EX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_WADDR_EX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WADDR_EX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RADDR` reader - reg_apb_mem_raddr_ch3."]
pub struct APB_MEM_RADDR_R(crate::FieldReader<u16, u16>);
impl APB_MEM_RADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        APB_MEM_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` reader - reg_state_ch3."]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER_ERR` reader - reg_mem_owner_err_ch3."]
pub struct MEM_OWNER_ERR_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FULL` reader - reg_mem_full_ch3."]
pub struct MEM_FULL_R(crate::FieldReader<bool, bool>);
impl MEM_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RD_ERR` reader - reg_apb_mem_rd_err_ch3."]
pub struct APB_MEM_RD_ERR_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RD_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - reg_mem_waddr_ex_ch3."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - reg_apb_mem_raddr_ch3."]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:24 - reg_state_ch3."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - reg_mem_owner_err_ch3."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reg_mem_full_ch3."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reg_apb_mem_rd_err_ch3."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "RMT_CH3STATUS_REG.\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3status]
(index.html) module"]
pub struct CH3STATUS_SPEC;
impl crate::RegisterSpec for CH3STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3status::R]
(R) reader structure"]
impl crate::Readable for CH3STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3STATUS to value 0"]
impl crate::Resettable for CH3STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
