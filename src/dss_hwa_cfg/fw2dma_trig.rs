#[doc = "Register `FW2DMA_TRIG` reader"]
pub type R = crate::R<Fw2dmaTrigSpec>;
#[doc = "Register `FW2DMA_TRIG` writer"]
pub type W = crate::W<Fw2dmaTrigSpec>;
#[doc = "Field `fw2dma_trigger` reader - 31:0\\]
SW Override for HWA Trigger to DMA by the CPU It s a Self clearing bit"]
pub type Fw2dmaTriggerR = crate::FieldReader<u32>;
#[doc = "Field `fw2dma_trigger` writer - 31:0\\]
SW Override for HWA Trigger to DMA by the CPU It s a Self clearing bit"]
pub type Fw2dmaTriggerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SW Override for HWA Trigger to DMA by the CPU It s a Self clearing bit"]
    #[inline(always)]
    pub fn fw2dma_trigger(&self) -> Fw2dmaTriggerR {
        Fw2dmaTriggerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SW Override for HWA Trigger to DMA by the CPU It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw2dma_trigger(&mut self) -> Fw2dmaTriggerW<Fw2dmaTrigSpec> {
        Fw2dmaTriggerW::new(self, 0)
    }
}
#[doc = "FW2DMA_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`fw2dma_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fw2dma_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fw2dmaTrigSpec;
impl crate::RegisterSpec for Fw2dmaTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw2dma_trig::R`](R) reader structure"]
impl crate::Readable for Fw2dmaTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`fw2dma_trig::W`](W) writer structure"]
impl crate::Writable for Fw2dmaTrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW2DMA_TRIG to value 0"]
impl crate::Resettable for Fw2dmaTrigSpec {
    const RESET_VALUE: u32 = 0;
}
