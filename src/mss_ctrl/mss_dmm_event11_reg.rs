#[doc = "Register `MSS_DMM_EVENT11_REG` reader"]
pub type R = crate::R<MssDmmEvent11RegSpec>;
#[doc = "Register `MSS_DMM_EVENT11_REG` writer"]
pub type W = crate::W<MssDmmEvent11RegSpec>;
#[doc = "Field `event_trig44` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig44R = crate::BitReader;
#[doc = "Field `event_trig44` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel44` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel44R = crate::BitReader;
#[doc = "Field `event_sel44` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig45` reader - 8:8\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
pub type EventTrig45R = crate::BitReader;
#[doc = "Field `event_trig45` writer - 8:8\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
pub type EventTrig45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel45` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel45R = crate::BitReader;
#[doc = "Field `event_sel45` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig46` reader - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
pub type EventTrig46R = crate::BitReader;
#[doc = "Field `event_trig46` writer - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
pub type EventTrig46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel46` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel46R = crate::BitReader;
#[doc = "Field `event_sel46` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig47` reader - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
pub type EventTrig47R = crate::BitReader;
#[doc = "Field `event_trig47` writer - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
pub type EventTrig47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel47` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel47R = crate::BitReader;
#[doc = "Field `event_sel47` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel47W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig44(&self) -> EventTrig44R {
        EventTrig44R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel44(&self) -> EventSel44R {
        EventSel44R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
    #[inline(always)]
    pub fn event_trig45(&self) -> EventTrig45R {
        EventTrig45R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel45(&self) -> EventSel45R {
        EventSel45R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
    #[inline(always)]
    pub fn event_trig46(&self) -> EventTrig46R {
        EventTrig46R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel46(&self) -> EventSel46R {
        EventSel46R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
    #[inline(always)]
    pub fn event_trig47(&self) -> EventTrig47R {
        EventTrig47R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel47(&self) -> EventSel47R {
        EventSel47R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig44(&mut self) -> EventTrig44W<MssDmmEvent11RegSpec> {
        EventTrig44W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel44(&mut self) -> EventSel44W<MssDmmEvent11RegSpec> {
        EventSel44W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANA_FE_INT source 0"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig45(&mut self) -> EventTrig45W<MssDmmEvent11RegSpec> {
        EventTrig45W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel45(&mut self) -> EventSel45W<MssDmmEvent11RegSpec> {
        EventSel45W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANA_FE_INT source 1"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig46(&mut self) -> EventTrig46W<MssDmmEvent11RegSpec> {
        EventTrig46W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel46(&mut self) -> EventSel46W<MssDmmEvent11RegSpec> {
        EventSel46W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig47(&mut self) -> EventTrig47W<MssDmmEvent11RegSpec> {
        EventTrig47W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel47(&mut self) -> EventSel47W<MssDmmEvent11RegSpec> {
        EventSel47W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT11_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event11_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event11_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent11RegSpec;
impl crate::RegisterSpec for MssDmmEvent11RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event11_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent11RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event11_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent11RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT11_REG to value 0"]
impl crate::Resettable for MssDmmEvent11RegSpec {
    const RESET_VALUE: u32 = 0;
}
