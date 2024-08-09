#[doc = "Register `MSS_DMM_EVENT5_REG` reader"]
pub type R = crate::R<MssDmmEvent5RegSpec>;
#[doc = "Register `MSS_DMM_EVENT5_REG` writer"]
pub type W = crate::W<MssDmmEvent5RegSpec>;
#[doc = "Field `event_trig20` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig20R = crate::BitReader;
#[doc = "Field `event_trig20` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel20` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel20R = crate::BitReader;
#[doc = "Field `event_sel20` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig21` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig21R = crate::BitReader;
#[doc = "Field `event_trig21` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel21` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel21R = crate::BitReader;
#[doc = "Field `event_sel21` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig22` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig22R = crate::BitReader;
#[doc = "Field `event_trig22` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel22` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel22R = crate::BitReader;
#[doc = "Field `event_sel22` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig23` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig23R = crate::BitReader;
#[doc = "Field `event_trig23` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel23` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel23R = crate::BitReader;
#[doc = "Field `event_sel23` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig20(&self) -> EventTrig20R {
        EventTrig20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel20(&self) -> EventSel20R {
        EventSel20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig21(&self) -> EventTrig21R {
        EventTrig21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel21(&self) -> EventSel21R {
        EventSel21R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig22(&self) -> EventTrig22R {
        EventTrig22R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel22(&self) -> EventSel22R {
        EventSel22R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig23(&self) -> EventTrig23R {
        EventTrig23R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel23(&self) -> EventSel23R {
        EventSel23R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig20(&mut self) -> EventTrig20W<MssDmmEvent5RegSpec> {
        EventTrig20W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel20(&mut self) -> EventSel20W<MssDmmEvent5RegSpec> {
        EventSel20W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig21(&mut self) -> EventTrig21W<MssDmmEvent5RegSpec> {
        EventTrig21W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel21(&mut self) -> EventSel21W<MssDmmEvent5RegSpec> {
        EventSel21W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig22(&mut self) -> EventTrig22W<MssDmmEvent5RegSpec> {
        EventTrig22W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel22(&mut self) -> EventSel22W<MssDmmEvent5RegSpec> {
        EventSel22W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig23(&mut self) -> EventTrig23W<MssDmmEvent5RegSpec> {
        EventTrig23W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel23(&mut self) -> EventSel23W<MssDmmEvent5RegSpec> {
        EventSel23W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT5_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event5_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event5_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent5RegSpec;
impl crate::RegisterSpec for MssDmmEvent5RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event5_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent5RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event5_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent5RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT5_REG to value 0"]
impl crate::Resettable for MssDmmEvent5RegSpec {
    const RESET_VALUE: u32 = 0;
}
