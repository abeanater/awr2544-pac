#[doc = "Register `PLL_CORE_TENABLE` reader"]
pub type R = crate::R<PllCoreTenableSpec>;
#[doc = "Register `PLL_CORE_TENABLE` writer"]
pub type W = crate::W<PllCoreTenableSpec>;
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
    pub fn tenable(&mut self) -> TenableW<PllCoreTenableSpec> {
        TenableW::new(self, 0)
    }
}
#[doc = "PLL_CORE_TENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_tenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_tenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCoreTenableSpec;
impl crate::RegisterSpec for PllCoreTenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_core_tenable::R`](R) reader structure"]
impl crate::Readable for PllCoreTenableSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_core_tenable::W`](W) writer structure"]
impl crate::Writable for PllCoreTenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CORE_TENABLE to value 0"]
impl crate::Resettable for PllCoreTenableSpec {
    const RESET_VALUE: u32 = 0;
}
