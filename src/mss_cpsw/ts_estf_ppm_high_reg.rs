#[doc = "Register `TS_ESTF_PPM_HIGH_REG` reader"]
pub type R = crate::R<TsEstfPpmHighRegSpec>;
#[doc = "Register `TS_ESTF_PPM_HIGH_REG` writer"]
pub type W = crate::W<TsEstfPpmHighRegSpec>;
#[doc = "Field `TIME_STAMP_ESTF` reader - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
pub type TimeStampEstfR = crate::FieldReader<u16>;
#[doc = "Field `TIME_STAMP_ESTF` writer - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
pub type TimeStampEstfW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    pub fn time_stamp_estf(&self) -> TimeStampEstfR {
        TimeStampEstfR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_estf(&mut self) -> TimeStampEstfW<TsEstfPpmHighRegSpec> {
        TimeStampEstfW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_ppm_high_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_ppm_high_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsEstfPpmHighRegSpec;
impl crate::RegisterSpec for TsEstfPpmHighRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_estf_ppm_high_reg::R`](R) reader structure"]
impl crate::Readable for TsEstfPpmHighRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_estf_ppm_high_reg::W`](W) writer structure"]
impl crate::Writable for TsEstfPpmHighRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_ESTF_PPM_HIGH_REG to value 0"]
impl crate::Resettable for TsEstfPpmHighRegSpec {
    const RESET_VALUE: u32 = 0;
}
