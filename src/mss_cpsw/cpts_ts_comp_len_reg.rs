#[doc = "Register `CPTS_TS_COMP_LEN_REG` reader"]
pub type R = crate::R<CptsTsCompLenRegSpec>;
#[doc = "Register `CPTS_TS_COMP_LEN_REG` writer"]
pub type W = crate::W<CptsTsCompLenRegSpec>;
#[doc = "Field `TIME_STAMP_COMPARISON` reader - 31:0\\]
Time stamp comparison length"]
pub type TimeStampComparisonR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_COMPARISON` writer - 31:0\\]
Time stamp comparison length"]
pub type TimeStampComparisonW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison length"]
    #[inline(always)]
    pub fn time_stamp_comparison(&self) -> TimeStampComparisonR {
        TimeStampComparisonR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison length"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_comparison(&mut self) -> TimeStampComparisonW<CptsTsCompLenRegSpec> {
        TimeStampComparisonW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_comp_len_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_comp_len_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsCompLenRegSpec;
impl crate::RegisterSpec for CptsTsCompLenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_comp_len_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsCompLenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_comp_len_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsCompLenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_COMP_LEN_REG to value 0"]
impl crate::Resettable for CptsTsCompLenRegSpec {
    const RESET_VALUE: u32 = 0;
}
