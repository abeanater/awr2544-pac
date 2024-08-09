#[doc = "Register `Interrupt Vector` reader"]
pub type R = crate::R<InterruptVectorSpec>;
#[doc = "Register `Interrupt Vector` writer"]
pub type W = crate::W<InterruptVectorSpec>;
#[doc = "Field `intr_vec` reader - 31:0\\]
Interrupt vector. Reads mpu_intr_vector input signal."]
pub type IntrVecR = crate::FieldReader<u32>;
#[doc = "Field `intr_vec` writer - 31:0\\]
Interrupt vector. Reads mpu_intr_vector input signal."]
pub type IntrVecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt vector. Reads mpu_intr_vector input signal."]
    #[inline(always)]
    pub fn intr_vec(&self) -> IntrVecR {
        IntrVecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt vector. Reads mpu_intr_vector input signal."]
    #[inline(always)]
    #[must_use]
    pub fn intr_vec(&mut self) -> IntrVecW<InterruptVectorSpec> {
        IntrVecW::new(self, 0)
    }
}
#[doc = "Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptVectorSpec;
impl crate::RegisterSpec for InterruptVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_vector::R`](R) reader structure"]
impl crate::Readable for InterruptVectorSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_vector::W`](W) writer structure"]
impl crate::Writable for InterruptVectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Interrupt Vector to value 0"]
impl crate::Resettable for InterruptVectorSpec {
    const RESET_VALUE: u32 = 0;
}
