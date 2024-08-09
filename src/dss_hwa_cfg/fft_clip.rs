#[doc = "Register `FFT_CLIP` reader"]
pub type R = crate::R<FftClipSpec>;
#[doc = "Register `FFT_CLIP` writer"]
pub type W = crate::W<FftClipSpec>;
#[doc = "Field `fft_clip` reader - 13:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the individual butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate or clip samples. In that case, this saturation event is indicated in the corresponding bit in this status register, so that the Cortex-R4F processor can read it. If multiple FFTs are performed, this status register includes any saturation events happening in any of them. This status register can only be cleared by the R4F, by setting another single-bit register CLR_FFTCLIP, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored. The second MSB of this register indicates clip status corresponding to the radix 3 butterfly.The MSB indicates the clip status corresponding to the conjugate combiner operation in real 2x mode."]
pub type FftClipR = crate::FieldReader<u16>;
#[doc = "Field `fft_clip` writer - 13:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the individual butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate or clip samples. In that case, this saturation event is indicated in the corresponding bit in this status register, so that the Cortex-R4F processor can read it. If multiple FFTs are performed, this status register includes any saturation events happening in any of them. This status register can only be cleared by the R4F, by setting another single-bit register CLR_FFTCLIP, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored. The second MSB of this register indicates clip status corresponding to the radix 3 butterfly.The MSB indicates the clip status corresponding to the conjugate combiner operation in real 2x mode."]
pub type FftClipW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the individual butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate or clip samples. In that case, this saturation event is indicated in the corresponding bit in this status register, so that the Cortex-R4F processor can read it. If multiple FFTs are performed, this status register includes any saturation events happening in any of them. This status register can only be cleared by the R4F, by setting another single-bit register CLR_FFTCLIP, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored. The second MSB of this register indicates clip status corresponding to the radix 3 butterfly.The MSB indicates the clip status corresponding to the conjugate combiner operation in real 2x mode."]
    #[inline(always)]
    pub fn fft_clip(&self) -> FftClipR {
        FftClipR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the individual butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate or clip samples. In that case, this saturation event is indicated in the corresponding bit in this status register, so that the Cortex-R4F processor can read it. If multiple FFTs are performed, this status register includes any saturation events happening in any of them. This status register can only be cleared by the R4F, by setting another single-bit register CLR_FFTCLIP, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored. The second MSB of this register indicates clip status corresponding to the radix 3 butterfly.The MSB indicates the clip status corresponding to the conjugate combiner operation in real 2x mode."]
    #[inline(always)]
    #[must_use]
    pub fn fft_clip(&mut self) -> FftClipW<FftClipSpec> {
        FftClipW::new(self, 0)
    }
}
#[doc = "FFT_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`fft_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fft_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftClipSpec;
impl crate::RegisterSpec for FftClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fft_clip::R`](R) reader structure"]
impl crate::Readable for FftClipSpec {}
#[doc = "`write(|w| ..)` method takes [`fft_clip::W`](W) writer structure"]
impl crate::Writable for FftClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFT_CLIP to value 0"]
impl crate::Resettable for FftClipSpec {
    const RESET_VALUE: u32 = 0;
}
