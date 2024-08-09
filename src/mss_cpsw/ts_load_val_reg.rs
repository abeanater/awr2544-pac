#[doc = "Register `TS_LOAD_VAL_REG` reader"]
pub type R = crate::R<TsLoadValRegSpec>;
#[doc = "Register `TS_LOAD_VAL_REG` writer"]
pub type W = crate::W<TsLoadValRegSpec>;
#[doc = "Field `TIME_STAMP_LOAD` reader - 31:0\\]
Time stamp load low value"]
pub type TimeStampLoadR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_LOAD` writer - 31:0\\]
Time stamp load low value"]
pub type TimeStampLoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp load low value"]
    #[inline(always)]
    pub fn time_stamp_load(&self) -> TimeStampLoadR {
        TimeStampLoadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp load low value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_load(&mut self) -> TimeStampLoadW<TsLoadValRegSpec> {
        TimeStampLoadW::new(self, 0)
    }
}
#[doc = "Time Stamp Load Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_load_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_load_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsLoadValRegSpec;
impl crate::RegisterSpec for TsLoadValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_load_val_reg::R`](R) reader structure"]
impl crate::Readable for TsLoadValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_load_val_reg::W`](W) writer structure"]
impl crate::Writable for TsLoadValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_LOAD_VAL_REG to value 0"]
impl crate::Resettable for TsLoadValRegSpec {
    const RESET_VALUE: u32 = 0;
}
