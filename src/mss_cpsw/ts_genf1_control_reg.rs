#[doc = "Register `TS_GENF1_CONTROL_REG` reader"]
pub type R = crate::R<TsGenf1ControlRegSpec>;
#[doc = "Register `TS_GENF1_CONTROL_REG` writer"]
pub type W = crate::W<TsGenf1ControlRegSpec>;
#[doc = "Field `TIME_STAMP_GENERATE` reader - 0:0\\]
Time Stamp Generate Function PPM Direction"]
pub type TimeStampGenerateR = crate::BitReader;
#[doc = "Field `TIME_STAMP_GENERATE` writer - 0:0\\]
Time Stamp Generate Function PPM Direction"]
pub type TimeStampGenerateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_STAMP_GENERATE` reader - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
pub type TimeStampGenerateR = crate::BitReader;
#[doc = "Field `TIME_STAMP_GENERATE` writer - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
pub type TimeStampGenerateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp Generate Function PPM Direction"]
    #[inline(always)]
    pub fn time_stamp_generate(&self) -> TimeStampGenerateR {
        TimeStampGenerateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
    #[inline(always)]
    pub fn time_stamp_generate(&self) -> TimeStampGenerateR {
        TimeStampGenerateR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp Generate Function PPM Direction"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_generate(&mut self) -> TimeStampGenerateW<TsGenf1ControlRegSpec> {
        TimeStampGenerateW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_generate(&mut self) -> TimeStampGenerateW<TsGenf1ControlRegSpec> {
        TimeStampGenerateW::new(self, 1)
    }
}
#[doc = "Time Stamp Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf1_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf1_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsGenf1ControlRegSpec;
impl crate::RegisterSpec for TsGenf1ControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_genf1_control_reg::R`](R) reader structure"]
impl crate::Readable for TsGenf1ControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_genf1_control_reg::W`](W) writer structure"]
impl crate::Writable for TsGenf1ControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_GENF1_CONTROL_REG to value 0"]
impl crate::Resettable for TsGenf1ControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
