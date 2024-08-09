#[doc = "Register `MSS_DMM_EVENT4_REG` reader"]
pub type R = crate::R<MssDmmEvent4RegSpec>;
#[doc = "Register `MSS_DMM_EVENT4_REG` writer"]
pub type W = crate::W<MssDmmEvent4RegSpec>;
#[doc = "Field `event_trig16` reader - 0:0\\]
DMM trigger for ADC_CLK_ENABLE_VALID"]
pub type EventTrig16R = crate::BitReader;
#[doc = "Field `event_trig16` writer - 0:0\\]
DMM trigger for ADC_CLK_ENABLE_VALID"]
pub type EventTrig16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel16` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel16R = crate::BitReader;
#[doc = "Field `event_sel16` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig17` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig17R = crate::BitReader;
#[doc = "Field `event_trig17` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel17` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel17R = crate::BitReader;
#[doc = "Field `event_sel17` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig18` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig18R = crate::BitReader;
#[doc = "Field `event_trig18` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel18` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel18R = crate::BitReader;
#[doc = "Field `event_sel18` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig19` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig19R = crate::BitReader;
#[doc = "Field `event_trig19` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel19` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel19R = crate::BitReader;
#[doc = "Field `event_sel19` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for ADC_CLK_ENABLE_VALID"]
    #[inline(always)]
    pub fn event_trig16(&self) -> EventTrig16R {
        EventTrig16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel16(&self) -> EventSel16R {
        EventSel16R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig17(&self) -> EventTrig17R {
        EventTrig17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel17(&self) -> EventSel17R {
        EventSel17R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig18(&self) -> EventTrig18R {
        EventTrig18R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel18(&self) -> EventSel18R {
        EventSel18R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig19(&self) -> EventTrig19R {
        EventTrig19R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel19(&self) -> EventSel19R {
        EventSel19R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for ADC_CLK_ENABLE_VALID"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig16(&mut self) -> EventTrig16W<MssDmmEvent4RegSpec> {
        EventTrig16W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel16(&mut self) -> EventSel16W<MssDmmEvent4RegSpec> {
        EventSel16W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig17(&mut self) -> EventTrig17W<MssDmmEvent4RegSpec> {
        EventTrig17W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel17(&mut self) -> EventSel17W<MssDmmEvent4RegSpec> {
        EventSel17W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig18(&mut self) -> EventTrig18W<MssDmmEvent4RegSpec> {
        EventTrig18W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel18(&mut self) -> EventSel18W<MssDmmEvent4RegSpec> {
        EventSel18W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig19(&mut self) -> EventTrig19W<MssDmmEvent4RegSpec> {
        EventTrig19W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel19(&mut self) -> EventSel19W<MssDmmEvent4RegSpec> {
        EventSel19W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT4_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event4_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event4_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent4RegSpec;
impl crate::RegisterSpec for MssDmmEvent4RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event4_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent4RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event4_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent4RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT4_REG to value 0"]
impl crate::Resettable for MssDmmEvent4RegSpec {
    const RESET_VALUE: u32 = 0;
}
