#[doc = "Register `CMULT_RESERVED_2` reader"]
pub type R = crate::R<CmultReserved2Spec>;
#[doc = "Register `CMULT_RESERVED_2` writer"]
pub type W = crate::W<CmultReserved2Spec>;
#[doc = "Field `cmult_reserved_2` reader - 31:0\\]
Reserved for future addition"]
pub type CmultReserved2R = crate::FieldReader<u32>;
#[doc = "Field `cmult_reserved_2` writer - 31:0\\]
Reserved for future addition"]
pub type CmultReserved2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn cmult_reserved_2(&self) -> CmultReserved2R {
        CmultReserved2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn cmult_reserved_2(&mut self) -> CmultReserved2W<CmultReserved2Spec> {
        CmultReserved2W::new(self, 0)
    }
}
#[doc = "CMULT_RESERVED_2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmult_reserved_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmult_reserved_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmultReserved2Spec;
impl crate::RegisterSpec for CmultReserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmult_reserved_2::R`](R) reader structure"]
impl crate::Readable for CmultReserved2Spec {}
#[doc = "`write(|w| ..)` method takes [`cmult_reserved_2::W`](W) writer structure"]
impl crate::Writable for CmultReserved2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMULT_RESERVED_2 to value 0"]
impl crate::Resettable for CmultReserved2Spec {
    const RESET_VALUE: u32 = 0;
}
