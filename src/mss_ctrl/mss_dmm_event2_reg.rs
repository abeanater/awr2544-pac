#[doc = "Register `MSS_DMM_EVENT2_REG` reader"]
pub type R = crate::R<MssDmmEvent2RegSpec>;
#[doc = "Register `MSS_DMM_EVENT2_REG` writer"]
pub type W = crate::W<MssDmmEvent2RegSpec>;
#[doc = "Field `event_trig8` reader - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX6_INT"]
pub type EventTrig8R = crate::BitReader;
#[doc = "Field `event_trig8` writer - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX6_INT"]
pub type EventTrig8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel8` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel8R = crate::BitReader;
#[doc = "Field `event_sel8` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig9` reader - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX7_INT"]
pub type EventTrig9R = crate::BitReader;
#[doc = "Field `event_trig9` writer - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX7_INT"]
pub type EventTrig9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel9` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel9R = crate::BitReader;
#[doc = "Field `event_sel9` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig10` reader - 16:16\\]
DMM trigger for DFE_FRAME_START_TO_DSS"]
pub type EventTrig10R = crate::BitReader;
#[doc = "Field `event_trig10` writer - 16:16\\]
DMM trigger for DFE_FRAME_START_TO_DSS"]
pub type EventTrig10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel10` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel10R = crate::BitReader;
#[doc = "Field `event_sel10` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig11` reader - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to DMA)"]
pub type EventTrig11R = crate::BitReader;
#[doc = "Field `event_trig11` writer - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to DMA)"]
pub type EventTrig11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel11` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel11R = crate::BitReader;
#[doc = "Field `event_sel11` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX6_INT"]
    #[inline(always)]
    pub fn event_trig8(&self) -> EventTrig8R {
        EventTrig8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel8(&self) -> EventSel8R {
        EventSel8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX7_INT"]
    #[inline(always)]
    pub fn event_trig9(&self) -> EventTrig9R {
        EventTrig9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel9(&self) -> EventSel9R {
        EventSel9R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DFE_FRAME_START_TO_DSS"]
    #[inline(always)]
    pub fn event_trig10(&self) -> EventTrig10R {
        EventTrig10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel10(&self) -> EventSel10R {
        EventSel10R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to DMA)"]
    #[inline(always)]
    pub fn event_trig11(&self) -> EventTrig11R {
        EventTrig11R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel11(&self) -> EventSel11R {
        EventSel11R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX6_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig8(&mut self) -> EventTrig8W<MssDmmEvent2RegSpec> {
        EventTrig8W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel8(&mut self) -> EventSel8W<MssDmmEvent2RegSpec> {
        EventSel8W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX7_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig9(&mut self) -> EventTrig9W<MssDmmEvent2RegSpec> {
        EventTrig9W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel9(&mut self) -> EventSel9W<MssDmmEvent2RegSpec> {
        EventSel9W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DFE_FRAME_START_TO_DSS"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig10(&mut self) -> EventTrig10W<MssDmmEvent2RegSpec> {
        EventTrig10W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel10(&mut self) -> EventSel10W<MssDmmEvent2RegSpec> {
        EventSel10W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_ADC_CAPTURE_COMPLETE (to DMA)"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig11(&mut self) -> EventTrig11W<MssDmmEvent2RegSpec> {
        EventTrig11W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel11(&mut self) -> EventSel11W<MssDmmEvent2RegSpec> {
        EventSel11W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT2_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event2_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event2_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent2RegSpec;
impl crate::RegisterSpec for MssDmmEvent2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event2_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event2_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT2_REG to value 0"]
impl crate::Resettable for MssDmmEvent2RegSpec {
    const RESET_VALUE: u32 = 0;
}
