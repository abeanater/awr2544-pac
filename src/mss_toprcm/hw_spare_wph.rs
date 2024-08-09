#[doc = "Register `HW_SPARE_WPH` reader"]
pub type R = crate::R<HwSpareWphSpec>;
#[doc = "Register `HW_SPARE_WPH` writer"]
pub type W = crate::W<HwSpareWphSpec>;
#[doc = "Field `hw_spare_wph` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareWphR = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_wph` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareWphW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_wph(&self) -> HwSpareWphR {
        HwSpareWphR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_wph(&mut self) -> HwSpareWphW<HwSpareWphSpec> {
        HwSpareWphW::new(self, 0)
    }
}
#[doc = "HW_SPARE_WPH\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_wph::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_wph::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareWphSpec;
impl crate::RegisterSpec for HwSpareWphSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_wph::R`](R) reader structure"]
impl crate::Readable for HwSpareWphSpec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_wph::W`](W) writer structure"]
impl crate::Writable for HwSpareWphSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_WPH to value 0"]
impl crate::Resettable for HwSpareWphSpec {
    const RESET_VALUE: u32 = 0;
}
