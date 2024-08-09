#[doc = "Register `TESTPATTERNRX1QCFG` reader"]
pub type R = crate::R<Testpatternrx1qcfgSpec>;
#[doc = "Register `TESTPATTERNRX1QCFG` writer"]
pub type W = crate::W<Testpatternrx1qcfgSpec>;
#[doc = "Field `TSTPATRX1QOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1qoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX1QOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1qoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX1QINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1qincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX1QINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1qincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    pub fn tstpatrx1qoffset(&self) -> Tstpatrx1qoffsetR {
        Tstpatrx1qoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    pub fn tstpatrx1qincr(&self) -> Tstpatrx1qincrR {
        Tstpatrx1qincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx1qoffset(&mut self) -> Tstpatrx1qoffsetW<Testpatternrx1qcfgSpec> {
        Tstpatrx1qoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx1qincr(&mut self) -> Tstpatrx1qincrW<Testpatternrx1qcfgSpec> {
        Tstpatrx1qincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX1QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx1qcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx1qcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx1qcfgSpec;
impl crate::RegisterSpec for Testpatternrx1qcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx1qcfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx1qcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx1qcfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx1qcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX1QCFG to value 0"]
impl crate::Resettable for Testpatternrx1qcfgSpec {
    const RESET_VALUE: u32 = 0;
}
