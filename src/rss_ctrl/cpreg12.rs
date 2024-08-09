#[doc = "Register `CPREG12` reader"]
pub type R = crate::R<Cpreg12Spec>;
#[doc = "Register `CPREG12` writer"]
pub type W = crate::W<Cpreg12Spec>;
#[doc = "Field `CPREG12` reader - 31:0\\]
Chirp Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg12R = crate::FieldReader<u32>;
#[doc = "Field `CPREG12` writer - 31:0\\]
Chirp Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Cpreg12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn cpreg12(&self) -> Cpreg12R {
        Cpreg12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Chirp Parameters Register 12. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn cpreg12(&mut self) -> Cpreg12W<Cpreg12Spec> {
        Cpreg12W::new(self, 0)
    }
}
#[doc = "CPREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`cpreg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpreg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpreg12Spec;
impl crate::RegisterSpec for Cpreg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpreg12::R`](R) reader structure"]
impl crate::Readable for Cpreg12Spec {}
#[doc = "`write(|w| ..)` method takes [`cpreg12::W`](W) writer structure"]
impl crate::Writable for Cpreg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPREG12 to value 0"]
impl crate::Resettable for Cpreg12Spec {
    const RESET_VALUE: u32 = 0;
}
