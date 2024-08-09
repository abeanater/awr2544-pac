#[doc = "Register `TESTPATTERNRX3QCFG` reader"]
pub type R = crate::R<Testpatternrx3qcfgSpec>;
#[doc = "Register `TESTPATTERNRX3QCFG` writer"]
pub type W = crate::W<Testpatternrx3qcfgSpec>;
#[doc = "Field `TSTPATRX3QOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 2"]
pub type Tstpatrx3qoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX3QOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 2"]
pub type Tstpatrx3qoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX3QINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 2"]
pub type Tstpatrx3qincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX3QINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 2"]
pub type Tstpatrx3qincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 2"]
    #[inline(always)]
    pub fn tstpatrx3qoffset(&self) -> Tstpatrx3qoffsetR {
        Tstpatrx3qoffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 2"]
    #[inline(always)]
    pub fn tstpatrx3qincr(&self) -> Tstpatrx3qincrR {
        Tstpatrx3qincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in Q channel Rx channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx3qoffset(&mut self) -> Tstpatrx3qoffsetW<Testpatternrx3qcfgSpec> {
        Tstpatrx3qoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in Q channel Rx channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx3qincr(&mut self) -> Tstpatrx3qincrW<Testpatternrx3qcfgSpec> {
        Tstpatrx3qincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX3QCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx3qcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx3qcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx3qcfgSpec;
impl crate::RegisterSpec for Testpatternrx3qcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx3qcfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx3qcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx3qcfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx3qcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX3QCFG to value 0"]
impl crate::Resettable for Testpatternrx3qcfgSpec {
    const RESET_VALUE: u32 = 0;
}
