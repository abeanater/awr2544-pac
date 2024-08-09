#[doc = "Register `CPTS_TS_LOAD_EN_REG` reader"]
pub type R = crate::R<CptsTsLoadEnRegSpec>;
#[doc = "Register `CPTS_TS_LOAD_EN_REG` writer"]
pub type W = crate::W<CptsTsLoadEnRegSpec>;
#[doc = "Field `TIME_STAMP_LOAD` reader - 0:0\\]
Time stamp load enable"]
pub type TimeStampLoadR = crate::BitReader;
#[doc = "Field `TIME_STAMP_LOAD` writer - 0:0\\]
Time stamp load enable"]
pub type TimeStampLoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time stamp load enable"]
    #[inline(always)]
    pub fn time_stamp_load(&self) -> TimeStampLoadR {
        TimeStampLoadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time stamp load enable"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_load(&mut self) -> TimeStampLoadW<CptsTsLoadEnRegSpec> {
        TimeStampLoadW::new(self, 0)
    }
}
#[doc = "Time Stamp Load Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_load_en_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_load_en_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsLoadEnRegSpec;
impl crate::RegisterSpec for CptsTsLoadEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_load_en_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsLoadEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_load_en_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsLoadEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_LOAD_EN_REG to value 0"]
impl crate::Resettable for CptsTsLoadEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
