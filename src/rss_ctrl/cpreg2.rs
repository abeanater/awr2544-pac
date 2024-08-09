#[doc = "Register `CPREG2` reader"]
pub type R = crate::R<Cpreg2Spec>;
#[doc = "Register `CPREG2` writer"]
pub type W = crate::W<Cpreg2Spec>;
#[doc = "Field `CPREG2` reader - 31:0\\]
Chirp Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg2R = crate::FieldReader<u32>;
#[doc = "Field `CPREG2` writer - 31:0\\]
Chirp Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg2(&self) -> Cpreg2R {
        Cpreg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg2(&mut self) -> Cpreg2W<Cpreg2Spec> {
        Cpreg2W::new(self, 0)
    }
}
#[doc = "CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg2Spec;
impl crate::RegisterSpec for Cpreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg2::R`](R) reader structure"]
impl crate::Readable for Cpreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg2::W`](W) writer structure"]
impl crate::Writable for Cpreg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG2 to value 0"]
impl crate::Resettable for Cpreg2Spec {
    const RESET_VALUE: u32 = 0;
}
