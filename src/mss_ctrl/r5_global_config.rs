#[doc = "Register `R5_GLOBAL_CONFIG` reader"]
pub type R = crate::R<R5GlobalConfigSpec>;
#[doc = "Register `R5_GLOBAL_CONFIG` writer"]
pub type W = crate::W<R5GlobalConfigSpec>;
#[doc = "Field `teinit` reader - 0:0\\]
Exception handling state at reset. 0-ARM 1-Thumb"]
pub type TeinitR = crate::BitReader;
#[doc = "Field `teinit` writer - 0:0\\]
Exception handling state at reset. 0-ARM 1-Thumb"]
pub type TeinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Exception handling state at reset. 0-ARM 1-Thumb"]
    #[inline(always)]
    pub fn teinit(&self) -> TeinitR {
        TeinitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Exception handling state at reset. 0-ARM 1-Thumb"]
    #[inline(always)]
    #[must_use]
    pub fn teinit(&mut self) -> TeinitW<R5GlobalConfigSpec> {
        TeinitW::new(self, 0)
    }
}
#[doc = "R5_GLOBAL_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_global_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_global_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5GlobalConfigSpec;
impl crate::RegisterSpec for R5GlobalConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_global_config::R`](R) reader structure"]
impl crate::Readable for R5GlobalConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_global_config::W`](W) writer structure"]
impl crate::Writable for R5GlobalConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_GLOBAL_CONFIG to value 0"]
impl crate::Resettable for R5GlobalConfigSpec {
    const RESET_VALUE: u32 = 0;
}
