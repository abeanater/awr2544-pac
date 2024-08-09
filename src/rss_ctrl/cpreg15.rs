#[doc = "Register `CPREG15` reader"]
pub type R = crate::R<Cpreg15Spec>;
#[doc = "Register `CPREG15` writer"]
pub type W = crate::W<Cpreg15Spec>;
#[doc = "Field `CPREG15` reader - 31:0\\]
Chirp Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg15R = crate::FieldReader<u32>;
#[doc = "Field `CPREG15` writer - 31:0\\]
Chirp Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg15(&self) -> Cpreg15R {
        Cpreg15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 15. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg15(&mut self) -> Cpreg15W<Cpreg15Spec> {
        Cpreg15W::new(self, 0)
    }
}
#[doc = "CPREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg15Spec;
impl crate::RegisterSpec for Cpreg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg15::R`](R) reader structure"]
impl crate::Readable for Cpreg15Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg15::W`](W) writer structure"]
impl crate::Writable for Cpreg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG15 to value 0"]
impl crate::Resettable for Cpreg15Spec {
    const RESET_VALUE: u32 = 0;
}
