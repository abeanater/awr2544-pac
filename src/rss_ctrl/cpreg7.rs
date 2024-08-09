#[doc = "Register `CPREG7` reader"]
pub type R = crate::R<Cpreg7Spec>;
#[doc = "Register `CPREG7` writer"]
pub type W = crate::W<Cpreg7Spec>;
#[doc = "Field `CPREG7` reader - 31:0\\]
Chirp Parameters Register 7. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg7R = crate::FieldReader<u32>;
#[doc = "Field `CPREG7` writer - 31:0\\]
Chirp Parameters Register 7. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 7. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg7(&self) -> Cpreg7R {
        Cpreg7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 7. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg7(&mut self) -> Cpreg7W<Cpreg7Spec> {
        Cpreg7W::new(self, 0)
    }
}
#[doc = "CPREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg7Spec;
impl crate::RegisterSpec for Cpreg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg7::R`](R) reader structure"]
impl crate::Readable for Cpreg7Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg7::W`](W) writer structure"]
impl crate::Writable for Cpreg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG7 to value 0"]
impl crate::Resettable for Cpreg7Spec {
    const RESET_VALUE: u32 = 0;
}
