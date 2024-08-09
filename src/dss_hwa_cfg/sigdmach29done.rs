#[doc = "Register `SIGDMACH29DONE` reader"]
pub type R = crate::R<Sigdmach29doneSpec>;
#[doc = "Register `SIGDMACH29DONE` writer"]
pub type W = crate::W<Sigdmach29doneSpec>;
#[doc = "Field `SIGDMACH29DONE` reader - 31:0\\]
Signature for DMA channel 29 completion : 0x2000_0000"]
pub type Sigdmach29doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH29DONE` writer - 31:0\\]
Signature for DMA channel 29 completion : 0x2000_0000"]
pub type Sigdmach29doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 29 completion : 0x2000_0000"]
    #[inline(always)]
    pub fn sigdmach29done(&self) -> Sigdmach29doneR {
        Sigdmach29doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 29 completion : 0x2000_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach29done(&mut self) -> Sigdmach29doneW<Sigdmach29doneSpec> {
        Sigdmach29doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH29DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach29done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach29done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach29doneSpec;
impl crate::RegisterSpec for Sigdmach29doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach29done::R`](R) reader structure"]
impl crate::Readable for Sigdmach29doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach29done::W`](W) writer structure"]
impl crate::Writable for Sigdmach29doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH29DONE to value 0"]
impl crate::Resettable for Sigdmach29doneSpec {
    const RESET_VALUE: u32 = 0;
}
