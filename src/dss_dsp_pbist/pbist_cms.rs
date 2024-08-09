#[doc = "Register `PBIST_CMS` reader"]
pub type R = crate::R<PbistCmsSpec>;
#[doc = "Register `PBIST_CMS` writer"]
pub type W = crate::W<PbistCmsSpec>;
#[doc = "Field `PBIST_CMS` reader - 3:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type PbistCmsR = crate::FieldReader;
#[doc = "Field `PBIST_CMS` writer - 3:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
pub type PbistCmsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    pub fn pbist_cms(&self) -> PbistCmsR {
        PbistCmsR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
TI Internal Register.Reserved for HW RnD These registers do not have a default value after reset."]
    #[inline(always)]
    #[must_use]
    pub fn pbist_cms(&mut self) -> PbistCmsW<PbistCmsSpec> {
        PbistCmsW::new(self, 0)
    }
}
#[doc = "Clock mux select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_cms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_cms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistCmsSpec;
impl crate::RegisterSpec for PbistCmsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_cms::R`](R) reader structure"]
impl crate::Readable for PbistCmsSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_cms::W`](W) writer structure"]
impl crate::Writable for PbistCmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_CMS to value 0"]
impl crate::Resettable for PbistCmsSpec {
    const RESET_VALUE: u8 = 0;
}
