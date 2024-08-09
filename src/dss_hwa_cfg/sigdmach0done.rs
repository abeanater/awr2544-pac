#[doc = "Register `SIGDMACH0DONE` reader"]
pub type R = crate::R<Sigdmach0doneSpec>;
#[doc = "Register `SIGDMACH0DONE` writer"]
pub type W = crate::W<Sigdmach0doneSpec>;
#[doc = "Field `SIGDMACH0DONE` reader - 31:0\\]
Signature for DMA channel 0 completion : 0x0000_0001 Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2HWA_TRIGGER register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
pub type Sigdmach0doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH0DONE` writer - 31:0\\]
Signature for DMA channel 0 completion : 0x0000_0001 Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2HWA_TRIGGER register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
pub type Sigdmach0doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 0 completion : 0x0000_0001 Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2HWA_TRIGGER register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
    #[inline(always)]
    pub fn sigdmach0done(&self) -> Sigdmach0doneR {
        Sigdmach0doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 0 completion : 0x0000_0001 Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2HWA_TRIGGER register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach0done(&mut self) -> Sigdmach0doneW<Sigdmach0doneSpec> {
        Sigdmach0doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH0DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach0done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach0done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach0doneSpec;
impl crate::RegisterSpec for Sigdmach0doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach0done::R`](R) reader structure"]
impl crate::Readable for Sigdmach0doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach0done::W`](W) writer structure"]
impl crate::Writable for Sigdmach0doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH0DONE to value 0"]
impl crate::Resettable for Sigdmach0doneSpec {
    const RESET_VALUE: u32 = 0;
}
