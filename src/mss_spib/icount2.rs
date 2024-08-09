#[doc = "Register `ICOUNT2` reader"]
pub type R = crate::R<Icount2Spec>;
#[doc = "Register `ICOUNT2` writer"]
pub type W = crate::W<Icount2Spec>;
#[doc = "Field `COUNT` reader - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Actual number of remaining DMA transfer COUNTx is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set. Since the real COUNTx is always ICLOUNTx +1, the 17th bit of COUNTx is available on DMAxCTRL(6) bit."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<Icount2Spec> {
        CountW::new(self, 0)
    }
}
#[doc = "MibSPI DMAxCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`icount2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icount2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icount2Spec;
impl crate::RegisterSpec for Icount2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`icount2::R`](R) reader structure"]
impl crate::Readable for Icount2Spec {}
#[doc = "`write(|w| ..)` method takes [`icount2::W`](W) writer structure"]
impl crate::Writable for Icount2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ICOUNT2 to value 0"]
impl crate::Resettable for Icount2Spec {
    const RESET_VALUE: u16 = 0;
}
