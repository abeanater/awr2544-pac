#[doc = "Register `SIGDMACH21DONE` reader"]
pub type R = crate::R<Sigdmach21doneSpec>;
#[doc = "Register `SIGDMACH21DONE` writer"]
pub type W = crate::W<Sigdmach21doneSpec>;
#[doc = "Field `SIGDMACH21DONE` reader - 31:0\\]
Signature for DMA channel 21 completion : 0x0020_0000"]
pub type Sigdmach21doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH21DONE` writer - 31:0\\]
Signature for DMA channel 21 completion : 0x0020_0000"]
pub type Sigdmach21doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 21 completion : 0x0020_0000"]
    #[inline(always)]
    pub fn sigdmach21done(&self) -> Sigdmach21doneR {
        Sigdmach21doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 21 completion : 0x0020_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach21done(&mut self) -> Sigdmach21doneW<Sigdmach21doneSpec> {
        Sigdmach21doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH21DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach21done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach21done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach21doneSpec;
impl crate::RegisterSpec for Sigdmach21doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach21done::R`](R) reader structure"]
impl crate::Readable for Sigdmach21doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach21done::W`](W) writer structure"]
impl crate::Writable for Sigdmach21doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH21DONE to value 0"]
impl crate::Resettable for Sigdmach21doneSpec {
    const RESET_VALUE: u32 = 0;
}
