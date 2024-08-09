#[doc = "Register `TESTPATTERNRX1ICFG` reader"]
pub type R = crate::R<Testpatternrx1icfgSpec>;
#[doc = "Register `TESTPATTERNRX1ICFG` writer"]
pub type W = crate::W<Testpatternrx1icfgSpec>;
#[doc = "Field `TSTPATRX1IOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1ioffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX1IOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1ioffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX1IINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1iincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX1IINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
pub type Tstpatrx1iincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    pub fn tstpatrx1ioffset(&self) -> Tstpatrx1ioffsetR {
        Tstpatrx1ioffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    pub fn tstpatrx1iincr(&self) -> Tstpatrx1iincrR {
        Tstpatrx1iincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx1ioffset(&mut self) -> Tstpatrx1ioffsetW<Testpatternrx1icfgSpec> {
        Tstpatrx1ioffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 0. In this register the naming convention for the 4 Rx channel indices are from 1 to 4 instead of 0 to 3."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx1iincr(&mut self) -> Tstpatrx1iincrW<Testpatternrx1icfgSpec> {
        Tstpatrx1iincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX1ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx1icfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx1icfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx1icfgSpec;
impl crate::RegisterSpec for Testpatternrx1icfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx1icfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx1icfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx1icfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx1icfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX1ICFG to value 0"]
impl crate::Resettable for Testpatternrx1icfgSpec {
    const RESET_VALUE: u32 = 0;
}
