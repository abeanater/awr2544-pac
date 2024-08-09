#[doc = "Register `FFT_RESERVED_1` reader"]
pub type R = crate::R<FftReserved1Spec>;
#[doc = "Register `FFT_RESERVED_1` writer"]
pub type W = crate::W<FftReserved1Spec>;
#[doc = "Field `fft_reserved_1` reader - 31:0\\]
Reserved for future addition"]
pub type FftReserved1R = crate::FieldReader<u32>;
#[doc = "Field `fft_reserved_1` writer - 31:0\\]
Reserved for future addition"]
pub type FftReserved1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn fft_reserved_1(&self) -> FftReserved1R {
        FftReserved1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn fft_reserved_1(&mut self) -> FftReserved1W<FftReserved1Spec> {
        FftReserved1W::new(self, 0)
    }
}
#[doc = "FFT_RESERVED_1\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftReserved1Spec;
impl crate::RegisterSpec for FftReserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fft_reserved_1::R`](R) reader structure"]
impl crate::Readable for FftReserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`fft_reserved_1::W`](W) writer structure"]
impl crate::Writable for FftReserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFT_RESERVED_1 to value 0"]
impl crate::Resettable for FftReserved1Spec {
    const RESET_VALUE: u32 = 0;
}
