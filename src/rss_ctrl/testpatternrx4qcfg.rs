#[doc = "Register `TESTPATTERNRX4QCFG` reader"]
pub type R = crate::R<Testpatternrx4qcfgSpec>;
#[doc = "Register `TESTPATTERNRX4QCFG` writer"]
pub type W = crate::W<Testpatternrx4qcfgSpec>;
#[doc = "Field `TSTPATRX4QOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 3"]
pub type Tstpatrx4qoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX4QOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 3"]
pub type Tstpatrx4qoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX4QINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 3"]
pub type Tstpatrx4qincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX4QINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 3"]
pub type Tstpatrx4qincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 3"]
    #[inline(always)]
    pub fn tstpatrx4qoffset(&self) -> Tstpatrx4qoffsetR {
        Tstpatrx4qoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 3"]
    #[inline(always)]
    pub fn tstpatrx4qincr(&self) -> Tstpatrx4qincrR {
        Tstpatrx4qincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx4qoffset(&mut self) -> Tstpatrx4qoffsetW<Testpatternrx4qcfgSpec> {
        Tstpatrx4qoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx4qincr(&mut self) -> Tstpatrx4qincrW<Testpatternrx4qcfgSpec> {
        Tstpatrx4qincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX4QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx4qcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx4qcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx4qcfgSpec;
impl crate::RegisterSpec for Testpatternrx4qcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx4qcfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx4qcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx4qcfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx4qcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX4QCFG to value 0"]
impl crate::Resettable for Testpatternrx4qcfgSpec {
    const RESET_VALUE: u32 = 0;
}
