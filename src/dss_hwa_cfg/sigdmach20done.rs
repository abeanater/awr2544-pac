#[doc = "Register `SIGDMACH20DONE` reader"]
pub type R = crate::R<Sigdmach20doneSpec>;
#[doc = "Register `SIGDMACH20DONE` writer"]
pub type W = crate::W<Sigdmach20doneSpec>;
#[doc = "Field `SIGDMACH20DONE` reader - 31:0\\]
Signature for DMA channel 20 completion : 0x0010_0000"]
pub type Sigdmach20doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH20DONE` writer - 31:0\\]
Signature for DMA channel 20 completion : 0x0010_0000"]
pub type Sigdmach20doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 20 completion : 0x0010_0000"]
    #[inline(always)]
    pub fn sigdmach20done(&self) -> Sigdmach20doneR {
        Sigdmach20doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 20 completion : 0x0010_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach20done(&mut self) -> Sigdmach20doneW<Sigdmach20doneSpec> {
        Sigdmach20doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH20DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach20done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach20done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach20doneSpec;
impl crate::RegisterSpec for Sigdmach20doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach20done::R`](R) reader structure"]
impl crate::Readable for Sigdmach20doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach20done::W`](W) writer structure"]
impl crate::Writable for Sigdmach20doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH20DONE to value 0"]
impl crate::Resettable for Sigdmach20doneSpec {
    const RESET_VALUE: u32 = 0;
}
