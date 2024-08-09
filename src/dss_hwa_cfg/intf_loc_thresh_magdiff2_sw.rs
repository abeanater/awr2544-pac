#[doc = "Register `INTF_LOC_THRESH_MAGDIFF2_SW` reader"]
pub type R = crate::R<IntfLocThreshMagdiff2SwSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAGDIFF2_SW` writer"]
pub type W = crate::W<IntfLocThreshMagdiff2SwSpec>;
#[doc = "Field `intf_loc_thresh_magdiff2_sw` reader - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=2"]
pub type IntfLocThreshMagdiff2SwR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_magdiff2_sw` writer - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=2"]
pub type IntfLocThreshMagdiff2SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=2"]
    #[inline(always)]
    pub fn intf_loc_thresh_magdiff2_sw(&self) -> IntfLocThreshMagdiff2SwR {
        IntfLocThreshMagdiff2SwR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=2"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_magdiff2_sw(
        &mut self,
    ) -> IntfLocThreshMagdiff2SwW<IntfLocThreshMagdiff2SwSpec> {
        IntfLocThreshMagdiff2SwW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAGDIFF2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff2_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff2_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMagdiff2SwSpec;
impl crate::RegisterSpec for IntfLocThreshMagdiff2SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_magdiff2_sw::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMagdiff2SwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_magdiff2_sw::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMagdiff2SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAGDIFF2_SW to value 0"]
impl crate::Resettable for IntfLocThreshMagdiff2SwSpec {
    const RESET_VALUE: u32 = 0;
}
