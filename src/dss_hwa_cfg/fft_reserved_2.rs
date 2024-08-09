#[doc = "Register `FFT_RESERVED_2` reader"]
pub type R = crate::R<FftReserved2Spec>;
#[doc = "Register `FFT_RESERVED_2` writer"]
pub type W = crate::W<FftReserved2Spec>;
#[doc = "Field `fft_reserved_2` reader - 31:0\\]
Reserved for future addition"]
pub type FftReserved2R = crate::FieldReader<u32>;
#[doc = "Field `fft_reserved_2` writer - 31:0\\]
Reserved for future addition"]
pub type FftReserved2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn fft_reserved_2(&self) -> FftReserved2R {
        FftReserved2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn fft_reserved_2(&mut self) -> FftReserved2W<FftReserved2Spec> {
        FftReserved2W::new(self, 0)
    }
}
#[doc = "FFT_RESERVED_2\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftReserved2Spec;
impl crate::RegisterSpec for FftReserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fft_reserved_2::R`](R) reader structure"]
impl crate::Readable for FftReserved2Spec {}
#[doc = "`write(|w| ..)` method takes [`fft_reserved_2::W`](W) writer structure"]
impl crate::Writable for FftReserved2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFT_RESERVED_2 to value 0"]
impl crate::Resettable for FftReserved2Spec {
    const RESET_VALUE: u32 = 0;
}
