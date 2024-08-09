#[doc = "Register `SIGDMACH19DONE` reader"]
pub type R = crate::R<Sigdmach19doneSpec>;
#[doc = "Register `SIGDMACH19DONE` writer"]
pub type W = crate::W<Sigdmach19doneSpec>;
#[doc = "Field `SIGDMACH19DONE` reader - 31:0\\]
Signature for DMA channel 19 completion : 0x0008_0000"]
pub type Sigdmach19doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH19DONE` writer - 31:0\\]
Signature for DMA channel 19 completion : 0x0008_0000"]
pub type Sigdmach19doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 19 completion : 0x0008_0000"]
    #[inline(always)]
    pub fn sigdmach19done(&self) -> Sigdmach19doneR {
        Sigdmach19doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 19 completion : 0x0008_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach19done(&mut self) -> Sigdmach19doneW<Sigdmach19doneSpec> {
        Sigdmach19doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH19DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach19done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach19done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach19doneSpec;
impl crate::RegisterSpec for Sigdmach19doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach19done::R`](R) reader structure"]
impl crate::Readable for Sigdmach19doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach19done::W`](W) writer structure"]
impl crate::Writable for Sigdmach19doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH19DONE to value 0"]
impl crate::Resettable for Sigdmach19doneSpec {
    const RESET_VALUE: u32 = 0;
}
