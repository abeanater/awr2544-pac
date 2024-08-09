#[doc = "Register `FIQGSTS` reader"]
pub type R = crate::R<FiqgstsSpec>;
#[doc = "Register `FIQGSTS` writer"]
pub type W = crate::W<FiqgstsSpec>;
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
    pub fn sts(&mut self) -> StsW<FiqgstsSpec> {
        StsW::new(self, 0)
    }
}
#[doc = "The FIQ Group Status Register indicates which groups have pending FIQ interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqgsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqgsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiqgstsSpec;
impl crate::RegisterSpec for FiqgstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiqgsts::R`](R) reader structure"]
impl crate::Readable for FiqgstsSpec {}
#[doc = "`write(|w| ..)` method takes [`fiqgsts::W`](W) writer structure"]
impl crate::Writable for FiqgstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQGSTS to value 0"]
impl crate::Resettable for FiqgstsSpec {
    const RESET_VALUE: u32 = 0;
}
