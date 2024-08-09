#[doc = "Register `MSS_DMM_EVENT8_REG` reader"]
pub type R = crate::R<MssDmmEvent8RegSpec>;
#[doc = "Register `MSS_DMM_EVENT8_REG` writer"]
pub type W = crate::W<MssDmmEvent8RegSpec>;
#[doc = "Field `event_trig32` reader - 0:0\\]
DMM trigger for DSS_HWA_THREAD2_LOOP"]
pub type EventTrig32R = crate::BitReader;
#[doc = "Field `event_trig32` writer - 0:0\\]
DMM trigger for DSS_HWA_THREAD2_LOOP"]
pub type EventTrig32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel32` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel32R = crate::BitReader;
#[doc = "Field `event_sel32` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig33` reader - 8:8\\]
DMM trigger for DSS_HWA_THREAD2_PARAM_DONE"]
pub type EventTrig33R = crate::BitReader;
#[doc = "Field `event_trig33` writer - 8:8\\]
DMM trigger for DSS_HWA_THREAD2_PARAM_DONE"]
pub type EventTrig33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel33` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel33R = crate::BitReader;
#[doc = "Field `event_sel33` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig34` reader - 16:16\\]
DMM trigger for DSS_HWA_LOCAL_RAM_ERR"]
pub type EventTrig34R = crate::BitReader;
#[doc = "Field `event_trig34` writer - 16:16\\]
DMM trigger for DSS_HWA_LOCAL_RAM_ERR"]
pub type EventTrig34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel34` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel34R = crate::BitReader;
#[doc = "Field `event_sel34` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig35` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig35R = crate::BitReader;
#[doc = "Field `event_trig35` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel35` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel35R = crate::BitReader;
#[doc = "Field `event_sel35` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel35W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for DSS_HWA_THREAD2_LOOP"]
    #[inline(always)]
    pub fn event_trig32(&self) -> EventTrig32R {
        EventTrig32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel32(&self) -> EventSel32R {
        EventSel32R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for DSS_HWA_THREAD2_PARAM_DONE"]
    #[inline(always)]
    pub fn event_trig33(&self) -> EventTrig33R {
        EventTrig33R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel33(&self) -> EventSel33R {
        EventSel33R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DSS_HWA_LOCAL_RAM_ERR"]
    #[inline(always)]
    pub fn event_trig34(&self) -> EventTrig34R {
        EventTrig34R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel34(&self) -> EventSel34R {
        EventSel34R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig35(&self) -> EventTrig35R {
        EventTrig35R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel35(&self) -> EventSel35R {
        EventSel35R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger for DSS_HWA_THREAD2_LOOP"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig32(&mut self) -> EventTrig32W<MssDmmEvent8RegSpec> {
        EventTrig32W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel32(&mut self) -> EventSel32W<MssDmmEvent8RegSpec> {
        EventSel32W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger for DSS_HWA_THREAD2_PARAM_DONE"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig33(&mut self) -> EventTrig33W<MssDmmEvent8RegSpec> {
        EventTrig33W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel33(&mut self) -> EventSel33W<MssDmmEvent8RegSpec> {
        EventSel33W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger for DSS_HWA_LOCAL_RAM_ERR"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig34(&mut self) -> EventTrig34W<MssDmmEvent8RegSpec> {
        EventTrig34W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel34(&mut self) -> EventSel34W<MssDmmEvent8RegSpec> {
        EventSel34W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig35(&mut self) -> EventTrig35W<MssDmmEvent8RegSpec> {
        EventTrig35W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel35(&mut self) -> EventSel35W<MssDmmEvent8RegSpec> {
        EventSel35W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT8_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event8_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event8_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent8RegSpec;
impl crate::RegisterSpec for MssDmmEvent8RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event8_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent8RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event8_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent8RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT8_REG to value 0"]
impl crate::Resettable for MssDmmEvent8RegSpec {
    const RESET_VALUE: u32 = 0;
}
