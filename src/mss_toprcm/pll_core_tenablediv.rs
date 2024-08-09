#[doc = "Register `PLL_CORE_TENABLEDIV` reader"]
pub type R = crate::R<PllCoreTenabledivSpec>;
#[doc = "Register `PLL_CORE_TENABLEDIV` writer"]
pub type W = crate::W<PllCoreTenabledivSpec>;
#[doc = "Field `TENABLEDIV` reader - 0:0\\]
M2 and N2 latch (active rise edge)"]
pub type TenabledivR = crate::BitReader;
#[doc = "Field `TENABLEDIV` writer - 0:0\\]
M2 and N2 latch (active rise edge)"]
pub type TenabledivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
M2 and N2 latch (active rise edge)"]
    #[inline(always)]
    pub fn tenablediv(&self) -> TenabledivR {
        TenabledivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
M2 and N2 latch (active rise edge)"]
    #[inline(always)]
    #[must_use]
    pub fn tenablediv(&mut self) -> TenabledivW<PllCoreTenabledivSpec> {
        TenabledivW::new(self, 0)
    }
}
#[doc = "PLL_CORE_TENABLEDIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_tenablediv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_tenablediv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCoreTenabledivSpec;
impl crate::RegisterSpec for PllCoreTenabledivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_core_tenablediv::R`](R) reader structure"]
impl crate::Readable for PllCoreTenabledivSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_core_tenablediv::W`](W) writer structure"]
impl crate::Writable for PllCoreTenabledivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CORE_TENABLEDIV to value 0"]
impl crate::Resettable for PllCoreTenabledivSpec {
    const RESET_VALUE: u32 = 0;
}
