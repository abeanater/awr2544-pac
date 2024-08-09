#[doc = "Register `STCCICR` reader"]
pub type R = crate::R<StccicrSpec>;
#[doc = "Register `STCCICR` writer"]
pub type W = crate::W<StccicrSpec>;
#[doc = "Field `CORE1_ICOUNT` reader - 15:0\\]
Specifies the last interval number for CORE1 This specifies the Last executed Interval number of a self-test run."]
pub type Core1IcountR = crate::FieldReader<u16>;
#[doc = "Field `CORE1_ICOUNT` writer - 15:0\\]
Specifies the last interval number for CORE1 This specifies the Last executed Interval number of a self-test run."]
pub type Core1IcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CORE2_ICOUNT` reader - 31:16\\]
Specifies the last interval number for CORE2 This specifies the Last executed Interval number for CORE2 of Segment0 if self test is being executed for secondary core as well. This field is applicable only for Segment 0."]
pub type Core2IcountR = crate::FieldReader<u16>;
#[doc = "Field `CORE2_ICOUNT` writer - 31:16\\]
Specifies the last interval number for CORE2 This specifies the Last executed Interval number for CORE2 of Segment0 if self test is being executed for secondary core as well. This field is applicable only for Segment 0."]
pub type Core2IcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the last interval number for CORE1 This specifies the Last executed Interval number of a self-test run."]
    #[inline(always)]
    pub fn core1_icount(&self) -> Core1IcountR {
        Core1IcountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Specifies the last interval number for CORE2 This specifies the Last executed Interval number for CORE2 of Segment0 if self test is being executed for secondary core as well. This field is applicable only for Segment 0."]
    #[inline(always)]
    pub fn core2_icount(&self) -> Core2IcountR {
        Core2IcountR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the last interval number for CORE1 This specifies the Last executed Interval number of a self-test run."]
    #[inline(always)]
    #[must_use]
    pub fn core1_icount(&mut self) -> Core1IcountW<StccicrSpec> {
        Core1IcountW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Specifies the last interval number for CORE2 This specifies the Last executed Interval number for CORE2 of Segment0 if self test is being executed for secondary core as well. This field is applicable only for Segment 0."]
    #[inline(always)]
    #[must_use]
    pub fn core2_icount(&mut self) -> Core2IcountW<StccicrSpec> {
        Core2IcountW::new(self, 16)
    }
}
#[doc = "Current Interval count register\n\nYou can [`read`](crate::Reg::read) this register and get [`stccicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stccicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StccicrSpec;
impl crate::RegisterSpec for StccicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stccicr::R`](R) reader structure"]
impl crate::Readable for StccicrSpec {}
#[doc = "`write(|w| ..)` method takes [`stccicr::W`](W) writer structure"]
impl crate::Writable for StccicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCCICR to value 0"]
impl crate::Resettable for StccicrSpec {
    const RESET_VALUE: u32 = 0;
}
