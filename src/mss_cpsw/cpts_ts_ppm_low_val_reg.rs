#[doc = "Register `CPTS_TS_PPM_LOW_VAL_REG` reader"]
pub type R = crate::R<CptsTsPpmLowValRegSpec>;
#[doc = "Register `CPTS_TS_PPM_LOW_VAL_REG` writer"]
pub type W = crate::W<CptsTsPpmLowValRegSpec>;
#[doc = "Field `TIME_STAMP_PPM` reader - 31:0\\]
Time stamp PPM Low value"]
pub type TimeStampPpmR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_PPM` writer - 31:0\\]
Time stamp PPM Low value"]
pub type TimeStampPpmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp PPM Low value"]
    #[inline(always)]
    pub fn time_stamp_ppm(&self) -> TimeStampPpmR {
        TimeStampPpmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp PPM Low value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_ppm(&mut self) -> TimeStampPpmW<CptsTsPpmLowValRegSpec> {
        TimeStampPpmW::new(self, 0)
    }
}
#[doc = "Time Stamp PPM Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_ppm_low_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_ppm_low_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsPpmLowValRegSpec;
impl crate::RegisterSpec for CptsTsPpmLowValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_ppm_low_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsPpmLowValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_ppm_low_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsPpmLowValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_PPM_LOW_VAL_REG to value 0"]
impl crate::Resettable for CptsTsPpmLowValRegSpec {
    const RESET_VALUE: u32 = 0;
}
