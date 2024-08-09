#[doc = "Register `INTF_LOC_THRESH_MAG0_SW` reader"]
pub type R = crate::R<IntfLocThreshMag0SwSpec>;
#[doc = "Register `INTF_LOC_THRESH_MAG0_SW` writer"]
pub type W = crate::W<IntfLocThreshMag0SwSpec>;
#[doc = "Field `intf_loc_thresh_mag0_sw` reader - 23:0\\]
SW programmed interface threshold magnitude for bcnt=0"]
pub type IntfLocThreshMag0SwR = crate::FieldReader<u32>;
#[doc = "Field `intf_loc_thresh_mag0_sw` writer - 23:0\\]
SW programmed interface threshold magnitude for bcnt=0"]
pub type IntfLocThreshMag0SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude for bcnt=0"]
    #[inline(always)]
    pub fn intf_loc_thresh_mag0_sw(&self) -> IntfLocThreshMag0SwR {
        IntfLocThreshMag0SwR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed interface threshold magnitude for bcnt=0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_thresh_mag0_sw(&mut self) -> IntfLocThreshMag0SwW<IntfLocThreshMag0SwSpec> {
        IntfLocThreshMag0SwW::new(self, 0)
    }
}
#[doc = "INTF_LOC_THRESH_MAG0_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_thresh_mag0_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_thresh_mag0_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocThreshMag0SwSpec;
impl crate::RegisterSpec for IntfLocThreshMag0SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_thresh_mag0_sw::R`](R) reader structure"]
impl crate::Readable for IntfLocThreshMag0SwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_thresh_mag0_sw::W`](W) writer structure"]
impl crate::Writable for IntfLocThreshMag0SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_THRESH_MAG0_SW to value 0"]
impl crate::Resettable for IntfLocThreshMag0SwSpec {
    const RESET_VALUE: u32 = 0;
}
