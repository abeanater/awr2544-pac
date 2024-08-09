#[doc = "Register `CLR_SAFETY` reader"]
pub type R = crate::R<ClrSafetySpec>;
#[doc = "Register `CLR_SAFETY` writer"]
pub type W = crate::W<ClrSafetySpec>;
#[doc = "Field `CLR_SAFETY` reader - 31:0\\]
Clear Register field corresponding to STAT_SAFETY. Write 0x1 to Clear the field"]
pub type ClrSafetyR = crate::FieldReader<u32>;
#[doc = "Field `CLR_SAFETY` writer - 31:0\\]
Clear Register field corresponding to STAT_SAFETY. Write 0x1 to Clear the field"]
pub type ClrSafetyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Register field corresponding to STAT_SAFETY. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn clr_safety(&self) -> ClrSafetyR {
        ClrSafetyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Register field corresponding to STAT_SAFETY. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn clr_safety(&mut self) -> ClrSafetyW<ClrSafetySpec> {
        ClrSafetyW::new(self, 0)
    }
}
#[doc = "CLR_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_safety::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_safety::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSafetySpec;
impl crate::RegisterSpec for ClrSafetySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_safety::R`](R) reader structure"]
impl crate::Readable for ClrSafetySpec {}
#[doc = "`write(|w| ..)` method takes [`clr_safety::W`](W) writer structure"]
impl crate::Writable for ClrSafetySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_SAFETY to value 0"]
impl crate::Resettable for ClrSafetySpec {
    const RESET_VALUE: u32 = 0;
}
