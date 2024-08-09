#[doc = "Register `CPTS_TS_CONFIG` reader"]
pub type R = crate::R<CptsTsConfigSpec>;
#[doc = "Register `CPTS_TS_CONFIG` writer"]
pub type W = crate::W<CptsTsConfigSpec>;
#[doc = "Field `THE_NUMBER_OF` reader - 7:0\\]
The number of CPTS GENF outputs"]
pub type TheNumberOfR = crate::FieldReader;
#[doc = "Field `THE_NUMBER_OF` writer - 7:0\\]
The number of CPTS GENF outputs"]
pub type TheNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_EVENT_FIFO` reader - 15:8\\]
The Event FIFO Depth"]
pub type TheEventFifoR = crate::FieldReader;
#[doc = "Field `THE_EVENT_FIFO` writer - 15:8\\]
The Event FIFO Depth"]
pub type TheEventFifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Event FIFO Depth"]
    #[inline(always)]
    pub fn the_event_fifo(&self) -> TheEventFifoR {
        TheEventFifoR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<CptsTsConfigSpec> {
        TheNumberOfW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Event FIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn the_event_fifo(&mut self) -> TheEventFifoW<CptsTsConfigSpec> {
        TheEventFifoW::new(self, 8)
    }
}
#[doc = "Time Stamp Configuration Read\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsConfigSpec;
impl crate::RegisterSpec for CptsTsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_config::R`](R) reader structure"]
impl crate::Readable for CptsTsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_config::W`](W) writer structure"]
impl crate::Writable for CptsTsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_CONFIG to value 0"]
impl crate::Resettable for CptsTsConfigSpec {
    const RESET_VALUE: u32 = 0;
}
