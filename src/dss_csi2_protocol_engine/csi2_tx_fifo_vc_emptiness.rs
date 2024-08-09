#[doc = "Register `CSI2_TX_FIFO_VC_EMPTINESS` reader"]
pub type R = crate::R<Csi2TxFifoVcEmptinessSpec>;
#[doc = "Register `CSI2_TX_FIFO_VC_EMPTINESS` writer"]
pub type W = crate::W<Csi2TxFifoVcEmptinessSpec>;
#[doc = "Field `VC0_FIFO_EMPTINESS` reader - 7:0\\]
Emptiness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc0FifoEmptinessR = crate::FieldReader;
#[doc = "Field `VC0_FIFO_EMPTINESS` writer - 7:0\\]
Emptiness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc0FifoEmptinessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC1_FIFO_EMPTINESS` reader - 15:8\\]
Emptiness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc1FifoEmptinessR = crate::FieldReader;
#[doc = "Field `VC1_FIFO_EMPTINESS` writer - 15:8\\]
Emptiness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc1FifoEmptinessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC2_FIFO_EMPTINESS` reader - 23:16\\]
Emptiness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc2FifoEmptinessR = crate::FieldReader;
#[doc = "Field `VC2_FIFO_EMPTINESS` writer - 23:16\\]
Emptiness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc2FifoEmptinessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VC3_FIFO_EMPTINESS` reader - 31:24\\]
Emptiness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc3FifoEmptinessR = crate::FieldReader;
#[doc = "Field `VC3_FIFO_EMPTINESS` writer - 31:24\\]
Emptiness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
pub type Vc3FifoEmptinessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Emptiness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc0_fifo_emptiness(&self) -> Vc0FifoEmptinessR {
        Vc0FifoEmptinessR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Emptiness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc1_fifo_emptiness(&self) -> Vc1FifoEmptinessR {
        Vc1FifoEmptinessR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Emptiness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc2_fifo_emptiness(&self) -> Vc2FifoEmptinessR {
        Vc2FifoEmptinessR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Emptiness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    pub fn vc3_fifo_emptiness(&self) -> Vc3FifoEmptinessR {
        Vc3FifoEmptinessR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Emptiness of the FIFO allocated for virtual channel 0.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc0_fifo_emptiness(&mut self) -> Vc0FifoEmptinessW<Csi2TxFifoVcEmptinessSpec> {
        Vc0FifoEmptinessW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Emptiness of the FIFO allocated for virtual channel 1.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc1_fifo_emptiness(&mut self) -> Vc1FifoEmptinessW<Csi2TxFifoVcEmptinessSpec> {
        Vc1FifoEmptinessW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Emptiness of the FIFO allocated for virtual channel 2.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc2_fifo_emptiness(&mut self) -> Vc2FifoEmptinessW<Csi2TxFifoVcEmptinessSpec> {
        Vc2FifoEmptinessW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Emptiness of the FIFO allocated for virtual channel 3.The valid values are from 0 to CSI2_GNQ.TX_FIFODEPTH-1 corresponding to 1x33-bit,...up to CSI2_GNQ.TX_FIFODEPTH x33-bit."]
    #[inline(always)]
    #[must_use]
    pub fn vc3_fifo_emptiness(&mut self) -> Vc3FifoEmptinessW<Csi2TxFifoVcEmptinessSpec> {
        Vc3FifoEmptinessW::new(self, 24)
    }
}
#[doc = "Defines the emptiness of each space allocated for each virtual channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_fifo_vc_emptiness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_fifo_vc_emptiness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TxFifoVcEmptinessSpec;
impl crate::RegisterSpec for Csi2TxFifoVcEmptinessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_tx_fifo_vc_emptiness::R`](R) reader structure"]
impl crate::Readable for Csi2TxFifoVcEmptinessSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_tx_fifo_vc_emptiness::W`](W) writer structure"]
impl crate::Writable for Csi2TxFifoVcEmptinessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TX_FIFO_VC_EMPTINESS to value 0"]
impl crate::Resettable for Csi2TxFifoVcEmptinessSpec {
    const RESET_VALUE: u32 = 0;
}
