#[doc = "Register `CPREG11` reader"]
pub type R = crate::R<Cpreg11Spec>;
#[doc = "Register `CPREG11` writer"]
pub type W = crate::W<Cpreg11Spec>;
#[doc = "Field `CPREG11` reader - 31:0\\]
Chirp Parameters Register 11. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg11R = crate::FieldReader<u32>;
#[doc = "Field `CPREG11` writer - 31:0\\]
Chirp Parameters Register 11. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 11. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg11(&self) -> Cpreg11R {
        Cpreg11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 11. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg11(&mut self) -> Cpreg11W<Cpreg11Spec> {
        Cpreg11W::new(self, 0)
    }
}
#[doc = "CPREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg11Spec;
impl crate::RegisterSpec for Cpreg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg11::R`](R) reader structure"]
impl crate::Readable for Cpreg11Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg11::W`](W) writer structure"]
impl crate::Writable for Cpreg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG11 to value 0"]
impl crate::Resettable for Cpreg11Spec {
    const RESET_VALUE: u32 = 0;
}
