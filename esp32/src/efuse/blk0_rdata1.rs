#[doc = "Register `BLK0_RDATA1` reader"]
pub struct R(crate::R<BLK0_RDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_MAC` reader - "]
pub type RD_MAC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_mac(&self) -> RD_MAC_R {
        RD_MAC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA1")
            .field("rd_mac", &format_args!("{}", self.rd_mac().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata1](index.html) module"]
pub struct BLK0_RDATA1_SPEC;
impl crate::RegisterSpec for BLK0_RDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata1::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA1 to value 0"]
impl crate::Resettable for BLK0_RDATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
