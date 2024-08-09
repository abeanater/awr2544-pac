#[doc = "Register `DC_RESERVED_5` reader"]
pub type R = crate::R<DcReserved5Spec>;
#[doc = "Register `DC_RESERVED_5` writer"]
pub type W = crate::W<DcReserved5Spec>;
#[doc = "Field `dc_sub_reserved_5` reader - 31:0\\]
Reserved for future addition"]
pub type DcSubReserved5R = crate::FieldReader<u32>;
#[doc = "Field `dc_sub_reserved_5` writer - 31:0\\]
Reserved for future addition"]
pub type DcSubReserved5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn dc_sub_reserved_5(&self) -> DcSubReserved5R {
        DcSubReserved5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn dc_sub_reserved_5(&mut self) -> DcSubReserved5W<DcReserved5Spec> {
        DcSubReserved5W::new(self, 0)
    }
}
#[doc = "DC_RESERVED_5\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_reserved_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_reserved_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcReserved5Spec;
impl crate::RegisterSpec for DcReserved5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_reserved_5::R`](R) reader structure"]
impl crate::Readable for DcReserved5Spec {}
#[doc = "`write(|w| ..)` method takes [`dc_reserved_5::W`](W) writer structure"]
impl crate::Writable for DcReserved5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_RESERVED_5 to value 0"]
impl crate::Resettable for DcReserved5Spec {
    const RESET_VALUE: u32 = 0;
}
