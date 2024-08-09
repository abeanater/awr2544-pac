#[doc = "Register `CPREG8` reader"]
pub type R = crate::R<Cpreg8Spec>;
#[doc = "Register `CPREG8` writer"]
pub type W = crate::W<Cpreg8Spec>;
#[doc = "Field `CPREG8` reader - 31:0\\]
Chirp Parameters Register 8. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg8R = crate::FieldReader<u32>;
#[doc = "Field `CPREG8` writer - 31:0\\]
Chirp Parameters Register 8. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 8. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg8(&self) -> Cpreg8R {
        Cpreg8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 8. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg8(&mut self) -> Cpreg8W<Cpreg8Spec> {
        Cpreg8W::new(self, 0)
    }
}
#[doc = "CPREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg8Spec;
impl crate::RegisterSpec for Cpreg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg8::R`](R) reader structure"]
impl crate::Readable for Cpreg8Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg8::W`](W) writer structure"]
impl crate::Writable for Cpreg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG8 to value 0"]
impl crate::Resettable for Cpreg8Spec {
    const RESET_VALUE: u32 = 0;
}
