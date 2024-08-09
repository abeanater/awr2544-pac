#[doc = "Register `PLL_PER_M2NDIV` reader"]
pub type R = crate::R<PllPerM2ndivSpec>;
#[doc = "Register `PLL_PER_M2NDIV` writer"]
pub type W = crate::W<PllPerM2ndivSpec>;
#[doc = "Field `N` reader - 7:0\\]
Pre-divider is REGN+1"]
pub type NR = crate::FieldReader;
#[doc = "Field `N` writer - 7:0\\]
Pre-divider is REGN+1"]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `M2` reader - 22:16\\]
Post-divider is REGM2"]
pub type M2R = crate::FieldReader;
#[doc = "Field `M2` writer - 22:16\\]
Post-divider is REGM2"]
pub type M2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Pre-divider is REGN+1"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Post-divider is REGM2"]
    #[inline(always)]
    pub fn m2(&self) -> M2R {
        M2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Pre-divider is REGN+1"]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> NW<PllPerM2ndivSpec> {
        NW::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Post-divider is REGM2"]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2W<PllPerM2ndivSpec> {
        M2W::new(self, 16)
    }
}
#[doc = "PLL_PER_M2NDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_m2ndiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_m2ndiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPerM2ndivSpec;
impl crate::RegisterSpec for PllPerM2ndivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_per_m2ndiv::R`](R) reader structure"]
impl crate::Readable for PllPerM2ndivSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_per_m2ndiv::W`](W) writer structure"]
impl crate::Writable for PllPerM2ndivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_PER_M2NDIV to value 0"]
impl crate::Resettable for PllPerM2ndivSpec {
    const RESET_VALUE: u32 = 0;
}
