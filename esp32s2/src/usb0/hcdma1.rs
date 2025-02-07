#[doc = "Register `HCDMA1` reader"]
pub struct R(crate::R<HCDMA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA1` writer"]
pub struct W(crate::W<HCDMA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA1_SPEC>;
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
impl From<crate::W<HCDMA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_DMAADDR1` reader - "]
pub type H_DMAADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR1` writer - "]
pub type H_DMAADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, HCDMA1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr1(&self) -> H_DMAADDR1_R {
        H_DMAADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA1")
            .field("h_dmaaddr1", &format_args!("{}", self.h_dmaaddr1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr1(&mut self) -> H_DMAADDR1_W<0> {
        H_DMAADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma1](index.html) module"]
pub struct HCDMA1_SPEC;
impl crate::RegisterSpec for HCDMA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma1::R](R) reader structure"]
impl crate::Readable for HCDMA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma1::W](W) writer structure"]
impl crate::Writable for HCDMA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA1 to value 0"]
impl crate::Resettable for HCDMA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
