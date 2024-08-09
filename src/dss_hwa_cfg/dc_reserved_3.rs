#[doc = "Register `DC_RESERVED_3` reader"]
pub type R = crate::R<DcReserved3Spec>;
#[doc = "Register `DC_RESERVED_3` writer"]
pub type W = crate::W<DcReserved3Spec>;
#[doc = "Field `dc_sub_reserved_3` reader - 31:0\\]
Reserved for future addition"]
pub type DcSubReserved3R = crate::FieldReader<u32>;
#[doc = "Field `dc_sub_reserved_3` writer - 31:0\\]
Reserved for future addition"]
pub type DcSubReserved3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn dc_sub_reserved_3(&self) -> DcSubReserved3R {
        DcSubReserved3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn dc_sub_reserved_3(&mut self) -> DcSubReserved3W<DcReserved3Spec> {
        DcSubReserved3W::new(self, 0)
    }
}
#[doc = "DC_RESERVED_3\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_reserved_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_reserved_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcReserved3Spec;
impl crate::RegisterSpec for DcReserved3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_reserved_3::R`](R) reader structure"]
impl crate::Readable for DcReserved3Spec {}
#[doc = "`write(|w| ..)` method takes [`dc_reserved_3::W`](W) writer structure"]
impl crate::Writable for DcReserved3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_RESERVED_3 to value 0"]
impl crate::Resettable for DcReserved3Spec {
    const RESET_VALUE: u32 = 0;
}
