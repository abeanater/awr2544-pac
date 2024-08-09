#[doc = "Register `SIGDMACH27DONE` reader"]
pub type R = crate::R<Sigdmach27doneSpec>;
#[doc = "Register `SIGDMACH27DONE` writer"]
pub type W = crate::W<Sigdmach27doneSpec>;
#[doc = "Field `SIGDMACH27DONE` reader - 31:0\\]
Signature for DMA channel 27 completion : 0x0800_0000"]
pub type Sigdmach27doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH27DONE` writer - 31:0\\]
Signature for DMA channel 27 completion : 0x0800_0000"]
pub type Sigdmach27doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 27 completion : 0x0800_0000"]
    #[inline(always)]
    pub fn sigdmach27done(&self) -> Sigdmach27doneR {
        Sigdmach27doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 27 completion : 0x0800_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach27done(&mut self) -> Sigdmach27doneW<Sigdmach27doneSpec> {
        Sigdmach27doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH27DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach27done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach27done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach27doneSpec;
impl crate::RegisterSpec for Sigdmach27doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach27done::R`](R) reader structure"]
impl crate::Readable for Sigdmach27doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach27done::W`](W) writer structure"]
impl crate::Writable for Sigdmach27doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH27DONE to value 0"]
impl crate::Resettable for Sigdmach27doneSpec {
    const RESET_VALUE: u32 = 0;
}
