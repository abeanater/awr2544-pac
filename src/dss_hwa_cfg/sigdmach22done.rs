#[doc = "Register `SIGDMACH22DONE` reader"]
pub type R = crate::R<Sigdmach22doneSpec>;
#[doc = "Register `SIGDMACH22DONE` writer"]
pub type W = crate::W<Sigdmach22doneSpec>;
#[doc = "Field `SIGDMACH22DONE` reader - 31:0\\]
Signature for DMA channel 22 completion : 0x0040_0000"]
pub type Sigdmach22doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH22DONE` writer - 31:0\\]
Signature for DMA channel 22 completion : 0x0040_0000"]
pub type Sigdmach22doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 22 completion : 0x0040_0000"]
    #[inline(always)]
    pub fn sigdmach22done(&self) -> Sigdmach22doneR {
        Sigdmach22doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 22 completion : 0x0040_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach22done(&mut self) -> Sigdmach22doneW<Sigdmach22doneSpec> {
        Sigdmach22doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH22DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach22done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach22done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach22doneSpec;
impl crate::RegisterSpec for Sigdmach22doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach22done::R`](R) reader structure"]
impl crate::Readable for Sigdmach22doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach22done::W`](W) writer structure"]
impl crate::Writable for Sigdmach22doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH22DONE to value 0"]
impl crate::Resettable for Sigdmach22doneSpec {
    const RESET_VALUE: u32 = 0;
}
