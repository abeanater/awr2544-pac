#[doc = "Register `DITHER_TWID_EN` reader"]
pub type R = crate::R<DitherTwidEnSpec>;
#[doc = "Register `DITHER_TWID_EN` writer"]
pub type W = crate::W<DitherTwidEnSpec>;
#[doc = "Field `dither_twid_en` reader - 0:0\\]
Twiddle factor dithering enable: This register-bit is used to enable and disable dithering of twiddle factors in the FFT. The twiddle factors are 24-bits wide (24-bits for each I and Q), but they are quantized to 21-bits before twiddle factor multiplication. This quantization is implemented with dithering on the LSB, to avoid periodic quantization pattern affecting SFDR performance of the FFT. TI recommends keeping this register bit set to 1 (for example, dithering enabled), with appropriate LSFR seed loaded (see the following)."]
pub type DitherTwidEnR = crate::BitReader;
#[doc = "Field `dither_twid_en` writer - 0:0\\]
Twiddle factor dithering enable: This register-bit is used to enable and disable dithering of twiddle factors in the FFT. The twiddle factors are 24-bits wide (24-bits for each I and Q), but they are quantized to 21-bits before twiddle factor multiplication. This quantization is implemented with dithering on the LSB, to avoid periodic quantization pattern affecting SFDR performance of the FFT. TI recommends keeping this register bit set to 1 (for example, dithering enabled), with appropriate LSFR seed loaded (see the following)."]
pub type DitherTwidEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Twiddle factor dithering enable: This register-bit is used to enable and disable dithering of twiddle factors in the FFT. The twiddle factors are 24-bits wide (24-bits for each I and Q), but they are quantized to 21-bits before twiddle factor multiplication. This quantization is implemented with dithering on the LSB, to avoid periodic quantization pattern affecting SFDR performance of the FFT. TI recommends keeping this register bit set to 1 (for example, dithering enabled), with appropriate LSFR seed loaded (see the following)."]
    #[inline(always)]
    pub fn dither_twid_en(&self) -> DitherTwidEnR {
        DitherTwidEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Twiddle factor dithering enable: This register-bit is used to enable and disable dithering of twiddle factors in the FFT. The twiddle factors are 24-bits wide (24-bits for each I and Q), but they are quantized to 21-bits before twiddle factor multiplication. This quantization is implemented with dithering on the LSB, to avoid periodic quantization pattern affecting SFDR performance of the FFT. TI recommends keeping this register bit set to 1 (for example, dithering enabled), with appropriate LSFR seed loaded (see the following)."]
    #[inline(always)]
    #[must_use]
    pub fn dither_twid_en(&mut self) -> DitherTwidEnW<DitherTwidEnSpec> {
        DitherTwidEnW::new(self, 0)
    }
}
#[doc = "DITHER_TWID_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`dither_twid_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dither_twid_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DitherTwidEnSpec;
impl crate::RegisterSpec for DitherTwidEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dither_twid_en::R`](R) reader structure"]
impl crate::Readable for DitherTwidEnSpec {}
#[doc = "`write(|w| ..)` method takes [`dither_twid_en::W`](W) writer structure"]
impl crate::Writable for DitherTwidEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DITHER_TWID_EN to value 0"]
impl crate::Resettable for DitherTwidEnSpec {
    const RESET_VALUE: u32 = 0;
}
