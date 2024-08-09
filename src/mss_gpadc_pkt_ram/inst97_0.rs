#[doc = "Register `INST97_0` reader"]
pub type R = crate::R<Inst97_0Spec>;
#[doc = "Register `INST97_0` writer"]
pub type W = crate::W<Inst97_0Spec>;
#[doc = "Field `CONFIG_VALUE` reader - "]
pub type ConfigValueR = crate::FieldReader<u32>;
#[doc = "Field `CONFIG_VALUE` writer - "]
pub type ConfigValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn config_value(&self) -> ConfigValueR {
        ConfigValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn config_value(&mut self) -> ConfigValueW<Inst97_0Spec> {
        ConfigValueW::new(self, 0)
    }
}
#[doc = "INST97_0\n\nYou can [`read`](crate::Reg::read) this register and get [`inst97_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inst97_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inst97_0Spec;
impl crate::RegisterSpec for Inst97_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inst97_0::R`](R) reader structure"]
impl crate::Readable for Inst97_0Spec {}
#[doc = "`write(|w| ..)` method takes [`inst97_0::W`](W) writer structure"]
impl crate::Writable for Inst97_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INST97_0 to value 0"]
impl crate::Resettable for Inst97_0Spec {
    const RESET_VALUE: u32 = 0;
}
