#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_TX_END[0-1]` reader - The masked interrupt status bit for CH%s_TX_END_INT."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END[2-3]` reader - The masked interrupt status bit for CH2_RX_END_INT."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR[0-1]` reader - The masked interrupt status bit for CH4_ERR_INT."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR[2-3]` reader - The masked interrupt status bit for CH6_ERR_INT."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT[0-1]` reader - The masked interrupt status bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT[2-3]` reader - The masked interrupt status bit for CH2_RX_THR_EVENT_INT."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_X_LOOP[0-1]` reader - The masked interrupt status bit for CH%s_TX_LOOP_INT."]
pub type CH_X_LOOP_R = crate::BitReader;
impl R {
    #[doc = "The masked interrupt status bit for CH[0-1]_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - The masked interrupt status bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> (n - 2 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH4_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH6_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> (n - 2 + 6)) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch2_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch3_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH[0-1]_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH2_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> (n - 2 + 10)) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for CH2_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for CH2_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH[0-1]_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_x_loop(&self, n: u8) -> CH_X_LOOP_R {
        CH_X_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_x_loop(&self) -> CH_X_LOOP_R {
        CH_X_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_x_loop(&self) -> CH_X_LOOP_R {
        CH_X_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("ch0_tx_end", &format_args!("{}", self.ch0_tx_end().bit()))
            .field("ch1_tx_end", &format_args!("{}", self.ch1_tx_end().bit()))
            .field("ch2_rx_end", &format_args!("{}", self.ch2_rx_end().bit()))
            .field("ch3_rx_end", &format_args!("{}", self.ch3_rx_end().bit()))
            .field("ch0_tx_err", &format_args!("{}", self.ch0_tx_err().bit()))
            .field("ch1_tx_err", &format_args!("{}", self.ch1_tx_err().bit()))
            .field("ch2_rx_err", &format_args!("{}", self.ch2_rx_err().bit()))
            .field("ch3_rx_err", &format_args!("{}", self.ch3_rx_err().bit()))
            .field(
                "ch0_tx_thr_event",
                &format_args!("{}", self.ch0_tx_thr_event().bit()),
            )
            .field(
                "ch1_tx_thr_event",
                &format_args!("{}", self.ch1_tx_thr_event().bit()),
            )
            .field(
                "ch2_rx_thr_event",
                &format_args!("{}", self.ch2_rx_thr_event().bit()),
            )
            .field(
                "ch3_rx_thr_event",
                &format_args!("{}", self.ch3_rx_thr_event().bit()),
            )
            .field("ch0_x_loop", &format_args!("{}", self.ch0_x_loop().bit()))
            .field("ch1_x_loop", &format_args!("{}", self.ch1_x_loop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
