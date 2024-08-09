#[doc = "Register `DMA2HWA_TRIG` reader"]
pub type R = crate::R<Dma2hwaTrigSpec>;
#[doc = "Register `DMA2HWA_TRIG` writer"]
pub type W = crate::W<Dma2hwaTrigSpec>;
#[doc = "Field `dma2hwa_trigger` reader - 31:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 32-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
pub type Dma2hwaTriggerR = crate::FieldReader<u32>;
#[doc = "Field `dma2hwa_trigger` writer - 31:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 32-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
pub type Dma2hwaTriggerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 32-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
    #[inline(always)]
    pub fn dma2hwa_trigger(&self) -> Dma2hwaTriggerR {
        Dma2hwaTriggerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DMA trigger register: This register is relevant whenever DMA triggered mode is used (i.e., TRIGMODE = 011b). Whenever a DMA channel has finished copying input samples into the local memory of the accelerator and wants to trigger the accelerator, the procedure to follow is to use a second linked DMA channel to write a 16-bit one-hot signature into this register to trigger the accelerator. In DMA triggered mode, the State Machine keeps monitoring this 32-bit register and waits as long as a specific bit (see DMA2ACC_CHANNEL_TRIGSRC) in this register is zero. The second linked DMA channel writes a one-hot signature that sets the specific bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dma2hwa_trigger(&mut self) -> Dma2hwaTriggerW<Dma2hwaTrigSpec> {
        Dma2hwaTriggerW::new(self, 0)
    }
}
#[doc = "DMA2HWA_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2hwa_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2hwa_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2hwaTrigSpec;
impl crate::RegisterSpec for Dma2hwaTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2hwa_trig::R`](R) reader structure"]
impl crate::Readable for Dma2hwaTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2hwa_trig::W`](W) writer structure"]
impl crate::Writable for Dma2hwaTrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA2HWA_TRIG to value 0"]
impl crate::Resettable for Dma2hwaTrigSpec {
    const RESET_VALUE: u32 = 0;
}
