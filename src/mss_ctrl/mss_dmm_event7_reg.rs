#[doc = "Register `MSS_DMM_EVENT7_REG` reader"]
pub type R = crate::R<MssDmmEvent7RegSpec>;
#[doc = "Register `MSS_DMM_EVENT7_REG` writer"]
pub type W = crate::W<MssDmmEvent7RegSpec>;
#[doc = "Field `event_trig28` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig28R = crate::BitReader;
#[doc = "Field `event_trig28` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel28` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel28R = crate::BitReader;
#[doc = "Field `event_sel28` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig29` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig29R = crate::BitReader;
#[doc = "Field `event_trig29` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel29` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel29R = crate::BitReader;
#[doc = "Field `event_sel29` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig30` reader - 16:16\\]
DMM trigger for DSS_HWA_THREAD1_LOOP"]
pub type EventTrig30R = crate::BitReader;
#[doc = "Field `event_trig30` writer - 16:16\\]
DMM trigger for DSS_HWA_THREAD1_LOOP"]
pub type EventTrig30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel30` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel30R = crate::BitReader;
#[doc = "Field `event_sel30` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig31` reader - 24:24\\]
DMM trigger for DSS_HWA_THREAD1_PARAM_DONE"]
pub type EventTrig31R = crate::BitReader;
#[doc = "Field `event_trig31` writer - 24:24\\]
DMM trigger for DSS_HWA_THREAD1_PARAM_DONE"]
pub type EventTrig31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel31` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel31R = crate::BitReader;
#[doc = "Field `event_sel31` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig28(&self) -> EventTrig28R {
        EventTrig28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel28(&self) -> EventSel28R {
        EventSel28R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig29(&self) -> EventTrig29R {
        EventTrig29R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel29(&self) -> EventSel29R {
        EventSel29R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DSS_HWA_THREAD1_LOOP"]
    #[inline(always)]
    pub fn event_trig30(&self) -> EventTrig30R {
        EventTrig30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel30(&self) -> EventSel30R {
        EventSel30R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for DSS_HWA_THREAD1_PARAM_DONE"]
    #[inline(always)]
    pub fn event_trig31(&self) -> EventTrig31R {
        EventTrig31R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel31(&self) -> EventSel31R {
        EventSel31R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig28(&mut self) -> EventTrig28W<MssDmmEvent7RegSpec> {
        EventTrig28W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel28(&mut self) -> EventSel28W<MssDmmEvent7RegSpec> {
        EventSel28W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig29(&mut self) -> EventTrig29W<MssDmmEvent7RegSpec> {
        EventTrig29W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel29(&mut self) -> EventSel29W<MssDmmEvent7RegSpec> {
        EventSel29W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DSS_HWA_THREAD1_LOOP"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig30(&mut self) -> EventTrig30W<MssDmmEvent7RegSpec> {
        EventTrig30W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel30(&mut self) -> EventSel30W<MssDmmEvent7RegSpec> {
        EventSel30W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for DSS_HWA_THREAD1_PARAM_DONE"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig31(&mut self) -> EventTrig31W<MssDmmEvent7RegSpec> {
        EventTrig31W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel31(&mut self) -> EventSel31W<MssDmmEvent7RegSpec> {
        EventSel31W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT7_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event7_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event7_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent7RegSpec;
impl crate::RegisterSpec for MssDmmEvent7RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event7_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent7RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event7_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent7RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT7_REG to value 0"]
impl crate::Resettable for MssDmmEvent7RegSpec {
    const RESET_VALUE: u32 = 0;
}
