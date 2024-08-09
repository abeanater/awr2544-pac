#[doc = "Register `CPREG6` reader"]
pub type R = crate::R<Cpreg6Spec>;
#[doc = "Register `CPREG6` writer"]
pub type W = crate::W<Cpreg6Spec>;
#[doc = "Field `CPREG6` reader - 31:0\\]
Chirp Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg6R = crate::FieldReader<u32>;
#[doc = "Field `CPREG6` writer - 31:0\\]
Chirp Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg6(&self) -> Cpreg6R {
        Cpreg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 6. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg6(&mut self) -> Cpreg6W<Cpreg6Spec> {
        Cpreg6W::new(self, 0)
    }
}
#[doc = "CPREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg6Spec;
impl crate::RegisterSpec for Cpreg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg6::R`](R) reader structure"]
impl crate::Readable for Cpreg6Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg6::W`](W) writer structure"]
impl crate::Writable for Cpreg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG6 to value 0"]
impl crate::Resettable for Cpreg6Spec {
    const RESET_VALUE: u32 = 0;
}
