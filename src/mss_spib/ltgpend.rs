#[doc = "Register `LTGPEND` reader"]
pub type R = crate::R<LtgpendSpec>;
#[doc = "Register `LTGPEND` writer"]
pub type W = crate::W<LtgpendSpec>;
#[doc = "Field `NU1` reader - 7:0\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:0\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPEND` reader - 15:8\\]
Last Transfer Group End Pointer Usually the transfer group end address (PEND) is inherently defined by the start value of the starting pointer of the subsequent transfer group (PSTART). The transfer group ends at the buffer one before the next transfer group starts (PEND\\[x\\]=PSTART\\[x+1\\]
- 1). For a full configuration of MibSPI, the 15th transfer group has no subsequent transfer group, i.e. no end address is inherently defined. Therefore LPEND has to be programmed to specify explicitly the end address of the 15th transfer group. Number of Transfer Groups implemented in a MibSPI can vary from one MibSPI to another since it is a generic parameter based implementation. If in a MibSPI, only 4 Transfer Groups are implemented, then the PEND of the 4th TG is defined by LPEND values defined in this LTGPEND register."]
pub type LpendR = crate::FieldReader;
#[doc = "Field `LPEND` writer - 15:8\\]
Last Transfer Group End Pointer Usually the transfer group end address (PEND) is inherently defined by the start value of the starting pointer of the subsequent transfer group (PSTART). The transfer group ends at the buffer one before the next transfer group starts (PEND\\[x\\]=PSTART\\[x+1\\]
- 1). For a full configuration of MibSPI, the 15th transfer group has no subsequent transfer group, i.e. no end address is inherently defined. Therefore LPEND has to be programmed to specify explicitly the end address of the 15th transfer group. Number of Transfer Groups implemented in a MibSPI can vary from one MibSPI to another since it is a generic parameter based implementation. If in a MibSPI, only 4 Transfer Groups are implemented, then the PEND of the 4th TG is defined by LPEND values defined in this LTGPEND register."]
pub type LpendW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU2` reader - 23:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 23:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TGINSERVICE` reader - 28:24\\]
Transfer Group currently being serviced by the Sequencer. Read-Only field indicating the current Transfer Group that is being serviced. This field can generally be used for code debug purpose. Read Value:TG IN SERVICE\\[4:0\\]
Description 00000b No Transfer Group is being serviced by the Sequencer 00001b Transfer Group0 is being serviced by the Sequencer ... ... 10000b Transfer Group15 is being serviced by the Sequencer 10001b - 11111b Invalid values"]
pub type TginserviceR = crate::FieldReader;
#[doc = "Field `TGINSERVICE` writer - 28:24\\]
Transfer Group currently being serviced by the Sequencer. Read-Only field indicating the current Transfer Group that is being serviced. This field can generally be used for code debug purpose. Read Value:TG IN SERVICE\\[4:0\\]
Description 00000b No Transfer Group is being serviced by the Sequencer 00001b Transfer Group0 is being serviced by the Sequencer ... ... 10000b Transfer Group15 is being serviced by the Sequencer 10001b - 11111b Invalid values"]
pub type TginserviceW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU3` reader - 31:29\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 31:29\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Last Transfer Group End Pointer Usually the transfer group end address (PEND) is inherently defined by the start value of the starting pointer of the subsequent transfer group (PSTART). The transfer group ends at the buffer one before the next transfer group starts (PEND\\[x\\]=PSTART\\[x+1\\]
- 1). For a full configuration of MibSPI, the 15th transfer group has no subsequent transfer group, i.e. no end address is inherently defined. Therefore LPEND has to be programmed to specify explicitly the end address of the 15th transfer group. Number of Transfer Groups implemented in a MibSPI can vary from one MibSPI to another since it is a generic parameter based implementation. If in a MibSPI, only 4 Transfer Groups are implemented, then the PEND of the 4th TG is defined by LPEND values defined in this LTGPEND register."]
    #[inline(always)]
    pub fn lpend(&self) -> LpendR {
        LpendR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Transfer Group currently being serviced by the Sequencer. Read-Only field indicating the current Transfer Group that is being serviced. This field can generally be used for code debug purpose. Read Value:TG IN SERVICE\\[4:0\\]
Description 00000b No Transfer Group is being serviced by the Sequencer 00001b Transfer Group0 is being serviced by the Sequencer ... ... 10000b Transfer Group15 is being serviced by the Sequencer 10001b - 11111b Invalid values"]
    #[inline(always)]
    pub fn tginservice(&self) -> TginserviceR {
        TginserviceR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<LtgpendSpec> {
        Nu1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Last Transfer Group End Pointer Usually the transfer group end address (PEND) is inherently defined by the start value of the starting pointer of the subsequent transfer group (PSTART). The transfer group ends at the buffer one before the next transfer group starts (PEND\\[x\\]=PSTART\\[x+1\\]
- 1). For a full configuration of MibSPI, the 15th transfer group has no subsequent transfer group, i.e. no end address is inherently defined. Therefore LPEND has to be programmed to specify explicitly the end address of the 15th transfer group. Number of Transfer Groups implemented in a MibSPI can vary from one MibSPI to another since it is a generic parameter based implementation. If in a MibSPI, only 4 Transfer Groups are implemented, then the PEND of the 4th TG is defined by LPEND values defined in this LTGPEND register."]
    #[inline(always)]
    #[must_use]
    pub fn lpend(&mut self) -> LpendW<LtgpendSpec> {
        LpendW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<LtgpendSpec> {
        Nu2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Transfer Group currently being serviced by the Sequencer. Read-Only field indicating the current Transfer Group that is being serviced. This field can generally be used for code debug purpose. Read Value:TG IN SERVICE\\[4:0\\]
Description 00000b No Transfer Group is being serviced by the Sequencer 00001b Transfer Group0 is being serviced by the Sequencer ... ... 10000b Transfer Group15 is being serviced by the Sequencer 10001b - 11111b Invalid values"]
    #[inline(always)]
    #[must_use]
    pub fn tginservice(&mut self) -> TginserviceW<LtgpendSpec> {
        TginserviceW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<LtgpendSpec> {
        Nu3W::new(self, 29)
    }
}
#[doc = "Last Transfer Group End Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ltgpend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltgpend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LtgpendSpec;
impl crate::RegisterSpec for LtgpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltgpend::R`](R) reader structure"]
impl crate::Readable for LtgpendSpec {}
#[doc = "`write(|w| ..)` method takes [`ltgpend::W`](W) writer structure"]
impl crate::Writable for LtgpendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTGPEND to value 0"]
impl crate::Resettable for LtgpendSpec {
    const RESET_VALUE: u32 = 0;
}
