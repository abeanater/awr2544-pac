#[doc = "Register `CPREG4` reader"]
pub type R = crate::R<Cpreg4Spec>;
#[doc = "Register `CPREG4` writer"]
pub type W = crate::W<Cpreg4Spec>;
#[doc = "Field `CPREG4` reader - 31:0\\]
Chirp Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg4R = crate::FieldReader<u32>;
#[doc = "Field `CPREG4` writer - 31:0\\]
Chirp Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg4(&self) -> Cpreg4R {
        Cpreg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg4(&mut self) -> Cpreg4W<Cpreg4Spec> {
        Cpreg4W::new(self, 0)
    }
}
#[doc = "CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg4Spec;
impl crate::RegisterSpec for Cpreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg4::R`](R) reader structure"]
impl crate::Readable for Cpreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg4::W`](W) writer structure"]
impl crate::Writable for Cpreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG4 to value 0"]
impl crate::Resettable for Cpreg4Spec {
    const RESET_VALUE: u32 = 0;
}
