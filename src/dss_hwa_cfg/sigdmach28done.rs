#[doc = "Register `SIGDMACH28DONE` reader"]
pub type R = crate::R<Sigdmach28doneSpec>;
#[doc = "Register `SIGDMACH28DONE` writer"]
pub type W = crate::W<Sigdmach28doneSpec>;
#[doc = "Field `SIGDMACH28DONE` reader - 31:0\\]
Signature for DMA channel 28 completion : 0x1000_0000"]
pub type Sigdmach28doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH28DONE` writer - 31:0\\]
Signature for DMA channel 28 completion : 0x1000_0000"]
pub type Sigdmach28doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 28 completion : 0x1000_0000"]
    #[inline(always)]
    pub fn sigdmach28done(&self) -> Sigdmach28doneR {
        Sigdmach28doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 28 completion : 0x1000_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach28done(&mut self) -> Sigdmach28doneW<Sigdmach28doneSpec> {
        Sigdmach28doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH28DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach28done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach28done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach28doneSpec;
impl crate::RegisterSpec for Sigdmach28doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach28done::R`](R) reader structure"]
impl crate::Readable for Sigdmach28doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach28done::W`](W) writer structure"]
impl crate::Writable for Sigdmach28doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH28DONE to value 0"]
impl crate::Resettable for Sigdmach28doneSpec {
    const RESET_VALUE: u32 = 0;
}
