#[doc = "Register `LFSR_SEED` reader"]
pub type R = crate::R<LfsrSeedSpec>;
#[doc = "Register `LFSR_SEED` writer"]
pub type W = crate::W<LfsrSeedSpec>;
#[doc = "Field `lfsr_seed` reader - 28:0\\]
Seed for LFSR (random pattern): For twiddle factor dithering, there is an LFSR that is used, whose seed value is loaded by writing to this 29-bit LFSRSEED register. The LFSRSEED register should be set to any non-zero value, say 0x1234567"]
pub type LfsrSeedR = crate::FieldReader<u32>;
#[doc = "Field `lfsr_seed` writer - 28:0\\]
Seed for LFSR (random pattern): For twiddle factor dithering, there is an LFSR that is used, whose seed value is loaded by writing to this 29-bit LFSRSEED register. The LFSRSEED register should be set to any non-zero value, say 0x1234567"]
pub type LfsrSeedW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Seed for LFSR (random pattern): For twiddle factor dithering, there is an LFSR that is used, whose seed value is loaded by writing to this 29-bit LFSRSEED register. The LFSRSEED register should be set to any non-zero value, say 0x1234567"]
    #[inline(always)]
    pub fn lfsr_seed(&self) -> LfsrSeedR {
        LfsrSeedR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Seed for LFSR (random pattern): For twiddle factor dithering, there is an LFSR that is used, whose seed value is loaded by writing to this 29-bit LFSRSEED register. The LFSRSEED register should be set to any non-zero value, say 0x1234567"]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_seed(&mut self) -> LfsrSeedW<LfsrSeedSpec> {
        LfsrSeedW::new(self, 0)
    }
}
#[doc = "LFSR_SEED\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsr_seed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_seed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrSeedSpec;
impl crate::RegisterSpec for LfsrSeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr_seed::R`](R) reader structure"]
impl crate::Readable for LfsrSeedSpec {}
#[doc = "`write(|w| ..)` method takes [`lfsr_seed::W`](W) writer structure"]
impl crate::Writable for LfsrSeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFSR_SEED to value 0"]
impl crate::Resettable for LfsrSeedSpec {
    const RESET_VALUE: u32 = 0;
}
