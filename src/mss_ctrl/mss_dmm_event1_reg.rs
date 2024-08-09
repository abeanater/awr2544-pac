#[doc = "Register `MSS_DMM_EVENT1_REG` reader"]
pub type R = crate::R<MssDmmEvent1RegSpec>;
#[doc = "Register `MSS_DMM_EVENT1_REG` writer"]
pub type W = crate::W<MssDmmEvent1RegSpec>;
#[doc = "Field `event_trig4` reader - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX2_INT"]
pub type EventTrig4R = crate::BitReader;
#[doc = "Field `event_trig4` writer - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX2_INT"]
pub type EventTrig4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel4` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel4R = crate::BitReader;
#[doc = "Field `event_sel4` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig5` reader - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX3_INT"]
pub type EventTrig5R = crate::BitReader;
#[doc = "Field `event_trig5` writer - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX3_INT"]
pub type EventTrig5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel5` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel5R = crate::BitReader;
#[doc = "Field `event_sel5` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig6` reader - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX4_INT"]
pub type EventTrig6R = crate::BitReader;
#[doc = "Field `event_trig6` writer - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX4_INT"]
pub type EventTrig6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel6` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel6R = crate::BitReader;
#[doc = "Field `event_sel6` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig7` reader - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX5_INT"]
pub type EventTrig7R = crate::BitReader;
#[doc = "Field `event_trig7` writer - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX5_INT"]
pub type EventTrig7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel7` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel7R = crate::BitReader;
#[doc = "Field `event_sel7` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX2_INT"]
    #[inline(always)]
    pub fn event_trig4(&self) -> EventTrig4R {
        EventTrig4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel4(&self) -> EventSel4R {
        EventSel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX3_INT"]
    #[inline(always)]
    pub fn event_trig5(&self) -> EventTrig5R {
        EventTrig5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel5(&self) -> EventSel5R {
        EventSel5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX4_INT"]
    #[inline(always)]
    pub fn event_trig6(&self) -> EventTrig6R {
        EventTrig6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel6(&self) -> EventSel6R {
        EventSel6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX5_INT"]
    #[inline(always)]
    pub fn event_trig7(&self) -> EventTrig7R {
        EventTrig7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel7(&self) -> EventSel7R {
        EventSel7R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_EOL_CNTX2_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig4(&mut self) -> EventTrig4W<MssDmmEvent1RegSpec> {
        EventTrig4W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel4(&mut self) -> EventSel4W<MssDmmEvent1RegSpec> {
        EventSel4W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_EOL_CNTX3_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig5(&mut self) -> EventTrig5W<MssDmmEvent1RegSpec> {
        EventTrig5W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel5(&mut self) -> EventSel5W<MssDmmEvent1RegSpec> {
        EventSel5W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX4_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig6(&mut self) -> EventTrig6W<MssDmmEvent1RegSpec> {
        EventTrig6W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel6(&mut self) -> EventSel6W<MssDmmEvent1RegSpec> {
        EventSel6W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX5_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig7(&mut self) -> EventTrig7W<MssDmmEvent1RegSpec> {
        EventTrig7W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel7(&mut self) -> EventSel7W<MssDmmEvent1RegSpec> {
        EventSel7W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT1_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event1_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event1_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent1RegSpec;
impl crate::RegisterSpec for MssDmmEvent1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event1_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event1_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT1_REG to value 0"]
impl crate::Resettable for MssDmmEvent1RegSpec {
    const RESET_VALUE: u32 = 0;
}
