#[doc = "Register `TESTPATTERNRX2QCFG` reader"]
pub type R = crate::R<Testpatternrx2qcfgSpec>;
#[doc = "Register `TESTPATTERNRX2QCFG` writer"]
pub type W = crate::W<Testpatternrx2qcfgSpec>;
#[doc = "Field `TSTPATRX2QOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 1."]
pub type Tstpatrx2qoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX2QOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 1."]
pub type Tstpatrx2qoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX2QINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 1."]
pub type Tstpatrx2qincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX2QINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 1."]
pub type Tstpatrx2qincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 1."]
    #[inline(always)]
    pub fn tstpatrx2qoffset(&self) -> Tstpatrx2qoffsetR {
        Tstpatrx2qoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 1."]
    #[inline(always)]
    pub fn tstpatrx2qincr(&self) -> Tstpatrx2qincrR {
        Tstpatrx2qincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx2qoffset(&mut self) -> Tstpatrx2qoffsetW<Testpatternrx2qcfgSpec> {
        Tstpatrx2qoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx2qincr(&mut self) -> Tstpatrx2qincrW<Testpatternrx2qcfgSpec> {
        Tstpatrx2qincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX2QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx2qcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx2qcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx2qcfgSpec;
impl crate::RegisterSpec for Testpatternrx2qcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx2qcfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx2qcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx2qcfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx2qcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX2QCFG to value 0"]
impl crate::Resettable for Testpatternrx2qcfgSpec {
    const RESET_VALUE: u32 = 0;
}
