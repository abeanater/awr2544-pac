#[doc = "Register `SIGDMACH31DONE` reader"]
pub type R = crate::R<Sigdmach31doneSpec>;
#[doc = "Register `SIGDMACH31DONE` writer"]
pub type W = crate::W<Sigdmach31doneSpec>;
#[doc = "Field `SIGDMACH31DONE` reader - 31:0\\]
Signature for DMA channel 31 completion : 0x8000_0000"]
pub type Sigdmach31doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH31DONE` writer - 31:0\\]
Signature for DMA channel 31 completion : 0x8000_0000"]
pub type Sigdmach31doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 31 completion : 0x8000_0000"]
    #[inline(always)]
    pub fn sigdmach31done(&self) -> Sigdmach31doneR {
        Sigdmach31doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 31 completion : 0x8000_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach31done(&mut self) -> Sigdmach31doneW<Sigdmach31doneSpec> {
        Sigdmach31doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH31DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach31done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach31done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach31doneSpec;
impl crate::RegisterSpec for Sigdmach31doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach31done::R`](R) reader structure"]
impl crate::Readable for Sigdmach31doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach31done::W`](W) writer structure"]
impl crate::Writable for Sigdmach31doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH31DONE to value 0"]
impl crate::Resettable for Sigdmach31doneSpec {
    const RESET_VALUE: u32 = 0;
}
