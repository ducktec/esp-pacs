#[doc = "Register `CORE0_DBUS_REJECT_ST` reader"]
pub struct R(crate::R<CORE0_DBUS_REJECT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_DBUS_REJECT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_DBUS_REJECT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_DBUS_REJECT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_DBUS_TAG_ATTR` reader - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub struct CORE0_DBUS_TAG_ATTR_R(crate::FieldReader<u8, u8>);
impl CORE0_DBUS_TAG_ATTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE0_DBUS_TAG_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_TAG_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_DBUS_ATTR` reader - The bits are used to indicate the attribute of CPU access dbus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub struct CORE0_DBUS_ATTR_R(crate::FieldReader<u8, u8>);
impl CORE0_DBUS_ATTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE0_DBUS_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_DBUS_WORLD` reader - The bit is used to indicate the world of CPU access dbus when authentication fail. 0: WORLD0, 1: WORLD1"]
pub struct CORE0_DBUS_WORLD_R(crate::FieldReader<bool, bool>);
impl CORE0_DBUS_WORLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_DBUS_WORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_WORLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn core0_dbus_tag_attr(&self) -> CORE0_DBUS_TAG_ATTR_R {
        CORE0_DBUS_TAG_ATTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The bits are used to indicate the attribute of CPU access dbus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn core0_dbus_attr(&self) -> CORE0_DBUS_ATTR_R {
        CORE0_DBUS_ATTR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The bit is used to indicate the world of CPU access dbus when authentication fail. 0: WORLD0, 1: WORLD1"]
    #[inline(always)]
    pub fn core0_dbus_world(&self) -> CORE0_DBUS_WORLD_R {
        CORE0_DBUS_WORLD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`]
(crate::generic::Reg::read). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_dbus_reject_st]
(index.html) module"]
pub struct CORE0_DBUS_REJECT_ST_SPEC;
impl crate::RegisterSpec for CORE0_DBUS_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_dbus_reject_st::R]
(R) reader structure"]
impl crate::Readable for CORE0_DBUS_REJECT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_DBUS_REJECT_ST to value 0"]
impl crate::Resettable for CORE0_DBUS_REJECT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
