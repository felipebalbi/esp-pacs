#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC_BYPASS` reader - need des"]
pub type SYNC_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC_BYPASS` writer - need des"]
pub type SYNC_BYPASS_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 2, O>;
#[doc = "Field `PAD_DRIVER` reader - need des"]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - need des"]
pub type PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `EDGE_WAKEUP_CLR` writer - need des"]
pub type EDGE_WAKEUP_CLR_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `INT_TYPE` reader - need des"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - need des"]
pub type INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 3, O>;
#[doc = "Field `WAKEUP_ENABLE` reader - need des"]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - need des"]
pub type WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `FILTER_EN` reader - need des"]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - need des"]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    pub fn sync_bypass(&self) -> SYNC_BYPASS_R {
        SYNC_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field(
                "sync_bypass",
                &format_args!("{}", self.sync_bypass().bits()),
            )
            .field("pad_driver", &format_args!("{}", self.pad_driver().bit()))
            .field("int_type", &format_args!("{}", self.int_type().bits()))
            .field(
                "wakeup_enable",
                &format_args!("{}", self.wakeup_enable().bit()),
            )
            .field("filter_en", &format_args!("{}", self.filter_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn sync_bypass(&mut self) -> SYNC_BYPASS_W<0> {
        SYNC_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<2> {
        PAD_DRIVER_W::new(self)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn edge_wakeup_clr(&mut self) -> EDGE_WAKEUP_CLR_W<3> {
        EDGE_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn int_type(&mut self) -> INT_TYPE_W<7> {
        INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<10> {
        WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<11> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
