#[doc = "Register `CPREG1` reader"]
pub type R = crate::R<Cpreg1Spec>;
#[doc = "Register `CPREG1` writer"]
pub type W = crate::W<Cpreg1Spec>;
#[doc = "Field `CPREG1` reader - 31:0\\]
Chirp Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg1R = crate::FieldReader<u32>;
#[doc = "Field `CPREG1` writer - 31:0\\]
Chirp Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg1(&self) -> Cpreg1R {
        Cpreg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg1(&mut self) -> Cpreg1W<Cpreg1Spec> {
        Cpreg1W::new(self, 0)
    }
}
#[doc = "CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg1Spec;
impl crate::RegisterSpec for Cpreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg1::R`](R) reader structure"]
impl crate::Readable for Cpreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg1::W`](W) writer structure"]
impl crate::Writable for Cpreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG1 to value 0"]
impl crate::Resettable for Cpreg1Spec {
    const RESET_VALUE: u32 = 0;
}
