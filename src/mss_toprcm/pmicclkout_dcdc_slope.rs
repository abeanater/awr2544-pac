#[doc = "Register `PMICCLKOUT_DCDC_SLOPE` reader"]
pub type R = crate::R<PmicclkoutDcdcSlopeSpec>;
#[doc = "Register `PMICCLKOUT_DCDC_SLOPE` writer"]
pub type W = crate::W<PmicclkoutDcdcSlopeSpec>;
#[doc = "Field `slope_val` reader - 26:0\\]
PMIC Clockout DCDC Slope Config Value"]
pub type SlopeValR = crate::FieldReader<u32>;
#[doc = "Field `slope_val` writer - 26:0\\]
PMIC Clockout DCDC Slope Config Value"]
pub type SlopeValW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - 26:0\\]
PMIC Clockout DCDC Slope Config Value"]
    #[inline(always)]
    pub fn slope_val(&self) -> SlopeValR {
        SlopeValR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - 26:0\\]
PMIC Clockout DCDC Slope Config Value"]
    #[inline(always)]
    #[must_use]
    pub fn slope_val(&mut self) -> SlopeValW<PmicclkoutDcdcSlopeSpec> {
        SlopeValW::new(self, 0)
    }
}
#[doc = "PMICCLKOUT_DCDC_SLOPE\n\nYou can [`read`](crate::Reg::read) this register and get [`pmicclkout_dcdc_slope::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmicclkout_dcdc_slope::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmicclkoutDcdcSlopeSpec;
impl crate::RegisterSpec for PmicclkoutDcdcSlopeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmicclkout_dcdc_slope::R`](R) reader structure"]
impl crate::Readable for PmicclkoutDcdcSlopeSpec {}
#[doc = "`write(|w| ..)` method takes [`pmicclkout_dcdc_slope::W`](W) writer structure"]
impl crate::Writable for PmicclkoutDcdcSlopeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMICCLKOUT_DCDC_SLOPE to value 0"]
impl crate::Resettable for PmicclkoutDcdcSlopeSpec {
    const RESET_VALUE: u32 = 0;
}
