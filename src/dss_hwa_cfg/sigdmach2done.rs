#[doc = "Register `SIGDMACH2DONE` reader"]
pub type R = crate::R<Sigdmach2doneSpec>;
#[doc = "Register `SIGDMACH2DONE` writer"]
pub type W = crate::W<Sigdmach2doneSpec>;
#[doc = "Field `SIGDMACH2DONE` reader - 31:0\\]
Signature for DMA channel 2 completion : 0x0000_0004"]
pub type Sigdmach2doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH2DONE` writer - 31:0\\]
Signature for DMA channel 2 completion : 0x0000_0004"]
pub type Sigdmach2doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 2 completion : 0x0000_0004"]
    #[inline(always)]
    pub fn sigdmach2done(&self) -> Sigdmach2doneR {
        Sigdmach2doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 2 completion : 0x0000_0004"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach2done(&mut self) -> Sigdmach2doneW<Sigdmach2doneSpec> {
        Sigdmach2doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH2DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach2done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach2done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach2doneSpec;
impl crate::RegisterSpec for Sigdmach2doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach2done::R`](R) reader structure"]
impl crate::Readable for Sigdmach2doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach2done::W`](W) writer structure"]
impl crate::Writable for Sigdmach2doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH2DONE to value 0"]
impl crate::Resettable for Sigdmach2doneSpec {
    const RESET_VALUE: u32 = 0;
}
