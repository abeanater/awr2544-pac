#[doc = "Register `CLR_FFTCLIP` reader"]
pub type R = crate::R<ClrFftclipSpec>;
#[doc = "Register `CLR_FFTCLIP` writer"]
pub type W = crate::W<ClrFftclipSpec>;
#[doc = "Field `clr_fftclip` reader - 0:0\\]
Clear FFT Clip Status register: This register bit, when set, clears the FFTCLIP register. It s a self clearing bit"]
pub type ClrFftclipR = crate::BitReader;
#[doc = "Field `clr_fftclip` writer - 0:0\\]
Clear FFT Clip Status register: This register bit, when set, clears the FFTCLIP register. It s a self clearing bit"]
pub type ClrFftclipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear FFT Clip Status register: This register bit, when set, clears the FFTCLIP register. It s a self clearing bit"]
    #[inline(always)]
    pub fn clr_fftclip(&self) -> ClrFftclipR {
        ClrFftclipR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear FFT Clip Status register: This register bit, when set, clears the FFTCLIP register. It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_fftclip(&mut self) -> ClrFftclipW<ClrFftclipSpec> {
        ClrFftclipW::new(self, 0)
    }
}
#[doc = "CLR_FFTCLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_fftclip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_fftclip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrFftclipSpec;
impl crate::RegisterSpec for ClrFftclipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_fftclip::R`](R) reader structure"]
impl crate::Readable for ClrFftclipSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_fftclip::W`](W) writer structure"]
impl crate::Writable for ClrFftclipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_FFTCLIP to value 0"]
impl crate::Resettable for ClrFftclipSpec {
    const RESET_VALUE: u32 = 0;
}
