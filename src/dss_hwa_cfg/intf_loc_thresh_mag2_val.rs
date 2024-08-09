#[doc = "Register `INTF_LOC_THRESH_MAG2_VAL` reader"]
pub type R = crate::R<IntfLocThreshMag2ValSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAG2_VAL` writer"]
pub type W = crate::W<IntfLocThreshMag2ValSpec>;
#[doc = "Field `intf_loc_thresh_mag2_val` reader - 23:0\\]
Interference magnitude threshold value from Interference stats module ( read only) for bcnt =2"]
pub type IntfLocThreshMag2ValR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_mag2_val` writer - 23:0\\]
Interference magnitude threshold value from Interference stats module ( read only) for bcnt =2"]
pub type IntfLocThreshMag2ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude threshold value from Interference stats module ( read only) for bcnt =2"]
    #[inline(always)]
    pub fn intf_loc_thresh_mag2_val(&self) -> IntfLocThreshMag2ValR {
        IntfLocThreshMag2ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Interference magnitude threshold value from Interference stats module ( read only) for bcnt =2"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_mag2_val(&mut self) -> IntfLocThreshMag2ValW<IntfLocThreshMag2ValSpec> {
        IntfLocThreshMag2ValW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAG2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag2_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag2_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMag2ValSpec;
impl crate::RegisterSpec for IntfLocThreshMag2ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_mag2_val::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMag2ValSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_mag2_val::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMag2ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAG2_VAL to value 0"]
impl crate::Resettable for IntfLocThreshMag2ValSpec {
    const RESET_VALUE: u32 = 0;
}
