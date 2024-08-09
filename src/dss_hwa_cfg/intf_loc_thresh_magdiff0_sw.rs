#[doc = "Register `INTF_LOC_THRESH_MAGDIFF0_SW` reader"]
pub type R = crate::R<IntfLocThreshMagdiff0SwSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAGDIFF0_SW` writer"]
pub type W = crate::W<IntfLocThreshMagdiff0SwSpec>;
#[doc = "Field `intf_loc_thresh_magdiff0_sw` reader - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=0"]
pub type IntfLocThreshMagdiff0SwR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_magdiff0_sw` writer - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=0"]
pub type IntfLocThreshMagdiff0SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=0"]
    #[inline(always)]
    pub fn intf_loc_thresh_magdiff0_sw(&self) -> IntfLocThreshMagdiff0SwR {
        IntfLocThreshMagdiff0SwR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude difference for bcnt=0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_magdiff0_sw(
        &mut self,
    ) -> IntfLocThreshMagdiff0SwW<IntfLocThreshMagdiff0SwSpec> {
        IntfLocThreshMagdiff0SwW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAGDIFF0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff0_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff0_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMagdiff0SwSpec;
impl crate::RegisterSpec for IntfLocThreshMagdiff0SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_magdiff0_sw::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMagdiff0SwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_magdiff0_sw::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMagdiff0SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAGDIFF0_SW to value 0"]
impl crate::Resettable for IntfLocThreshMagdiff0SwSpec {
    const RESET_VALUE: u32 = 0;
}
