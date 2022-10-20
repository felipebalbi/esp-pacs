#[doc = "Register `DOEPDMA3` reader"]
pub struct R(crate::R<DOEPDMA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA3` writer"]
pub struct W(crate::W<DOEPDMA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA3_SPEC>;
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
impl From<crate::W<DOEPDMA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR3` reader - "]
pub type DMAADDR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR3` writer - "]
pub type DMAADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPDMA3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr3(&self) -> DMAADDR3_R {
        DMAADDR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr3(&mut self) -> DMAADDR3_W<0> {
        DMAADDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma3](index.html) module"]
pub struct DOEPDMA3_SPEC;
impl crate::RegisterSpec for DOEPDMA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma3::R](R) reader structure"]
impl crate::Readable for DOEPDMA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma3::W](W) writer structure"]
impl crate::Writable for DOEPDMA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPDMA3 to value 0"]
impl crate::Resettable for DOEPDMA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
