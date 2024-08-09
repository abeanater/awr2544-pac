#[doc = "Register `CPREG13` reader"]
pub type R = crate::R<Cpreg13Spec>;
#[doc = "Register `CPREG13` writer"]
pub type W = crate::W<Cpreg13Spec>;
#[doc = "Field `CPREG13` reader - 31:0\\]
Chirp Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg13R = crate::FieldReader<u32>;
#[doc = "Field `CPREG13` writer - 31:0\\]
Chirp Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg13(&self) -> Cpreg13R {
        Cpreg13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 13. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg13(&mut self) -> Cpreg13W<Cpreg13Spec> {
        Cpreg13W::new(self, 0)
    }
}
#[doc = "CPREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg13Spec;
impl crate::RegisterSpec for Cpreg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg13::R`](R) reader structure"]
impl crate::Readable for Cpreg13Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg13::W`](W) writer structure"]
impl crate::Writable for Cpreg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG13 to value 0"]
impl crate::Resettable for Cpreg13Spec {
    const RESET_VALUE: u32 = 0;
}
