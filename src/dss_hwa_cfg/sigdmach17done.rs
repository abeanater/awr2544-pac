#[doc = "Register `SIGDMACH17DONE` reader"]
pub type R = crate::R<Sigdmach17doneSpec>;
#[doc = "Register `SIGDMACH17DONE` writer"]
pub type W = crate::W<Sigdmach17doneSpec>;
#[doc = "Field `SIGDMACH17DONE` reader - 31:0\\]
Signature for DMA channel 17 completion : 0x0002_0000"]
pub type Sigdmach17doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH17DONE` writer - 31:0\\]
Signature for DMA channel 17 completion : 0x0002_0000"]
pub type Sigdmach17doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 17 completion : 0x0002_0000"]
    #[inline(always)]
    pub fn sigdmach17done(&self) -> Sigdmach17doneR {
        Sigdmach17doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 17 completion : 0x0002_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach17done(&mut self) -> Sigdmach17doneW<Sigdmach17doneSpec> {
        Sigdmach17doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH17DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach17done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach17done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach17doneSpec;
impl crate::RegisterSpec for Sigdmach17doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach17done::R`](R) reader structure"]
impl crate::Readable for Sigdmach17doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach17done::W`](W) writer structure"]
impl crate::Writable for Sigdmach17doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH17DONE to value 0"]
impl crate::Resettable for Sigdmach17doneSpec {
    const RESET_VALUE: u32 = 0;
}
