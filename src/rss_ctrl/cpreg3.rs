#[doc = "Register `CPREG3` reader"]
pub type R = crate::R<Cpreg3Spec>;
#[doc = "Register `CPREG3` writer"]
pub type W = crate::W<Cpreg3Spec>;
#[doc = "Field `CPREG3` reader - 31:0\\]
Chirp Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg3R = crate::FieldReader<u32>;
#[doc = "Field `CPREG3` writer - 31:0\\]
Chirp Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg3(&self) -> Cpreg3R {
        Cpreg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg3(&mut self) -> Cpreg3W<Cpreg3Spec> {
        Cpreg3W::new(self, 0)
    }
}
#[doc = "CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg3Spec;
impl crate::RegisterSpec for Cpreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg3::R`](R) reader structure"]
impl crate::Readable for Cpreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg3::W`](W) writer structure"]
impl crate::Writable for Cpreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG3 to value 0"]
impl crate::Resettable for Cpreg3Spec {
    const RESET_VALUE: u32 = 0;
}
