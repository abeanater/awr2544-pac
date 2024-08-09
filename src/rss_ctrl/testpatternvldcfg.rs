#[doc = "Register `TESTPATTERNVLDCFG` reader"]
pub type R = crate::R<TestpatternvldcfgSpec>;
#[doc = "Register `TESTPATTERNVLDCFG` writer"]
pub type W = crate::W<TestpatternvldcfgSpec>;
#[doc = "Field `TSTPATVLDCNT` reader - 7:0\\]
Number of DSS Interconnect clocks (200 MHz) between successive samples for the test pattern gen."]
pub type TstpatvldcntR = crate::FieldReader;
#[doc = "Field `TSTPATVLDCNT` writer - 7:0\\]
Number of DSS Interconnect clocks (200 MHz) between successive samples for the test pattern gen."]
pub type TstpatvldcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSTPATGENEN` reader - 10:8\\]
Enable for test pattern generator. This is used to Mux with the functional data from BSS. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type TstpatgenenR = crate::FieldReader;
#[doc = "Field `TSTPATGENEN` writer - 10:8\\]
Enable for test pattern generator. This is used to Mux with the functional data from BSS. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type TstpatgenenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of DSS Interconnect clocks (200 MHz) between successive samples for the test pattern gen."]
    #[inline(always)]
    pub fn tstpatvldcnt(&self) -> TstpatvldcntR {
        TstpatvldcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable for test pattern generator. This is used to Mux with the functional data from BSS. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    pub fn tstpatgenen(&self) -> TstpatgenenR {
        TstpatgenenR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of DSS Interconnect clocks (200 MHz) between successive samples for the test pattern gen."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatvldcnt(&mut self) -> TstpatvldcntW<TestpatternvldcfgSpec> {
        TstpatvldcntW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Enable for test pattern generator. This is used to Mux with the functional data from BSS. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatgenen(&mut self) -> TstpatgenenW<TestpatternvldcfgSpec> {
        TstpatgenenW::new(self, 8)
    }
}
#[doc = "TESTPATTERNVLDCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternvldcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternvldcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestpatternvldcfgSpec;
impl crate::RegisterSpec for TestpatternvldcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternvldcfg::R`](R) reader structure"]
impl crate::Readable for TestpatternvldcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternvldcfg::W`](W) writer structure"]
impl crate::Writable for TestpatternvldcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNVLDCFG to value 0"]
impl crate::Resettable for TestpatternvldcfgSpec {
    const RESET_VALUE: u32 = 0;
}
