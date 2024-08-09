#[doc = "Register `FFT_RESERVED_3` reader"]
pub type R = crate::R<FftReserved3Spec>;
#[doc = "Register `FFT_RESERVED_3` writer"]
pub type W = crate::W<FftReserved3Spec>;
#[doc = "Field `fft_reserved_3` reader - 31:0\\]
Reserved for future addition"]
pub type FftReserved3R = crate::FieldReader<u32>;
#[doc = "Field `fft_reserved_3` writer - 31:0\\]
Reserved for future addition"]
pub type FftReserved3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn fft_reserved_3(&self) -> FftReserved3R {
        FftReserved3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn fft_reserved_3(&mut self) -> FftReserved3W<FftReserved3Spec> {
        FftReserved3W::new(self, 0)
    }
}
#[doc = "FFT_RESERVED_3\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_reserved_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_reserved_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftReserved3Spec;
impl crate::RegisterSpec for FftReserved3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fft_reserved_3::R`](R) reader structure"]
impl crate::Readable for FftReserved3Spec {}
#[doc = "`write(|w| ..)` method takes [`fft_reserved_3::W`](W) writer structure"]
impl crate::Writable for FftReserved3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFT_RESERVED_3 to value 0"]
impl crate::Resettable for FftReserved3Spec {
    const RESET_VALUE: u32 = 0;
}
