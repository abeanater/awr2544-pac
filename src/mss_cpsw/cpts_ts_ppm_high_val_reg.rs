#[doc = "Register `CPTS_TS_PPM_HIGH_VAL_REG` reader"]
pub type R = crate::R<CptsTsPpmHighValRegSpec>;
#[doc = "Register `CPTS_TS_PPM_HIGH_VAL_REG` writer"]
pub type W = crate::W<CptsTsPpmHighValRegSpec>;
#[doc = "Field `TIME_STAMP_PPM` reader - 9:0\\]
Time stamp PPM High value"]
pub type TimeStampPpmR = crate::FieldReader<u16>;
#[doc = "Field `TIME_STAMP_PPM` writer - 9:0\\]
Time stamp PPM High value"]
pub type TimeStampPpmW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time stamp PPM High value"]
    #[inline(always)]
    pub fn time_stamp_ppm(&self) -> TimeStampPpmR {
        TimeStampPpmR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time stamp PPM High value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_ppm(&mut self) -> TimeStampPpmW<CptsTsPpmHighValRegSpec> {
        TimeStampPpmW::new(self, 0)
    }
}
#[doc = "Time Stamp PPM High Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_ppm_high_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_ppm_high_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsPpmHighValRegSpec;
impl crate::RegisterSpec for CptsTsPpmHighValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_ppm_high_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsPpmHighValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_ppm_high_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsPpmHighValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_PPM_HIGH_VAL_REG to value 0"]
impl crate::Resettable for CptsTsPpmHighValRegSpec {
    const RESET_VALUE: u32 = 0;
}
