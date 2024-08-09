#[doc = "Register `MSS_DMM_EVENT12_REG` reader"]
pub type R = crate::R<MssDmmEvent12RegSpec>;
#[doc = "Register `MSS_DMM_EVENT12_REG` writer"]
pub type W = crate::W<MssDmmEvent12RegSpec>;
#[doc = "Field `event_trig48` reader - 0:0\\]
DMM trigger For MSS_MCANA_INT0"]
pub type EventTrig48R = crate::BitReader;
#[doc = "Field `event_trig48` writer - 0:0\\]
DMM trigger For MSS_MCANA_INT0"]
pub type EventTrig48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel48` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel48R = crate::BitReader;
#[doc = "Field `event_sel48` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig49` reader - 8:8\\]
DMM trigger For MSS_MCANA_INT1"]
pub type EventTrig49R = crate::BitReader;
#[doc = "Field `event_trig49` writer - 8:8\\]
DMM trigger For MSS_MCANA_INT1"]
pub type EventTrig49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel49` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel49R = crate::BitReader;
#[doc = "Field `event_sel49` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig50` reader - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
pub type EventTrig50R = crate::BitReader;
#[doc = "Field `event_trig50` writer - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
pub type EventTrig50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel50` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel50R = crate::BitReader;
#[doc = "Field `event_sel50` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig51` reader - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
pub type EventTrig51R = crate::BitReader;
#[doc = "Field `event_trig51` writer - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
pub type EventTrig51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel51` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel51R = crate::BitReader;
#[doc = "Field `event_sel51` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel51W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger For MSS_MCANA_INT0"]
    #[inline(always)]
    pub fn event_trig48(&self) -> EventTrig48R {
        EventTrig48R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel48(&self) -> EventSel48R {
        EventSel48R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANA_INT1"]
    #[inline(always)]
    pub fn event_trig49(&self) -> EventTrig49R {
        EventTrig49R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel49(&self) -> EventSel49R {
        EventSel49R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
    #[inline(always)]
    pub fn event_trig50(&self) -> EventTrig50R {
        EventTrig50R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel50(&self) -> EventSel50R {
        EventSel50R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
    #[inline(always)]
    pub fn event_trig51(&self) -> EventTrig51R {
        EventTrig51R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel51(&self) -> EventSel51R {
        EventSel51R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger For MSS_MCANA_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig48(&mut self) -> EventTrig48W<MssDmmEvent12RegSpec> {
        EventTrig48W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel48(&mut self) -> EventSel48W<MssDmmEvent12RegSpec> {
        EventSel48W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANA_INT1"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig49(&mut self) -> EventTrig49W<MssDmmEvent12RegSpec> {
        EventTrig49W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel49(&mut self) -> EventSel49W<MssDmmEvent12RegSpec> {
        EventSel49W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig50(&mut self) -> EventTrig50W<MssDmmEvent12RegSpec> {
        EventTrig50W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel50(&mut self) -> EventSel50W<MssDmmEvent12RegSpec> {
        EventSel50W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig51(&mut self) -> EventTrig51W<MssDmmEvent12RegSpec> {
        EventTrig51W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel51(&mut self) -> EventSel51W<MssDmmEvent12RegSpec> {
        EventSel51W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT12_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event12_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event12_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent12RegSpec;
impl crate::RegisterSpec for MssDmmEvent12RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event12_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent12RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event12_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent12RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT12_REG to value 0"]
impl crate::Resettable for MssDmmEvent12RegSpec {
    const RESET_VALUE: u32 = 0;
}
