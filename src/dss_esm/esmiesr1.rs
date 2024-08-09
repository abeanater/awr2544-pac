#[doc = "Register `ESMIESR1` reader"]
pub type R = crate::R<Esmiesr1Spec>;
#[doc = "Register `ESMIESR1` writer"]
pub type W = crate::W<Esmiesr1Spec>;
#[doc = "Field `INTENSET` reader - 31:0\\]
Set interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding clear bit in the ESMIECR1 register unchanged. 1 Read: Interrupt is enabled. Write: Enables interrupt and sets the corresponding clear bit in the ESMIECR1 register."]
pub type IntensetR = crate::FieldReader<u32>;
#[doc = "Field `INTENSET` writer - 31:0\\]
Set interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding clear bit in the ESMIECR1 register unchanged. 1 Read: Interrupt is enabled. Write: Enables interrupt and sets the corresponding clear bit in the ESMIECR1 register."]
pub type IntensetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding clear bit in the ESMIECR1 register unchanged. 1 Read: Interrupt is enabled. Write: Enables interrupt and sets the corresponding clear bit in the ESMIECR1 register."]
    #[inline(always)]
    pub fn intenset(&self) -> IntensetR {
        IntensetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding clear bit in the ESMIECR1 register unchanged. 1 Read: Interrupt is enabled. Write: Enables interrupt and sets the corresponding clear bit in the ESMIECR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn intenset(&mut self) -> IntensetW<Esmiesr1Spec> {
        IntensetW::new(self, 0)
    }
}
#[doc = "ESM Interrupt Enable Set/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiesr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiesr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiesr1Spec;
impl crate::RegisterSpec for Esmiesr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiesr1::R`](R) reader structure"]
impl crate::Readable for Esmiesr1Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiesr1::W`](W) writer structure"]
impl crate::Writable for Esmiesr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIESR1 to value 0"]
impl crate::Resettable for Esmiesr1Spec {
    const RESET_VALUE: u32 = 0;
}
