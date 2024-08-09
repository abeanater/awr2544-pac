#[doc = "Register `SIGDMACH18DONE` reader"]
pub type R = crate::R<Sigdmach18doneSpec>;
#[doc = "Register `SIGDMACH18DONE` writer"]
pub type W = crate::W<Sigdmach18doneSpec>;
#[doc = "Field `SIGDMACH18DONE` reader - 31:0\\]
Signature for DMA channel 18 completion : 0x0004_0000"]
pub type Sigdmach18doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH18DONE` writer - 31:0\\]
Signature for DMA channel 18 completion : 0x0004_0000"]
pub type Sigdmach18doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 18 completion : 0x0004_0000"]
    #[inline(always)]
    pub fn sigdmach18done(&self) -> Sigdmach18doneR {
        Sigdmach18doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 18 completion : 0x0004_0000"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach18done(&mut self) -> Sigdmach18doneW<Sigdmach18doneSpec> {
        Sigdmach18doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH18DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach18done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach18done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach18doneSpec;
impl crate::RegisterSpec for Sigdmach18doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach18done::R`](R) reader structure"]
impl crate::Readable for Sigdmach18doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach18done::W`](W) writer structure"]
impl crate::Writable for Sigdmach18doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH18DONE to value 0"]
impl crate::Resettable for Sigdmach18doneSpec {
    const RESET_VALUE: u32 = 0;
}
