#[doc = "Register `MSS_DMM_EVENT13_REG` reader"]
pub type R = crate::R<MssDmmEvent13RegSpec>;
#[doc = "Register `MSS_DMM_EVENT13_REG` writer"]
pub type W = crate::W<MssDmmEvent13RegSpec>;
#[doc = "Field `event_trig52` reader - 0:0\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
pub type EventTrig52R = crate::BitReader;
#[doc = "Field `event_trig52` writer - 0:0\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
pub type EventTrig52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel52` reader - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel52R = crate::BitReader;
#[doc = "Field `event_sel52` writer - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig53` reader - 8:8\\]
DMM trigger For MSS_MCANB_INT0"]
pub type EventTrig53R = crate::BitReader;
#[doc = "Field `event_trig53` writer - 8:8\\]
DMM trigger For MSS_MCANB_INT0"]
pub type EventTrig53W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel53` reader - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel53R = crate::BitReader;
#[doc = "Field `event_sel53` writer - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel53W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig54` reader - 16:16\\]
DMM trigger For MSS_MCANB_INT1"]
pub type EventTrig54R = crate::BitReader;
#[doc = "Field `event_trig54` writer - 16:16\\]
DMM trigger For MSS_MCANB_INT1"]
pub type EventTrig54W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel54` reader - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel54R = crate::BitReader;
#[doc = "Field `event_sel54` writer - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel54W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_trig55` reader - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig55R = crate::BitReader;
#[doc = "Field `event_trig55` writer - 24:24\\]
DMM trigger Reserved"]
pub type EventTrig55W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `event_sel55` reader - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel55R = crate::BitReader;
#[doc = "Field `event_sel55` writer - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
pub type EventSel55W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
    #[inline(always)]
    pub fn event_trig52(&self) -> EventTrig52R {
        EventTrig52R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel52(&self) -> EventSel52R {
        EventSel52R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANB_INT0"]
    #[inline(always)]
    pub fn event_trig53(&self) -> EventTrig53R {
        EventTrig53R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel53(&self) -> EventSel53R {
        EventSel53R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANB_INT1"]
    #[inline(always)]
    pub fn event_trig54(&self) -> EventTrig54R {
        EventTrig54R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel54(&self) -> EventSel54R {
        EventSel54R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    pub fn event_trig55(&self) -> EventTrig55R {
        EventTrig55R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    pub fn event_sel55(&self) -> EventSel55R {
        EventSel55R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMM trigger For MSS_MCANA_FE_INT source 2"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig52(&mut self) -> EventTrig52W<MssDmmEvent13RegSpec> {
        EventTrig52W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel52(&mut self) -> EventSel52W<MssDmmEvent13RegSpec> {
        EventSel52W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
DMM trigger For MSS_MCANB_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig53(&mut self) -> EventTrig53W<MssDmmEvent13RegSpec> {
        EventTrig53W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel53(&mut self) -> EventSel53W<MssDmmEvent13RegSpec> {
        EventSel53W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DMM trigger For MSS_MCANB_INT1"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig54(&mut self) -> EventTrig54W<MssDmmEvent13RegSpec> {
        EventTrig54W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel54(&mut self) -> EventSel54W<MssDmmEvent13RegSpec> {
        EventSel54W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DMM trigger Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn event_trig55(&mut self) -> EventTrig55W<MssDmmEvent13RegSpec> {
        EventTrig55W::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 1'b1 : Selects DMM event_trig as interrupt source. 1'b0 : Selects actual interrupt as interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn event_sel55(&mut self) -> EventSel55W<MssDmmEvent13RegSpec> {
        EventSel55W::new(self, 28)
    }
}
#[doc = "MSS_DMM_EVENT13_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dmm_event13_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dmm_event13_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDmmEvent13RegSpec;
impl crate::RegisterSpec for MssDmmEvent13RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dmm_event13_reg::R`](R) reader structure"]
impl crate::Readable for MssDmmEvent13RegSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dmm_event13_reg::W`](W) writer structure"]
impl crate::Writable for MssDmmEvent13RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DMM_EVENT13_REG to value 0"]
impl crate::Resettable for MssDmmEvent13RegSpec {
    const RESET_VALUE: u32 = 0;
}
