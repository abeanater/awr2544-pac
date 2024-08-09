#[doc = "Register `DC_EST_CTRL` reader"]
pub type R = crate::R<DcEstCtrlSpec>;
#[doc = "Register `DC_EST_CTRL` writer"]
pub type W = crate::W<DcEstCtrlSpec>;
#[doc = "Field `dc_est_scale` reader - 8:0\\]
9-bit scale applied to all accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcEstScaleR = crate::FieldReader<u16>;
#[doc = "Field `dc_est_scale` writer - 8:0\\]
9-bit scale applied to all accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
pub type DcEstScaleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `dc_est_shift` reader - 19:16\\]
Programmable shift applied to all 12 accumulator outputs. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
pub type DcEstShiftR = crate::FieldReader;
#[doc = "Field `dc_est_shift` writer - 19:16\\]
Programmable shift applied to all 12 accumulator outputs. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
pub type DcEstShiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    pub fn dc_est_scale(&self) -> DcEstScaleR {
        DcEstScaleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Programmable shift applied to all 12 accumulator outputs. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
    #[inline(always)]
    pub fn dc_est_shift(&self) -> DcEstShiftR {
        DcEstShiftR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
9-bit scale applied to all accumulators. Multiplies the accumulator output by DCEST_SCALE/256.This is followed by right shift and truncation. Default value is 256 giving a scale of 1.0. Setting it to 128, gives a scale of 0.5"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_scale(&mut self) -> DcEstScaleW<DcEstCtrlSpec> {
        DcEstScaleW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Programmable shift applied to all 12 accumulator outputs. Cannot be bypassed. Output shifted by 2^(8 + 6+DCEST_SHIFT). For DCEST_SHIFT = 15 also gives 2^(28) and not 29 (saturate at 28)"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_shift(&mut self) -> DcEstShiftW<DcEstCtrlSpec> {
        DcEstShiftW::new(self, 16)
    }
}
#[doc = "DC_EST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstCtrlSpec;
impl crate::RegisterSpec for DcEstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_ctrl::R`](R) reader structure"]
impl crate::Readable for DcEstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_ctrl::W`](W) writer structure"]
impl crate::Writable for DcEstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_CTRL to value 0"]
impl crate::Resettable for DcEstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
