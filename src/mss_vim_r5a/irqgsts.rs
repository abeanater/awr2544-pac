#[doc = "Register `IRQGSTS` reader"]
pub type R = crate::R<IrqgstsSpec>;
#[doc = "Register `IRQGSTS` writer"]
pub type W = crate::W<IrqgstsSpec>;
#[doc = "Field `STS` reader - 31:0\\]
Indicates that the num field is valid."]
pub type StsR = crate::FieldReader<u32>;
#[doc = "Field `STS` writer - 31:0\\]
Indicates that the num field is valid."]
pub type StsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates that the num field is valid."]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates that the num field is valid."]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> StsW<IrqgstsSpec> {
        StsW::new(self, 0)
    }
}
#[doc = "The IRQ Group Status Register indicates which groups have pending IRQ interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqgsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqgsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqgstsSpec;
impl crate::RegisterSpec for IrqgstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqgsts::R`](R) reader structure"]
impl crate::Readable for IrqgstsSpec {}
#[doc = "`write(|w| ..)` method takes [`irqgsts::W`](W) writer structure"]
impl crate::Writable for IrqgstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQGSTS to value 0"]
impl crate::Resettable for IrqgstsSpec {
    const RESET_VALUE: u32 = 0;
}
