#[doc = "Register `PLL_PER_TENABLE` reader"]
pub type R = crate::R<PllPerTenableSpec>;
#[doc = "Register `PLL_PER_TENABLE` writer"]
pub type W = crate::W<PllPerTenableSpec>;
#[doc = "Field `TENABLE` reader - 0:0\\]
M, N. SD and SELFREQDCO latch (active rise edge)"]
pub type TenableR = crate::BitReader;
#[doc = "Field `TENABLE` writer - 0:0\\]
M, N. SD and SELFREQDCO latch (active rise edge)"]
pub type TenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
M, N. SD and SELFREQDCO latch (active rise edge)"]
    #[inline(always)]
    pub fn tenable(&self) -> TenableR {
        TenableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
M, N. SD and SELFREQDCO latch (active rise edge)"]
    #[inline(always)]
    #[must_use]
    pub fn tenable(&mut self) -> TenableW<PllPerTenableSpec> {
        TenableW::new(self, 0)
    }
}
#[doc = "PLL_PER_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_per_tenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_per_tenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPerTenableSpec;
impl crate::RegisterSpec for PllPerTenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_per_tenable::R`](R) reader structure"]
impl crate::Readable for PllPerTenableSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_per_tenable::W`](W) writer structure"]
impl crate::Writable for PllPerTenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_PER_TENABLE to value 0"]
impl crate::Resettable for PllPerTenableSpec {
    const RESET_VALUE: u32 = 0;
}
