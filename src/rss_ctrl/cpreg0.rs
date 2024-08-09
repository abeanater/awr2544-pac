#[doc = "Register `CPREG0` reader"]
pub type R = crate::R<Cpreg0Spec>;
#[doc = "Register `CPREG0` writer"]
pub type W = crate::W<Cpreg0Spec>;
#[doc = "Field `CPREG0` reader - 31:0\\]
Chirp Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg0R = crate::FieldReader<u32>;
#[doc = "Field `CPREG0` writer - 31:0\\]
Chirp Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg0(&self) -> Cpreg0R {
        Cpreg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg0(&mut self) -> Cpreg0W<Cpreg0Spec> {
        Cpreg0W::new(self, 0)
    }
}
#[doc = "CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg0Spec;
impl crate::RegisterSpec for Cpreg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg0::R`](R) reader structure"]
impl crate::Readable for Cpreg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg0::W`](W) writer structure"]
impl crate::Writable for Cpreg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG0 to value 0"]
impl crate::Resettable for Cpreg0Spec {
    const RESET_VALUE: u32 = 0;
}
