#[doc = "Register `SIGDMACH30DONE` reader"]
pub type R = crate::R<Sigdmach30doneSpec>;
#[doc = "Register `SIGDMACH30DONE` writer"]
pub type W = crate::W<Sigdmach30doneSpec>;
#[doc = "Field `SIGDMACH30DONE` reader - 31:0\\]
Signature for DMA channel 30 completion : 0x4000_0000"]
pub type Sigdmach30doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH30DONE` writer - 31:0\\]
Signature for DMA channel 30 completion : 0x4000_0000"]
pub type Sigdmach30doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 30 completion : 0x4000_0000"]
    #[inline(always)]
    pub fn sigdmach30done(&self) -> Sigdmach30doneR {
        Sigdmach30doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 30 completion : 0x4000_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach30done(&mut self) -> Sigdmach30doneW<Sigdmach30doneSpec> {
        Sigdmach30doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH30DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach30done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach30done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach30doneSpec;
impl crate::RegisterSpec for Sigdmach30doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach30done::R`](R) reader structure"]
impl crate::Readable for Sigdmach30doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach30done::W`](W) writer structure"]
impl crate::Writable for Sigdmach30doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH30DONE to value 0"]
impl crate::Resettable for Sigdmach30doneSpec {
    const RESET_VALUE: u32 = 0;
}
