#[doc = "Register `CPREG5` reader"]
pub type R = crate::R<Cpreg5Spec>;
#[doc = "Register `CPREG5` writer"]
pub type W = crate::W<Cpreg5Spec>;
#[doc = "Field `CPREG5` reader - 31:0\\]
Chirp Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg5R = crate::FieldReader<u32>;
#[doc = "Field `CPREG5` writer - 31:0\\]
Chirp Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg5(&self) -> Cpreg5R {
        Cpreg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg5(&mut self) -> Cpreg5W<Cpreg5Spec> {
        Cpreg5W::new(self, 0)
    }
}
#[doc = "CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg5Spec;
impl crate::RegisterSpec for Cpreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg5::R`](R) reader structure"]
impl crate::Readable for Cpreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg5::W`](W) writer structure"]
impl crate::Writable for Cpreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG5 to value 0"]
impl crate::Resettable for Cpreg5Spec {
    const RESET_VALUE: u32 = 0;
}
