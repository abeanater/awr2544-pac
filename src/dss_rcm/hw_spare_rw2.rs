#[doc = "Register `HW_SPARE_RW2` reader"]
pub type R = crate::R<HwSpareRw2Spec>;
#[doc = "Register `HW_SPARE_RW2` writer"]
pub type W = crate::W<HwSpareRw2Spec>;
#[doc = "Field `hw_spare_rw2` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw2R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_rw2` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_rw2(&self) -> HwSpareRw2R {
        HwSpareRw2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_rw2(&mut self) -> HwSpareRw2W<HwSpareRw2Spec> {
        HwSpareRw2W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RW2\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRw2Spec;
impl crate::RegisterSpec for HwSpareRw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_rw2::R`](R) reader structure"]
impl crate::Readable for HwSpareRw2Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_rw2::W`](W) writer structure"]
impl crate::Writable for HwSpareRw2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RW2 to value 0"]
impl crate::Resettable for HwSpareRw2Spec {
    const RESET_VALUE: u32 = 0;
}
