#[doc = "Register `MSS_DMM_EVENT6_REG` reader"]
pub type R = crate::R<MssDmmEvent6RegSpec>;
#[doc = "Register `MSS_DMM_EVENT6_REG` writer"]
pub type W = crate::W<MssDmmEvent6RegSpec>;
#[doc = "Field `event_trig24` reader - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig24R = crate::BitReader;
#[doc = "Field `event_trig24` writer - 0:0\\]
DMM trigger Reserved"]
pub type EventTrig24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel24` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel24R = crate::BitReader;
#[doc = "Field `event_sel24` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig25` reader - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig25R = crate::BitReader;
#[doc = "Field `event_trig25` writer - 8:8\\]
DMM trigger Reserved"]
pub type EventTrig25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel25` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel25R = crate::BitReader;
#[doc = "Field `event_sel25` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig26` reader - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig26R = crate::BitReader;
#[doc = "Field `event_trig26` writer - 16:16\\]
DMM trigger Reserved"]
pub type EventTrig26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel26` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel26R = crate::BitReader;
#[doc = "Field `event_sel26` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig27` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig27R = crate::BitReader;
#[doc = "Field `event_trig27` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel27` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel27R = crate::BitReader;
#[doc = "Field `event_sel27` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel27W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig24(&self) -> EventTrig24R {
        EventTrig24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel24(&self) -> EventSel24R {
        EventSel24R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig25(&self) -> EventTrig25R {
        EventTrig25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel25(&self) -> EventSel25R {
        EventSel25R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig26(&self) -> EventTrig26R {
        EventTrig26R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel26(&self) -> EventSel26R {
        EventSel26R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig27(&self) -> EventTrig27R {
        EventTrig27R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel27(&self) -> EventSel27R {
        EventSel27R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig24(&mut self) -> EventTrig24W<MssDmmEvent6RegSpec> {
        EventTrig24W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel24(&mut self) -> EventSel24W<MssDmmEvent6RegSpec> {
        EventSel24W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig25(&mut self) -> EventTrig25W<MssDmmEvent6RegSpec> {
        EventTrig25W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel25(&mut self) -> EventSel25W<MssDmmEvent6RegSpec> {
        EventSel25W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig26(&mut self) -> EventTrig26W<MssDmmEvent6RegSpec> {
        EventTrig26W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel26(&mut self) -> EventSel26W<MssDmmEvent6RegSpec> {
        EventSel26W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig27(&mut self) -> EventTrig27W<MssDmmEvent6RegSpec> {
        EventTrig27W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel27(&mut self) -> EventSel27W<MssDmmEvent6RegSpec> {
        EventSel27W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT6_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event6_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event6_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent6RegSpec;
impl crate::RegisterSpec for MssDmmEvent6RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event6_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent6RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event6_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent6RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT6_REG to value 0"]
impl crate::Resettable for MssDmmEvent6RegSpec {
    const RESET_VALUE: u32 = 0;
}
