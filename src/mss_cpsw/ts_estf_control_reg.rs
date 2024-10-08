#[doc = "Register `TS_ESTF_CONTROL_REG` reader"]
pub type R = crate::R<TsEstfControlRegSpec>;
#[doc = "Register `TS_ESTF_CONTROL_REG` writer"]
pub type W = crate::W<TsEstfControlRegSpec>;
#[doc = "Field `TIME_STAMP_ESTF_1` reader - 0:0\\]
Time Stamp ESTF Generate Function PPM Direction"]
pub type TimeStampEstf1R = crate::BitReader;
#[doc = "Field `TIME_STAMP_ESTF_1` writer - 0:0\\]
Time Stamp ESTF Generate Function PPM Direction"]
pub type TimeStampEstf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_STAMP_ESTF` reader - 1:1\\]
Time Stamp ESTF Generate Function Polarity Invert"]
pub type TimeStampEstfR = crate::BitReader;
#[doc = "Field `TIME_STAMP_ESTF` writer - 1:1\\]
Time Stamp ESTF Generate Function Polarity Invert"]
pub type TimeStampEstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp ESTF Generate Function PPM Direction"]
    #[inline(always)]
    pub fn time_stamp_estf_1(&self) -> TimeStampEstf1R {
        TimeStampEstf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp ESTF Generate Function Polarity Invert"]
    #[inline(always)]
    pub fn time_stamp_estf(&self) -> TimeStampEstfR {
        TimeStampEstfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp ESTF Generate Function PPM Direction"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_estf_1(&mut self) -> TimeStampEstf1W<TsEstfControlRegSpec> {
        TimeStampEstf1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp ESTF Generate Function Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_estf(&mut self) -> TimeStampEstfW<TsEstfControlRegSpec> {
        TimeStampEstfW::new(self, 1)
    }
}
#[doc = "Time Stamp ESTF Generate Function Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsEstfControlRegSpec;
impl crate::RegisterSpec for TsEstfControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_estf_control_reg::R`](R) reader structure"]
impl crate::Readable for TsEstfControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_estf_control_reg::W`](W) writer structure"]
impl crate::Writable for TsEstfControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_ESTF_CONTROL_REG to value 0"]
impl crate::Resettable for TsEstfControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
