#[doc = "Register `CPREG9` reader"]
pub type R = crate::R<Cpreg9Spec>;
#[doc = "Register `CPREG9` writer"]
pub type W = crate::W<Cpreg9Spec>;
#[doc = "Field `CPREG9` reader - 31:0\\]
Chirp Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg9R = crate::FieldReader<u32>;
#[doc = "Field `CPREG9` writer - 31:0\\]
Chirp Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg9(&self) -> Cpreg9R {
        Cpreg9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg9(&mut self) -> Cpreg9W<Cpreg9Spec> {
        Cpreg9W::new(self, 0)
    }
}
#[doc = "CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg9Spec;
impl crate::RegisterSpec for Cpreg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg9::R`](R) reader structure"]
impl crate::Readable for Cpreg9Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg9::W`](W) writer structure"]
impl crate::Writable for Cpreg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG9 to value 0"]
impl crate::Resettable for Cpreg9Spec {
    const RESET_VALUE: u32 = 0;
}
