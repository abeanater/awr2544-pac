#[doc = "Register `ESMIECR10` reader"]
pub type R = crate::R<Esmiecr10Spec>;
#[doc = "Register `ESMIECR10` writer"]
pub type W = crate::W<Esmiecr10Spec>;
#[doc = "Field `INTENCLR` reader - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR10 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR10 register."]
pub type IntenclrR = crate::FieldReader<u32>;
#[doc = "Field `INTENCLR` writer - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR10 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR10 register."]
pub type IntenclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR10 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR10 register."]
    #[inline(always)]
    pub fn intenclr(&self) -> IntenclrR {
        IntenclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR10 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR10 register."]
    #[inline(always)]
    #[must_use]
    pub fn intenclr(&mut self) -> IntenclrW<Esmiecr10Spec> {
        IntenclrW::new(self, 0)
    }
}
#[doc = "ESM Interrupt Enable Clear/Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiecr10Spec;
impl crate::RegisterSpec for Esmiecr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiecr10::R`](R) reader structure"]
impl crate::Readable for Esmiecr10Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiecr10::W`](W) writer structure"]
impl crate::Writable for Esmiecr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIECR10 to value 0"]
impl crate::Resettable for Esmiecr10Spec {
    const RESET_VALUE: u32 = 0;
}
