#[doc = "Register `CPTS_INTSTAT_RAW_REG` reader"]
pub type R = crate::R<CptsIntstatRawRegSpec>;
#[doc = "Register `CPTS_INTSTAT_RAW_REG` writer"]
pub type W = crate::W<CptsIntstatRawRegSpec>;
#[doc = "Field `TS_PEND_RAW_INT_READ` reader - 0:0\\]
TS_PEND_RAW int read (before enable)"]
pub type TsPendRawIntReadR = crate::BitReader;
#[doc = "Field `TS_PEND_RAW_INT_READ` writer - 0:0\\]
TS_PEND_RAW int read (before enable)"]
pub type TsPendRawIntReadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND_RAW int read (before enable)"]
    #[inline(always)]
    pub fn ts_pend_raw_int_read(&self) -> TsPendRawIntReadR {
        TsPendRawIntReadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND_RAW int read (before enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend_raw_int_read(&mut self) -> TsPendRawIntReadW<CptsIntstatRawRegSpec> {
        TsPendRawIntReadW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_intstat_raw_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_intstat_raw_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsIntstatRawRegSpec;
impl crate::RegisterSpec for CptsIntstatRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_intstat_raw_reg::R`](R) reader structure"]
impl crate::Readable for CptsIntstatRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_intstat_raw_reg::W`](W) writer structure"]
impl crate::Writable for CptsIntstatRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_INTSTAT_RAW_REG to value 0"]
impl crate::Resettable for CptsIntstatRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
