#[doc = "Register `INTF_LOC_THRESH_MAGDIFF3_VAL` reader"]
pub type R = crate::R<IntfLocThreshMagdiff3ValSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAGDIFF3_VAL` writer"]
pub type W = crate::W<IntfLocThreshMagdiff3ValSpec>;
#[doc = "Field `intf_loc_thresh_magdiff3_val` reader - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =3"]
pub type IntfLocThreshMagdiff3ValR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_magdiff3_val` writer - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =3"]
pub type IntfLocThreshMagdiff3ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =3"]
    #[inline(always)]
    pub fn intf_loc_thresh_magdiff3_val(&self) -> IntfLocThreshMagdiff3ValR {
        IntfLocThreshMagdiff3ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude difference threshold value from Interference stats module ( read only) for bcnt =3"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_magdiff3_val(
        &mut self,
    ) -> IntfLocThreshMagdiff3ValW<IntfLocThreshMagdiff3ValSpec> {
        IntfLocThreshMagdiff3ValW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAGDIFF3_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_magdiff3_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_magdiff3_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMagdiff3ValSpec;
impl crate::RegisterSpec for IntfLocThreshMagdiff3ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_magdiff3_val::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMagdiff3ValSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_magdiff3_val::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMagdiff3ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAGDIFF3_VAL to value 0"]
impl crate::Resettable for IntfLocThreshMagdiff3ValSpec {
    const RESET_VALUE: u32 = 0;
}
