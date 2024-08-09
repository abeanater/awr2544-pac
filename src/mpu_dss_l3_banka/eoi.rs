#[doc = "Register `EOI` reader"]
pub type R = crate::R<EoiSpec>;
#[doc = "Register `EOI` writer"]
pub type W = crate::W<EoiSpec>;
#[doc = "Field `eoi_vector` reader - 7:0\\]
EOI vector value.Write this with the interrupt distribution value in the chip. This drives the mpu_eoi_vector output signal."]
pub type EoiVectorR = crate::FieldReader;
#[doc = "Field `eoi_vector` writer - 7:0\\]
EOI vector value.Write this with the interrupt distribution value in the chip. This drives the mpu_eoi_vector output signal."]
pub type EoiVectorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value.Write this with the interrupt distribution value in the chip. This drives the mpu_eoi_vector output signal."]
    #[inline(always)]
    pub fn eoi_vector(&self) -> EoiVectorR {
        EoiVectorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value.Write this with the interrupt distribution value in the chip. This drives the mpu_eoi_vector output signal."]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector(&mut self) -> EoiVectorW<EoiSpec> {
        EoiVectorW::new(self, 0)
    }
}
#[doc = "EOI\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EoiSpec;
impl crate::RegisterSpec for EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eoi::R`](R) reader structure"]
impl crate::Readable for EoiSpec {}
#[doc = "`write(|w| ..)` method takes [`eoi::W`](W) writer structure"]
impl crate::Writable for EoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EOI to value 0"]
impl crate::Resettable for EoiSpec {
    const RESET_VALUE: u32 = 0;
}
