#[doc = "Register `MSS_DMM_EVENT10_REG` reader"]
pub type R = crate::R<MssDmmEvent10RegSpec>;
#[doc = "Register `MSS_DMM_EVENT10_REG` writer"]
pub type W = crate::W<MssDmmEvent10RegSpec>;
#[doc = "Field `event_trig40` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig40R = crate::BitReader;
#[doc = "Field `event_trig40` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel40` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel40R = crate::BitReader;
#[doc = "Field `event_sel40` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig41` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig41R = crate::BitReader;
#[doc = "Field `event_trig41` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel41` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel41R = crate::BitReader;
#[doc = "Field `event_sel41` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig42` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig42R = crate::BitReader;
#[doc = "Field `event_trig42` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel42` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel42R = crate::BitReader;
#[doc = "Field `event_sel42` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig43` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig43R = crate::BitReader;
#[doc = "Field `event_trig43` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel43` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel43R = crate::BitReader;
#[doc = "Field `event_sel43` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel43W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig40(&self) -> EventTrig40R {
        EventTrig40R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel40(&self) -> EventSel40R {
        EventSel40R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig41(&self) -> EventTrig41R {
        EventTrig41R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel41(&self) -> EventSel41R {
        EventSel41R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig42(&self) -> EventTrig42R {
        EventTrig42R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel42(&self) -> EventSel42R {
        EventSel42R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig43(&self) -> EventTrig43R {
        EventTrig43R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel43(&self) -> EventSel43R {
        EventSel43R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig40(&mut self) -> EventTrig40W<MssDmmEvent10RegSpec> {
        EventTrig40W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel40(&mut self) -> EventSel40W<MssDmmEvent10RegSpec> {
        EventSel40W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig41(&mut self) -> EventTrig41W<MssDmmEvent10RegSpec> {
        EventTrig41W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel41(&mut self) -> EventSel41W<MssDmmEvent10RegSpec> {
        EventSel41W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig42(&mut self) -> EventTrig42W<MssDmmEvent10RegSpec> {
        EventTrig42W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel42(&mut self) -> EventSel42W<MssDmmEvent10RegSpec> {
        EventSel42W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig43(&mut self) -> EventTrig43W<MssDmmEvent10RegSpec> {
        EventTrig43W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel43(&mut self) -> EventSel43W<MssDmmEvent10RegSpec> {
        EventSel43W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT10_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event10_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event10_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent10RegSpec;
impl crate::RegisterSpec for MssDmmEvent10RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event10_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent10RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event10_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent10RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT10_REG to value 0"]
impl crate::Resettable for MssDmmEvent10RegSpec {
    const RESET_VALUE: u32 = 0;
}
