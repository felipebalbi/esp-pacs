#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0DATA` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGLS0DATA` reader - "]
pub type RECORD_PRO_PDEBUGLS0DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugls0data(&self) -> RECORD_PRO_PDEBUGLS0DATA_R {
        RECORD_PRO_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGLS0DATA")
            .field(
                "record_pro_pdebugls0data",
                &format_args!("{}", self.record_pro_pdebugls0data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugls0data](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugls0data::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGLS0DATA to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGLS0DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
