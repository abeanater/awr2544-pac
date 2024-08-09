#[doc = "Register `TESTPATTERNRX2ICFG` reader"]
pub type R = crate::R<Testpatternrx2icfgSpec>;
#[doc = "Register `TESTPATTERNRX2ICFG` writer"]
pub type W = crate::W<Testpatternrx2icfgSpec>;
#[doc = "Field `TSTPATRX2IOFFSET` reader - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 1."]
pub type Tstpatrx2ioffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX2IOFFSET` writer - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 1."]
pub type Tstpatrx2ioffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTPATRX2IINCR` reader - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 1."]
pub type Tstpatrx2iincrR = crate::FieldReader<u16>;
#[doc = "Field `TSTPATRX2IINCR` writer - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 1."]
pub type Tstpatrx2iincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 1."]
    #[inline(always)]
    pub fn tstpatrx2ioffset(&self) -> Tstpatrx2ioffsetR {
        Tstpatrx2ioffsetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 1."]
    #[inline(always)]
    pub fn tstpatrx2iincr(&self) -> Tstpatrx2iincrR {
        Tstpatrx2iincrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Offset value to be used for the first sample for the test pattern data in I channel Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx2ioffset(&mut self) -> Tstpatrx2ioffsetW<Testpatternrx2icfgSpec> {
        Tstpatrx2ioffsetW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value to be added for each successive sample for the test pattern data in I channel Rx channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn tstpatrx2iincr(&mut self) -> Tstpatrx2iincrW<Testpatternrx2icfgSpec> {
        Tstpatrx2iincrW::new(self, 16)
    }
}
#[doc = "TESTPATTERNRX2ICFG\n\nYou can [`read`](crate::Reg::read) this register and get [`testpatternrx2icfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testpatternrx2icfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Testpatternrx2icfgSpec;
impl crate::RegisterSpec for Testpatternrx2icfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testpatternrx2icfg::R`](R) reader structure"]
impl crate::Readable for Testpatternrx2icfgSpec {}
#[doc = "`write(|w| ..)` method takes [`testpatternrx2icfg::W`](W) writer structure"]
impl crate::Writable for Testpatternrx2icfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTPATTERNRX2ICFG to value 0"]
impl crate::Resettable for Testpatternrx2icfgSpec {
    const RESET_VALUE: u32 = 0;
}
