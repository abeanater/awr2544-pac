#[doc = "Register `MSS_DMM_EVENT0_REG` reader"]
pub type R = crate::R<MssDmmEvent0RegSpec>;
#[doc = "Register `MSS_DMM_EVENT0_REG` writer"]
pub type W = crate::W<MssDmmEvent0RegSpec>;
#[doc = "Field `event_trig0` reader - 0:0\\]
DMM trigger for RSS_CSI2A_SOF_INT0"]
pub type EventTrig0R = crate::BitReader;
#[doc = "Field `event_trig0` writer - 0:0\\]
DMM trigger for RSS_CSI2A_SOF_INT0"]
pub type EventTrig0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel0` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel0R = crate::BitReader;
#[doc = "Field `event_sel0` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig1` reader - 8:8\\]
DMM trigger for RSS_CSI2A_SOF_INT1"]
pub type EventTrig1R = crate::BitReader;
#[doc = "Field `event_trig1` writer - 8:8\\]
DMM trigger for RSS_CSI2A_SOF_INT1"]
pub type EventTrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel1` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel1R = crate::BitReader;
#[doc = "Field `event_sel1` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig2` reader - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX0_INT"]
pub type EventTrig2R = crate::BitReader;
#[doc = "Field `event_trig2` writer - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX0_INT"]
pub type EventTrig2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel2` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel2R = crate::BitReader;
#[doc = "Field `event_sel2` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig3` reader - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX1_INT"]
pub type EventTrig3R = crate::BitReader;
#[doc = "Field `event_trig3` writer - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX1_INT"]
pub type EventTrig3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel3` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel3R = crate::BitReader;
#[doc = "Field `event_sel3` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_SOF_INT0"]
    #[inline(always)]
    pub fn event_trig0(&self) -> EventTrig0R {
        EventTrig0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel0(&self) -> EventSel0R {
        EventSel0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_SOF_INT1"]
    #[inline(always)]
    pub fn event_trig1(&self) -> EventTrig1R {
        EventTrig1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel1(&self) -> EventSel1R {
        EventSel1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX0_INT"]
    #[inline(always)]
    pub fn event_trig2(&self) -> EventTrig2R {
        EventTrig2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel2(&self) -> EventSel2R {
        EventSel2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX1_INT"]
    #[inline(always)]
    pub fn event_trig3(&self) -> EventTrig3R {
        EventTrig3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel3(&self) -> EventSel3R {
        EventSel3R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for RSS_CSI2A_SOF_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig0(&mut self) -> EventTrig0W<MssDmmEvent0RegSpec> {
        EventTrig0W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel0(&mut self) -> EventSel0W<MssDmmEvent0RegSpec> {
        EventSel0W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for RSS_CSI2A_SOF_INT1"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig1(&mut self) -> EventTrig1W<MssDmmEvent0RegSpec> {
        EventTrig1W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel1(&mut self) -> EventSel1W<MssDmmEvent0RegSpec> {
        EventSel1W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for RSS_CSI2A_EOL_CNTX0_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig2(&mut self) -> EventTrig2W<MssDmmEvent0RegSpec> {
        EventTrig2W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel2(&mut self) -> EventSel2W<MssDmmEvent0RegSpec> {
        EventSel2W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger for RSS_CSI2A_EOL_CNTX1_INT"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig3(&mut self) -> EventTrig3W<MssDmmEvent0RegSpec> {
        EventTrig3W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel3(&mut self) -> EventSel3W<MssDmmEvent0RegSpec> {
        EventSel3W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event0_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event0_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent0RegSpec;
impl crate::RegisterSpec for MssDmmEvent0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event0_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event0_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT0_REG to value 0"]
impl crate::Resettable for MssDmmEvent0RegSpec {
    const RESET_VALUE: u32 = 0;
}
