#[doc = "Register `TESTPATTERNRX3ICFG` reader"]
pub type R = crate::R<Testpatternrx3icfgSpec>;
#[doc = "Register `TESTPATTERNRX3ICFG` writer"]
pub type W = crate::W<Testpatternrx3icfgSpec>;
#[doc = "Field `TSTPATRX3IOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 2"]
pub type Tstpatrx3ioffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX3IOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 2"]
pub type Tstpatrx3ioffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX3IINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 2"]
pub type Tstpatrx3iincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX3IINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 2"]
pub type Tstpatrx3iincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 2"]
    #[inline(always)]
    pub fn tstpatrx3ioffset(&self) -> Tstpatrx3ioffsetR {
        Tstpatrx3ioffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 2"]
    #[inline(always)]
    pub fn tstpatrx3iincr(&self) -> Tstpatrx3iincrR {
        Tstpatrx3iincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx3ioffset(&mut self) -> Tstpatrx3ioffsetW<Testpatternrx3icfgSpec> {
        Tstpatrx3ioffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx3iincr(&mut self) -> Tstpatrx3iincrW<Testpatternrx3icfgSpec> {
        Tstpatrx3iincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX3ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx3icfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx3icfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx3icfgSpec;
impl crate::RegisterSpec for Testpatternrx3icfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx3icfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx3icfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx3icfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx3icfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX3ICFG to value 0"]
impl crate::Resettable for Testpatternrx3icfgSpec {
    const RESET_VALUE: u32 = 0;
}
