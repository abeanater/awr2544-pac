#[doc = "Register `MSS_DMM_EVENT9_REG` reader"]
pub type R = crate::R<MssDmmEvent9RegSpec>;
#[doc = "Register `MSS_DMM_EVENT9_REG` writer"]
pub type W = crate::W<MssDmmEvent9RegSpec>;
#[doc = "Field `event_trig36` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig36R = crate::BitReader;
#[doc = "Field `event_trig36` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel36` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel36R = crate::BitReader;
#[doc = "Field `event_sel36` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig37` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig37R = crate::BitReader;
#[doc = "Field `event_trig37` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel37` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel37R = crate::BitReader;
#[doc = "Field `event_sel37` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig38` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig38R = crate::BitReader;
#[doc = "Field `event_trig38` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel38` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel38R = crate::BitReader;
#[doc = "Field `event_sel38` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig39` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig39R = crate::BitReader;
#[doc = "Field `event_trig39` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel39` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel39R = crate::BitReader;
#[doc = "Field `event_sel39` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel39W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig36(&self) -> EventTrig36R {
        EventTrig36R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel36(&self) -> EventSel36R {
        EventSel36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig37(&self) -> EventTrig37R {
        EventTrig37R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel37(&self) -> EventSel37R {
        EventSel37R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig38(&self) -> EventTrig38R {
        EventTrig38R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel38(&self) -> EventSel38R {
        EventSel38R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig39(&self) -> EventTrig39R {
        EventTrig39R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel39(&self) -> EventSel39R {
        EventSel39R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig36(&mut self) -> EventTrig36W<MssDmmEvent9RegSpec> {
        EventTrig36W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel36(&mut self) -> EventSel36W<MssDmmEvent9RegSpec> {
        EventSel36W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig37(&mut self) -> EventTrig37W<MssDmmEvent9RegSpec> {
        EventTrig37W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel37(&mut self) -> EventSel37W<MssDmmEvent9RegSpec> {
        EventSel37W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig38(&mut self) -> EventTrig38W<MssDmmEvent9RegSpec> {
        EventTrig38W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel38(&mut self) -> EventSel38W<MssDmmEvent9RegSpec> {
        EventSel38W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig39(&mut self) -> EventTrig39W<MssDmmEvent9RegSpec> {
        EventTrig39W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel39(&mut self) -> EventSel39W<MssDmmEvent9RegSpec> {
        EventSel39W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT9_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event9_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event9_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent9RegSpec;
impl crate::RegisterSpec for MssDmmEvent9RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event9_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent9RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event9_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent9RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT9_REG to value 0"]
impl crate::Resettable for MssDmmEvent9RegSpec {
    const RESET_VALUE: u32 = 0;
}
