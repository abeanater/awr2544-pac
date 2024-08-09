#[doc = "Register `DC_EST_CTRL_PROFILE2` reader"]
pub type R = crate::R<DcEstCtrlProfile2Spec>;
#[doc = "Register `DC_EST_CTRL_PROFILE2` writer"]
pub type W = crate::W<DcEstCtrlProfile2Spec>;
#[doc = "Field `dc_est_scale_profile2` reader - 8:0\\]
9-bit scale applied to all accumulators for second chirp profile. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcEstScaleProfile2R = crate::FieldReader<u16>;
#[doc = "Field `dc_est_scale_profile2` writer - 8:0\\]
9-bit scale applied to all accumulators for second chirp profile. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcEstScaleProfile2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `dc_est_shift_profile2` reader - 19:16\\]
Programmable shift applied to all accumulator outputs of second chirp profile. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
pub type DcEstShiftProfile2R = crate::FieldReader;
#[doc = "Field `dc_est_shift_profile2` writer - 19:16\\]
Programmable shift applied to all accumulator outputs of second chirp profile. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
pub type DcEstShiftProfile2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all accumulators for second chirp profile. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    pub fn dc_est_scale_profile2(&self) -> DcEstScaleProfile2R {
        DcEstScaleProfile2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Programmable shift applied to all accumulator outputs of second chirp profile. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
    #[inline(always)]
    pub fn dc_est_shift_profile2(&self) -> DcEstShiftProfile2R {
        DcEstShiftProfile2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all accumulators for second chirp profile. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_scale_profile2(&mut self) -> DcEstScaleProfile2W<DcEstCtrlProfile2Spec> {
        DcEstScaleProfile2W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Programmable shift applied to all accumulator outputs of second chirp profile. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_shift_profile2(&mut self) -> DcEstShiftProfile2W<DcEstCtrlProfile2Spec> {
        DcEstShiftProfile2W::new(self, 16)
    }
}
#[doc = "DC_EST_CTRL_PROFILE2\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_ctrl_profile2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_ctrl_profile2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstCtrlProfile2Spec;
impl crate::RegisterSpec for DcEstCtrlProfile2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_ctrl_profile2::R`](R) reader structure"]
impl crate::Readable for DcEstCtrlProfile2Spec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_ctrl_profile2::W`](W) writer structure"]
impl crate::Writable for DcEstCtrlProfile2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_CTRL_PROFILE2 to value 0"]
impl crate::Resettable for DcEstCtrlProfile2Spec {
    const RESET_VALUE: u32 = 0;
}
