#[doc = "Register `TWID_INCR_DELTA_FRAC` reader"]
pub type R = crate::R<TwidIncrDeltaFracSpec>;
#[doc = "Register `TWID_INCR_DELTA_FRAC` writer"]
pub type W = crate::W<TwidIncrDeltaFracSpec>;
#[doc = "Field `twid_incr_delta_frac` reader - 9:0\\]
Used in complex multiplier mode 10 Delta Fractional frequency increment per param-set looping Instantaneous frequency is (TWIDINCR &lt;&lt; 10) +TWID_INCR_DELTA_ FRAC*c, c is current execution count of the parameter set."]
pub type TwidIncrDeltaFracR = crate::FieldReader<u16>;
#[doc = "Field `twid_incr_delta_frac` writer - 9:0\\]
Used in complex multiplier mode 10 Delta Fractional frequency increment per param-set looping Instantaneous frequency is (TWIDINCR &lt;&lt; 10) +TWID_INCR_DELTA_ FRAC*c, c is current execution count of the parameter set."]
pub type TwidIncrDeltaFracW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Used in complex multiplier mode 10 Delta Fractional frequency increment per param-set looping Instantaneous frequency is (TWIDINCR &lt;&lt; 10) +TWID_INCR_DELTA_ FRAC*c, c is current execution count of the parameter set."]
    #[inline(always)]
    pub fn twid_incr_delta_frac(&self) -> TwidIncrDeltaFracR {
        TwidIncrDeltaFracR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Used in complex multiplier mode 10 Delta Fractional frequency increment per param-set looping Instantaneous frequency is (TWIDINCR &lt;&lt; 10) +TWID_INCR_DELTA_ FRAC*c, c is current execution count of the parameter set."]
    #[inline(always)]
    #[must_use]
    pub fn twid_incr_delta_frac(&mut self) -> TwidIncrDeltaFracW<TwidIncrDeltaFracSpec> {
        TwidIncrDeltaFracW::new(self, 0)
    }
}
#[doc = "TWID_INCR_DELTA_FRAC\n\nYou can [`read`](crate::Reg::read) this register and get [`twid_incr_delta_frac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twid_incr_delta_frac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwidIncrDeltaFracSpec;
impl crate::RegisterSpec for TwidIncrDeltaFracSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twid_incr_delta_frac::R`](R) reader structure"]
impl crate::Readable for TwidIncrDeltaFracSpec {}
#[doc = "`write(|w| ..)` method takes [`twid_incr_delta_frac::W`](W) writer structure"]
impl crate::Writable for TwidIncrDeltaFracSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TWID_INCR_DELTA_FRAC to value 0"]
impl crate::Resettable for TwidIncrDeltaFracSpec {
    const RESET_VALUE: u32 = 0;
}
