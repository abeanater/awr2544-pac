#[doc = "Register `TS_GENF0_PPM_HIGH_REG` reader"]
pub type R = crate::R<TsGenf0PpmHighRegSpec>;
#[doc = "Register `TS_GENF0_PPM_HIGH_REG` writer"]
pub type W = crate::W<TsGenf0PpmHighRegSpec>;
#[doc = "Field `TIME_STAMP_GENERATE` reader - 9:0\\]
Time Stamp Generate Function PPM High Value"]
pub type TimeStampGenerateR = crate::FieldReader<u16>;
#[doc = "Field `TIME_STAMP_GENERATE` writer - 9:0\\]
Time Stamp Generate Function PPM High Value"]
pub type TimeStampGenerateW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub fn time_stamp_generate(&self) -> TimeStampGenerateR {
        TimeStampGenerateR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_generate(&mut self) -> TimeStampGenerateW<TsGenf0PpmHighRegSpec> {
        TimeStampGenerateW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf0_ppm_high_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf0_ppm_high_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsGenf0PpmHighRegSpec;
impl crate::RegisterSpec for TsGenf0PpmHighRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_genf0_ppm_high_reg::R`](R) reader structure"]
impl crate::Readable for TsGenf0PpmHighRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_genf0_ppm_high_reg::W`](W) writer structure"]
impl crate::Writable for TsGenf0PpmHighRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_GENF0_PPM_HIGH_REG to value 0"]
impl crate::Resettable for TsGenf0PpmHighRegSpec {
    const RESET_VALUE: u32 = 0;
}
