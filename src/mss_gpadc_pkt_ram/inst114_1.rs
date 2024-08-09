#[doc = "Register `INST114_1` reader"]
pub type R = crate::R<Inst114_1Spec>;
#[doc = "Register `INST114_1` writer"]
pub type W = crate::W<Inst114_1Spec>;
#[doc = "Field `PARAM` reader - "]
pub type ParamR = crate::FieldReader;
#[doc = "Field `PARAM` writer - "]
pub type ParamW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLLECT_SAMPLES` reader - "]
pub type CollectSamplesR = crate::FieldReader;
#[doc = "Field `COLLECT_SAMPLES` writer - "]
pub type CollectSamplesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SKIP_SAMPLES` reader - "]
pub type SkipSamplesR = crate::FieldReader;
#[doc = "Field `SKIP_SAMPLES` writer - "]
pub type SkipSamplesW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CHIRP_BRK` reader - "]
pub type ChirpBrkR = crate::BitReader;
#[doc = "Field `CHIRP_BRK` writer - "]
pub type ChirpBrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn param(&self) -> ParamR {
        ParamR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn collect_samples(&self) -> CollectSamplesR {
        CollectSamplesR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn skip_samples(&self) -> SkipSamplesR {
        SkipSamplesR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chirp_brk(&self) -> ChirpBrkR {
        ChirpBrkR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn param(&mut self) -> ParamW<Inst114_1Spec> {
        ParamW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn collect_samples(&mut self) -> CollectSamplesW<Inst114_1Spec> {
        CollectSamplesW::new(self, 8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn skip_samples(&mut self) -> SkipSamplesW<Inst114_1Spec> {
        SkipSamplesW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn chirp_brk(&mut self) -> ChirpBrkW<Inst114_1Spec> {
        ChirpBrkW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Inst114_1Spec> {
        Nu1W::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Inst114_1Spec> {
        Nu2W::new(self, 25)
    }
}
#[doc = "INST114_1\n\nYou can [`read`](crate::Reg::read) this register and get [`inst114_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inst114_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inst114_1Spec;
impl crate::RegisterSpec for Inst114_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inst114_1::R`](R) reader structure"]
impl crate::Readable for Inst114_1Spec {}
#[doc = "`write(|w| ..)` method takes [`inst114_1::W`](W) writer structure"]
impl crate::Writable for Inst114_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INST114_1 to value 0"]
impl crate::Resettable for Inst114_1Spec {
    const RESET_VALUE: u32 = 0;
}
