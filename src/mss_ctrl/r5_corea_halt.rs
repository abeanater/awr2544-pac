#[doc = "Register `R5_COREA_HALT` reader"]
pub type R = crate::R<R5CoreaHaltSpec>;
#[doc = "Register `R5_COREA_HALT` writer"]
pub type W = crate::W<R5CoreaHaltSpec>;
#[doc = "Field `halt` reader - 2:0\\]
writing '000' will unhalt CR5A. This register should be written only once."]
pub type HaltR = crate::FieldReader;
#[doc = "Field `halt` writer - 2:0\\]
writing '000' will unhalt CR5A. This register should be written only once."]
pub type HaltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' will unhalt CR5A. This register should be written only once."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' will unhalt CR5A. This register should be written only once."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<R5CoreaHaltSpec> {
        HaltW::new(self, 0)
    }
}
#[doc = "R5_COREA_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_corea_halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_corea_halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5CoreaHaltSpec;
impl crate::RegisterSpec for R5CoreaHaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_corea_halt::R`](R) reader structure"]
impl crate::Readable for R5CoreaHaltSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_corea_halt::W`](W) writer structure"]
impl crate::Writable for R5CoreaHaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_COREA_HALT to value 0"]
impl crate::Resettable for R5CoreaHaltSpec {
    const RESET_VALUE: u32 = 0;
}
