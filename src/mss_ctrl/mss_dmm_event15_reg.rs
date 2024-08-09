#[doc = "Register `MSS_DMM_EVENT15_REG` reader"]
pub type R = crate::R<MssDmmEvent15RegSpec>;
#[doc = "Register `MSS_DMM_EVENT15_REG` writer"]
pub type W = crate::W<MssDmmEvent15RegSpec>;
#[doc = "Field `event_trig60` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig60R = crate::BitReader;
#[doc = "Field `event_trig60` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig60W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel60` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel60R = crate::BitReader;
#[doc = "Field `event_sel60` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel60W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig61` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig61R = crate::BitReader;
#[doc = "Field `event_trig61` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig61W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel61` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel61R = crate::BitReader;
#[doc = "Field `event_sel61` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel61W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig62` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig62R = crate::BitReader;
#[doc = "Field `event_trig62` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig62W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel62` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel62R = crate::BitReader;
#[doc = "Field `event_sel62` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel62W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig63` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig63R = crate::BitReader;
#[doc = "Field `event_trig63` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig63W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel63` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel63R = crate::BitReader;
#[doc = "Field `event_sel63` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel63W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig60(&self) -> EventTrig60R {
        EventTrig60R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel60(&self) -> EventSel60R {
        EventSel60R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig61(&self) -> EventTrig61R {
        EventTrig61R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel61(&self) -> EventSel61R {
        EventSel61R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig62(&self) -> EventTrig62R {
        EventTrig62R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel62(&self) -> EventSel62R {
        EventSel62R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig63(&self) -> EventTrig63R {
        EventTrig63R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel63(&self) -> EventSel63R {
        EventSel63R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig60(&mut self) -> EventTrig60W<MssDmmEvent15RegSpec> {
        EventTrig60W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel60(&mut self) -> EventSel60W<MssDmmEvent15RegSpec> {
        EventSel60W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig61(&mut self) -> EventTrig61W<MssDmmEvent15RegSpec> {
        EventTrig61W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel61(&mut self) -> EventSel61W<MssDmmEvent15RegSpec> {
        EventSel61W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig62(&mut self) -> EventTrig62W<MssDmmEvent15RegSpec> {
        EventTrig62W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel62(&mut self) -> EventSel62W<MssDmmEvent15RegSpec> {
        EventSel62W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig63(&mut self) -> EventTrig63W<MssDmmEvent15RegSpec> {
        EventTrig63W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel63(&mut self) -> EventSel63W<MssDmmEvent15RegSpec> {
        EventSel63W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT15_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event15_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event15_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent15RegSpec;
impl crate::RegisterSpec for MssDmmEvent15RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event15_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent15RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event15_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent15RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT15_REG to value 0"]
impl crate::Resettable for MssDmmEvent15RegSpec {
    const RESET_VALUE: u32 = 0;
}
