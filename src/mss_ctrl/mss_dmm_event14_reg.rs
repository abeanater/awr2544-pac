#[doc = "Register `MSS_DMM_EVENT14_REG` reader"]
pub type R = crate::R<MssDmmEvent14RegSpec>;
#[doc = "Register `MSS_DMM_EVENT14_REG` writer"]
pub type W = crate::W<MssDmmEvent14RegSpec>;
#[doc = "Field `event_trig56` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig56R = crate::BitReader;
#[doc = "Field `event_trig56` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel56` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel56R = crate::BitReader;
#[doc = "Field `event_sel56` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig57` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig57R = crate::BitReader;
#[doc = "Field `event_trig57` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel57` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel57R = crate::BitReader;
#[doc = "Field `event_sel57` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig58` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig58R = crate::BitReader;
#[doc = "Field `event_trig58` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel58` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel58R = crate::BitReader;
#[doc = "Field `event_sel58` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig59` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig59R = crate::BitReader;
#[doc = "Field `event_trig59` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig59W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel59` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel59R = crate::BitReader;
#[doc = "Field `event_sel59` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel59W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig56(&self) -> EventTrig56R {
        EventTrig56R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel56(&self) -> EventSel56R {
        EventSel56R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig57(&self) -> EventTrig57R {
        EventTrig57R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel57(&self) -> EventSel57R {
        EventSel57R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig58(&self) -> EventTrig58R {
        EventTrig58R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel58(&self) -> EventSel58R {
        EventSel58R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig59(&self) -> EventTrig59R {
        EventTrig59R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel59(&self) -> EventSel59R {
        EventSel59R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig56(&mut self) -> EventTrig56W<MssDmmEvent14RegSpec> {
        EventTrig56W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel56(&mut self) -> EventSel56W<MssDmmEvent14RegSpec> {
        EventSel56W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig57(&mut self) -> EventTrig57W<MssDmmEvent14RegSpec> {
        EventTrig57W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel57(&mut self) -> EventSel57W<MssDmmEvent14RegSpec> {
        EventSel57W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig58(&mut self) -> EventTrig58W<MssDmmEvent14RegSpec> {
        EventTrig58W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel58(&mut self) -> EventSel58W<MssDmmEvent14RegSpec> {
        EventSel58W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig59(&mut self) -> EventTrig59W<MssDmmEvent14RegSpec> {
        EventTrig59W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel59(&mut self) -> EventSel59W<MssDmmEvent14RegSpec> {
        EventSel59W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT14_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event14_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event14_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent14RegSpec;
impl crate::RegisterSpec for MssDmmEvent14RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event14_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent14RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event14_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent14RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT14_REG to value 0"]
impl crate::Resettable for MssDmmEvent14RegSpec {
    const RESET_VALUE: u32 = 0;
}
