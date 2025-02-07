#[doc = "Register `BLK3_RDATA0` reader"]
pub struct R(crate::R<BLK3_RDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_RDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_RDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_RDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_CUSTOM_MAC_CRC` reader - "]
pub type RD_CUSTOM_MAC_CRC_R = crate::FieldReader;
#[doc = "Field `RD_CUSTOM_MAC` reader - "]
pub type RD_CUSTOM_MAC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_custom_mac_crc(&self) -> RD_CUSTOM_MAC_CRC_R {
        RD_CUSTOM_MAC_CRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rd_custom_mac(&self) -> RD_CUSTOM_MAC_R {
        RD_CUSTOM_MAC_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA0")
            .field(
                "rd_custom_mac_crc",
                &format_args!("{}", self.rd_custom_mac_crc().bits()),
            )
            .field(
                "rd_custom_mac",
                &format_args!("{}", self.rd_custom_mac().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_RDATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_rdata0](index.html) module"]
pub struct BLK3_RDATA0_SPEC;
impl crate::RegisterSpec for BLK3_RDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_rdata0::R](R) reader structure"]
impl crate::Readable for BLK3_RDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK3_RDATA0 to value 0"]
impl crate::Resettable for BLK3_RDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
