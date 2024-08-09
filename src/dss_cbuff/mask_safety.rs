#[doc = "Register `MASK_SAFETY` reader"]
pub type R = crate::R<MaskSafetySpec>;
#[doc = "Register `MASK_SAFETY` writer"]
pub type W = crate::W<MaskSafetySpec>;
#[doc = "Field `MASK_SAFETY` reader - 31:0\\]
Mask Register field corresponding to STAT_SAFETY. Refer STAT_SAFETY for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type MaskSafetyR = crate::FieldReader<u32>;
#[doc = "Field `MASK_SAFETY` writer - 31:0\\]
Mask Register field corresponding to STAT_SAFETY. Refer STAT_SAFETY for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
pub type MaskSafetyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_SAFETY. Refer STAT_SAFETY for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    pub fn mask_safety(&self) -> MaskSafetyR {
        MaskSafetyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Register field corresponding to STAT_SAFETY. Refer STAT_SAFETY for bitwise mapping. 0 : Event is unmasked and will cause an interrupt on occuruence 1 : Event is masked. No interrupt will be generated on occurrence"]
    #[inline(always)]
    #[must_use]
    pub fn mask_safety(&mut self) -> MaskSafetyW<MaskSafetySpec> {
        MaskSafetyW::new(self, 0)
    }
}
#[doc = "MASK_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_safety::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_safety::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSafetySpec;
impl crate::RegisterSpec for MaskSafetySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_safety::R`](R) reader structure"]
impl crate::Readable for MaskSafetySpec {}
#[doc = "`write(|w| ..)` method takes [`mask_safety::W`](W) writer structure"]
impl crate::Writable for MaskSafetySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_SAFETY to value 0"]
impl crate::Resettable for MaskSafetySpec {
    const RESET_VALUE: u32 = 0;
}
