#[doc = "Register `BLK0_RDATA0` reader"]
pub struct R(crate::R<BLK0_RDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_EFUSE_WR_DIS` reader - "]
pub type RD_EFUSE_WR_DIS_R = crate::FieldReader<u16>;
#[doc = "Field `RD_EFUSE_RD_DIS` reader - "]
pub type RD_EFUSE_RD_DIS_R = crate::FieldReader;
#[doc = "Field `RD_FLASH_CRYPT_CNT` reader - "]
pub type RD_FLASH_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `RD_UART_DOWNLOAD_DIS` reader - "]
pub type RD_UART_DOWNLOAD_DIS_R = crate::BitReader;
#[doc = "Field `RESERVED_0_28` reader - "]
pub type RESERVED_0_28_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rd_efuse_wr_dis(&self) -> RD_EFUSE_WR_DIS_R {
        RD_EFUSE_WR_DIS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rd_efuse_rd_dis(&self) -> RD_EFUSE_RD_DIS_R {
        RD_EFUSE_RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26"]
    #[inline(always)]
    pub fn rd_flash_crypt_cnt(&self) -> RD_FLASH_CRYPT_CNT_R {
        RD_FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_uart_download_dis(&self) -> RD_UART_DOWNLOAD_DIS_R {
        RD_UART_DOWNLOAD_DIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_0_28(&self) -> RESERVED_0_28_R {
        RESERVED_0_28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA0")
            .field(
                "rd_efuse_wr_dis",
                &format_args!("{}", self.rd_efuse_wr_dis().bits()),
            )
            .field(
                "rd_efuse_rd_dis",
                &format_args!("{}", self.rd_efuse_rd_dis().bits()),
            )
            .field(
                "rd_flash_crypt_cnt",
                &format_args!("{}", self.rd_flash_crypt_cnt().bits()),
            )
            .field(
                "rd_uart_download_dis",
                &format_args!("{}", self.rd_uart_download_dis().bit()),
            )
            .field(
                "reserved_0_28",
                &format_args!("{}", self.reserved_0_28().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata0](index.html) module"]
pub struct BLK0_RDATA0_SPEC;
impl crate::RegisterSpec for BLK0_RDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata0::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA0 to value 0"]
impl crate::Resettable for BLK0_RDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
