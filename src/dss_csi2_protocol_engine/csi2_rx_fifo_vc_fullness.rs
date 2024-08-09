#[doc = "Register `CSI2_RX_FIFO_VC_FULLNESS` reader"]
pub type R = crate::R<Csi2RxFifoVcFullnessSpec>;
#[doc = "Register `CSI2_RX_FIFO_VC_FULLNESS` writer"]
pub type W = crate::W<Csi2RxFifoVcFullnessSpec>;
#[doc = "Field `VC0_FIFO_FULLNESS` reader - 7:0\\]
Fullness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc0FifoFullnessR = crate::FieldReader;
#[doc = "Field `VC0_FIFO_FULLNESS` writer - 7:0\\]
Fullness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc0FifoFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC1_FIFO_FULLNESS` reader - 15:8\\]
Fullness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc1FifoFullnessR = crate::FieldReader;
#[doc = "Field `VC1_FIFO_FULLNESS` writer - 15:8\\]
Fullness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc1FifoFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC2_FIFO_FULLNESS` reader - 23:16\\]
Fullness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc2FifoFullnessR = crate::FieldReader;
#[doc = "Field `VC2_FIFO_FULLNESS` writer - 23:16\\]
Fullness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc2FifoFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC3_FIFO_FULLNESS` reader - 31:24\\]
Fullness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc3FifoFullnessR = crate::FieldReader;
#[doc = "Field `VC3_FIFO_FULLNESS` writer - 31:24\\]
Fullness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
pub type Vc3FifoFullnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Fullness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc0_fifo_fullness(&self) -> Vc0FifoFullnessR {
        Vc0FifoFullnessR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Fullness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc1_fifo_fullness(&self) -> Vc1FifoFullnessR {
        Vc1FifoFullnessR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Fullness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc2_fifo_fullness(&self) -> Vc2FifoFullnessR {
        Vc2FifoFullnessR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fullness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc3_fifo_fullness(&self) -> Vc3FifoFullnessR {
        Vc3FifoFullnessR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Fullness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc0_fifo_fullness(&mut self) -> Vc0FifoFullnessW<Csi2RxFifoVcFullnessSpec> {
        Vc0FifoFullnessW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Fullness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc1_fifo_fullness(&mut self) -> Vc1FifoFullnessW<Csi2RxFifoVcFullnessSpec> {
        Vc1FifoFullnessW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Fullness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc2_fifo_fullness(&mut self) -> Vc2FifoFullnessW<Csi2RxFifoVcFullnessSpec> {
        Vc2FifoFullnessW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fullness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.RX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.RX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc3_fifo_fullness(&mut self) -> Vc3FifoFullnessW<Csi2RxFifoVcFullnessSpec> {
        Vc3FifoFullnessW::new(self, 24)
    }
}
#[doc = "Defines the fullness of each space allocated for each virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_rx_fifo_vc_fullness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_rx_fifo_vc_fullness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2RxFifoVcFullnessSpec;
impl crate::RegisterSpec for Csi2RxFifoVcFullnessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_rx_fifo_vc_fullness::R`](R) reader structure"]
impl crate::Readable for Csi2RxFifoVcFullnessSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_rx_fifo_vc_fullness::W`](W) writer structure"]
impl crate::Writable for Csi2RxFifoVcFullnessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_RX_FIFO_VC_FULLNESS to value 0"]
impl crate::Resettable for Csi2RxFifoVcFullnessSpec {
    const RESET_VALUE: u32 = 0;
}
