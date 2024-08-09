#[doc = "Register `INTF_LOC_THRESH_MAGDIFF0_VAL` reader"]
pub type R = crate::R<IntfLocThreshMagdiff0ValSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAGDIFF0_VAL` writer"]
pub type W = crate::W<IntfLocThreshMagdiff0ValSpec>;
#[doc = "Field `intf_loc_thresh_magdiff0_val` reader - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =0"]
pub type IntfLocThreshMagdiff0ValR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_magdiff0_val` writer - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =0"]
pub type IntfLocThreshMagdiff0ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =0"]
    #[inline(always)]
    pub fn intf_loc_thresh_magdiff0_val(&self) -> IntfLocThreshMagdiff0ValR {
        IntfLocThreshMagdiff0ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_magdiff0_val(
        &mut self,
    ) -> IntfLocThreshMagdiff0ValW<IntfLocThreshMagdiff0ValSpec> {
        IntfLocThreshMagdiff0ValW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAGDIFF0_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff0_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff0_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMagdiff0ValSpec;
impl crate::RegisterSpec for IntfLocThreshMagdiff0ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_magdiff0_val::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMagdiff0ValSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_magdiff0_val::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMagdiff0ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAGDIFF0_VAL to value 0"]
impl crate::Resettable for IntfLocThreshMagdiff0ValSpec {
    const RESET_VALUE: u32 = 0;
}
