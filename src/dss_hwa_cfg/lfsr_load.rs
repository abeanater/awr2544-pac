#[doc = "Register `LFSR_LOAD` reader"]
pub type R = crate::R<LfsrLoadSpec>;
#[doc = "Register `LFSR_LOAD` writer"]
pub type W = crate::W<LfsrLoadSpec>;
#[doc = "Field `lfsr_load` reader - 0:0\\]
Its self clearing bit . It should be set for loading the LFSR_SEED. It s a self clearing bit"]
pub type LfsrLoadR = crate::BitReader;
#[doc = "Field `lfsr_load` writer - 0:0\\]
Its self clearing bit . It should be set for loading the LFSR_SEED. It s a self clearing bit"]
pub type LfsrLoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Its self clearing bit . It should be set for loading the LFSR_SEED. It s a self clearing bit"]
    #[inline(always)]
    pub fn lfsr_load(&self) -> LfsrLoadR {
        LfsrLoadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Its self clearing bit . It should be set for loading the LFSR_SEED. It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_load(&mut self) -> LfsrLoadW<LfsrLoadSpec> {
        LfsrLoadW::new(self, 0)
    }
}
#[doc = "LFSR_LOAD\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsr_load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrLoadSpec;
impl crate::RegisterSpec for LfsrLoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr_load::R`](R) reader structure"]
impl crate::Readable for LfsrLoadSpec {}
#[doc = "`write(|w| ..)` method takes [`lfsr_load::W`](W) writer structure"]
impl crate::Writable for LfsrLoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFSR_LOAD to value 0"]
impl crate::Resettable for LfsrLoadSpec {
    const RESET_VALUE: u32 = 0;
}
