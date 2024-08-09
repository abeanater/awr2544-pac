#[doc = "Register `TESTPATTERNRX4ICFG` reader"]
pub type R = crate::R<Testpatternrx4icfgSpec>;
#[doc = "Register `TESTPATTERNRX4ICFG` writer"]
pub type W = crate::W<Testpatternrx4icfgSpec>;
#[doc = "Field `TSTPATRX4IOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 3"]
pub type Tstpatrx4ioffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX4IOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 3"]
pub type Tstpatrx4ioffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX4IINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 3"]
pub type Tstpatrx4iincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX4IINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 3"]
pub type Tstpatrx4iincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 3"]
    #[inline(always)]
    pub fn tstpatrx4ioffset(&self) -> Tstpatrx4ioffsetR {
        Tstpatrx4ioffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 3"]
    #[inline(always)]
    pub fn tstpatrx4iincr(&self) -> Tstpatrx4iincrR {
        Tstpatrx4iincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx4ioffset(&mut self) -> Tstpatrx4ioffsetW<Testpatternrx4icfgSpec> {
        Tstpatrx4ioffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx4iincr(&mut self) -> Tstpatrx4iincrW<Testpatternrx4icfgSpec> {
        Tstpatrx4iincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX4ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx4icfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx4icfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx4icfgSpec;
impl crate::RegisterSpec for Testpatternrx4icfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx4icfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx4icfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx4icfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx4icfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX4ICFG to value 0"]
impl crate::Resettable for Testpatternrx4icfgSpec {
    const RESET_VALUE: u32 = 0;
}
