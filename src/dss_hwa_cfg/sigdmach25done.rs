#[doc = "Register `SIGDMACH25DONE` reader"]
pub type R = crate::R<Sigdmach25doneSpec>;
#[doc = "Register `SIGDMACH25DONE` writer"]
pub type W = crate::W<Sigdmach25doneSpec>;
#[doc = "Field `SIGDMACH25DONE` reader - 31:0\\]
Signature for DMA channel 25 completion : 0x0200_0000"]
pub type Sigdmach25doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH25DONE` writer - 31:0\\]
Signature for DMA channel 25 completion : 0x0200_0000"]
pub type Sigdmach25doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 25 completion : 0x0200_0000"]
    #[inline(always)]
    pub fn sigdmach25done(&self) -> Sigdmach25doneR {
        Sigdmach25doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 25 completion : 0x0200_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach25done(&mut self) -> Sigdmach25doneW<Sigdmach25doneSpec> {
        Sigdmach25doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH25DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach25done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach25done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach25doneSpec;
impl crate::RegisterSpec for Sigdmach25doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach25done::R`](R) reader structure"]
impl crate::Readable for Sigdmach25doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach25done::W`](W) writer structure"]
impl crate::Writable for Sigdmach25doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH25DONE to value 0"]
impl crate::Resettable for Sigdmach25doneSpec {
    const RESET_VALUE: u32 = 0;
}
