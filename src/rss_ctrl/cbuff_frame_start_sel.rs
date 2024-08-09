#[doc = "Register `CBUFF_FRAME_START_SEL` reader"]
pub type R = crate::R<CbuffFrameStartSelSpec>;
#[doc = "Register `CBUFF_FRAME_START_SEL` writer"]
pub type W = crate::W<CbuffFrameStartSelSpec>;
#[doc = "Field `sel` reader - 0:0\\]
writing: 1'b0: selects frame_start from DFE 1'b1: Selects frame_start from chirp_avail (adc capture complete)"]
pub type SelR = crate::BitReader;
#[doc = "Field `sel` writer - 0:0\\]
writing: 1'b0: selects frame_start from DFE 1'b1: Selects frame_start from chirp_avail (adc capture complete)"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing: 1'b0: selects frame_start from DFE 1'b1: Selects frame_start from chirp_avail (adc capture complete)"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing: 1'b0: selects frame_start from DFE 1'b1: Selects frame_start from chirp_avail (adc capture complete)"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<CbuffFrameStartSelSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "CBUFF_FRAME_START_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`cbuff_frame_start_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbuff_frame_start_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CbuffFrameStartSelSpec;
impl crate::RegisterSpec for CbuffFrameStartSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cbuff_frame_start_sel::R`](R) reader structure"]
impl crate::Readable for CbuffFrameStartSelSpec {}
#[doc = "`write(|w| ..)` method takes [`cbuff_frame_start_sel::W`](W) writer structure"]
impl crate::Writable for CbuffFrameStartSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CBUFF_FRAME_START_SEL to value 0"]
impl crate::Resettable for CbuffFrameStartSelSpec {
    const RESET_VALUE: u32 = 0;
}
