#[doc = "Register `MSS_DMM_EVENT3_REG` reader"]
pub type R = crate::R<MssDmmEvent3RegSpec>;
#[doc = "Register `MSS_DMM_EVENT3_REG` writer"]
pub type W = crate::W<MssDmmEvent3RegSpec>;
#[doc = "Field `event_trig12` reader - 0:0\\]
DMM trigger for RSS_DATA_CAPTURE_ENABLE_FALL"]
pub type EventTrig12R = crate::BitReader;
#[doc = "Field `event_trig12` writer - 0:0\\]
DMM trigger for RSS_DATA_CAPTURE_ENABLE_FALL"]
pub type EventTrig12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel12` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel12R = crate::BitReader;
#[doc = "Field `event_sel12` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig13` reader - 8:8\\]
DMM trigger for FRC_LOGICAL_FRAME_START"]
pub type EventTrig13R = crate::BitReader;
#[doc = "Field `event_trig13` writer - 8:8\\]
DMM trigger for FRC_LOGICAL_FRAME_START"]
pub type EventTrig13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel13` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel13R = crate::BitReader;
#[doc = "Field `event_sel13` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig14` reader - 16:16\\]
DMM trigger for FRC_LOGICAL_FRAME_END"]
pub type EventTrig14R = crate::BitReader;
#[doc = "Field `event_trig14` writer - 16:16\\]
DMM trigger for FRC_LOGICAL_FRAME_END"]
pub type EventTrig14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel14` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel14R = crate::BitReader;
#[doc = "Field `event_sel14` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig15` reader - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to interrupts)"]
pub type EventTrig15R = crate::BitReader;
#[doc = "Field `event_trig15` writer - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to interrupts)"]
pub type EventTrig15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel15` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel15R = crate::BitReader;
#[doc = "Field `event_sel15` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_DATA_CAPTURE_ENABLE_FALL"]
    #[inline(always)]
    pub fn event_trig12(&self) -> EventTrig12R {
        EventTrig12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel12(&self) -> EventSel12R {
        EventSel12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for FRC_LOGICAL_FRAME_START"]
    #[inline(always)]
    pub fn event_trig13(&self) -> EventTrig13R {
        EventTrig13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel13(&self) -> EventSel13R {
        EventSel13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for FRC_LOGICAL_FRAME_END"]
    #[inline(always)]
    pub fn event_trig14(&self) -> EventTrig14R {
        EventTrig14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel14(&self) -> EventSel14R {
        EventSel14R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to interrupts)"]
    #[inline(always)]
    pub fn event_trig15(&self) -> EventTrig15R {
        EventTrig15R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel15(&self) -> EventSel15R {
        EventSel15R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_DATA_CAPTURE_ENABLE_FALL"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig12(&mut self) -> EventTrig12W<MssDmmEvent3RegSpec> {
        EventTrig12W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel12(&mut self) -> EventSel12W<MssDmmEvent3RegSpec> {
        EventSel12W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for FRC_LOGICAL_FRAME_START"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig13(&mut self) -> EventTrig13W<MssDmmEvent3RegSpec> {
        EventTrig13W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel13(&mut self) -> EventSel13W<MssDmmEvent3RegSpec> {
        EventSel13W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for FRC_LOGICAL_FRAME_END"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig14(&mut self) -> EventTrig14W<MssDmmEvent3RegSpec> {
        EventTrig14W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel14(&mut self) -> EventSel14W<MssDmmEvent3RegSpec> {
        EventSel14W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to interrupts)"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig15(&mut self) -> EventTrig15W<MssDmmEvent3RegSpec> {
        EventTrig15W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel15(&mut self) -> EventSel15W<MssDmmEvent3RegSpec> {
        EventSel15W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT3_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event3_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event3_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent3RegSpec;
impl crate::RegisterSpec for MssDmmEvent3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event3_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event3_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT3_REG to value 0"]
impl crate::Resettable for MssDmmEvent3RegSpec {
    const RESET_VALUE: u32 = 0;
}
