#[doc = "Register `I2C_COMD6` reader"]
pub struct R(crate::R<I2C_COMD6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_COMD6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_COMD6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_COMD6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_COMD6` writer"]
pub struct W(crate::W<I2C_COMD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_COMD6_SPEC>;
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
impl From<crate::W<I2C_COMD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_COMD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_COMMAND6` reader - This is the content of command 6. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type I2C_COMMAND6_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_COMMAND6` writer - This is the content of command 6. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
pub type I2C_COMMAND6_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_COMD6_SPEC, 14, O, u16>;
#[doc = "Field `I2C_COMMAND6_DONE` reader - When command 6 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND6_DONE_R = crate::BitReader;
#[doc = "Field `I2C_COMMAND6_DONE` writer - When command 6 is done in I2C Master mode, this bit changes to high level."]
pub type I2C_COMMAND6_DONE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_COMD6_SPEC, O>;
impl R {
    #[doc = "Bits 0:13 - This is the content of command 6. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    pub fn i2c_command6(&self) -> I2C_COMMAND6_R {
        I2C_COMMAND6_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command 6 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    pub fn i2c_command6_done(&self) -> I2C_COMMAND6_DONE_R {
        I2C_COMMAND6_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_COMD6")
            .field(
                "i2c_command6",
                &format_args!("{}", self.i2c_command6().bits()),
            )
            .field(
                "i2c_command6_done",
                &format_args!("{}", self.i2c_command6_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_COMD6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command 6. It consists of three parts: op_code is the command, 0: RSTART, 1: WRITE, 2: READ, 3: STOP, 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_command6(&mut self) -> I2C_COMMAND6_W<0> {
        I2C_COMMAND6_W::new(self)
    }
    #[doc = "Bit 31 - When command 6 is done in I2C Master mode, this bit changes to high level."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_command6_done(&mut self) -> I2C_COMMAND6_DONE_W<31> {
        I2C_COMMAND6_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C command register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comd6](index.html) module"]
pub struct I2C_COMD6_SPEC;
impl crate::RegisterSpec for I2C_COMD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_comd6::R](R) reader structure"]
impl crate::Readable for I2C_COMD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_comd6::W](W) writer structure"]
impl crate::Writable for I2C_COMD6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_COMD6 to value 0"]
impl crate::Resettable for I2C_COMD6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
