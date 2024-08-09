#[doc = "Register `CSI2_TX_FIFO_VC_SIZE` reader"]
pub type R = crate::R<Csi2TxFifoVcSizeSpec>;
#[doc = "Register `CSI2_TX_FIFO_VC_SIZE` writer"]
pub type W = crate::W<Csi2TxFifoVcSizeSpec>;
#[doc = "Field `VC0_FIFO_ADD` reader - 2:0\\]
Address of the space allocated in the FIFO for virtual channel 0.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc0FifoAddR = crate::FieldReader;
#[doc = "Field `VC0_FIFO_ADD` writer - 2:0\\]
Address of the space allocated in the FIFO for virtual channel 0.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc0FifoAddW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VC0_FIFO_SIZE` reader - 7:4\\]
Size of the FIFO allocated for virtual channel 0.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc0FifoSizeR = crate::FieldReader;
#[doc = "Field `VC0_FIFO_SIZE` writer - 7:4\\]
Size of the FIFO allocated for virtual channel 0.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc0FifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VC1_FIFO_ADD` reader - 10:8\\]
Address of the space allocated in the FIFO for virtual channel 1.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc1FifoAddR = crate::FieldReader;
#[doc = "Field `VC1_FIFO_ADD` writer - 10:8\\]
Address of the space allocated in the FIFO for virtual channel 1.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc1FifoAddW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC1_FIFO_SIZE` reader - 15:12\\]
Size of the FIFO allocated for virtual channel 1.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc1FifoSizeR = crate::FieldReader;
#[doc = "Field `VC1_FIFO_SIZE` writer - 15:12\\]
Size of the FIFO allocated for virtual channel 1.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc1FifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VC2_FIFO_ADD` reader - 18:16\\]
Address of the space allocated in the FIFO for virtual channel 2.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc2FifoAddR = crate::FieldReader;
#[doc = "Field `VC2_FIFO_ADD` writer - 18:16\\]
Address of the space allocated in the FIFO for virtual channel 2.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc2FifoAddW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - "]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - "]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC2_FIFO_SIZE` reader - 23:20\\]
Size of the FIFO allocated for virtual channel 2.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc2FifoSizeR = crate::FieldReader;
#[doc = "Field `VC2_FIFO_SIZE` writer - 23:20\\]
Size of the FIFO allocated for virtual channel 2.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc2FifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VC3_FIFO_ADD` reader - 26:24\\]
Address of the space allocated in the FIFO for virtual channel 3.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc3FifoAddR = crate::FieldReader;
#[doc = "Field `VC3_FIFO_ADD` writer - 26:24\\]
Address of the space allocated in the FIFO for virtual channel 3.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
pub type Vc3FifoAddW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED2` reader - "]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC3_FIFO_SIZE` reader - 31:28\\]
Size of the FIFO allocated for virtual channel 3.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc3FifoSizeR = crate::FieldReader;
#[doc = "Field `VC3_FIFO_SIZE` writer - 31:28\\]
Size of the FIFO allocated for virtual channel 3.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
pub type Vc3FifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Address of the space allocated in the FIFO for virtual channel 0.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    pub fn vc0_fifo_add(&self) -> Vc0FifoAddR {
        Vc0FifoAddR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the FIFO allocated for virtual channel 0.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    pub fn vc0_fifo_size(&self) -> Vc0FifoSizeR {
        Vc0FifoSizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Address of the space allocated in the FIFO for virtual channel 1.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    pub fn vc1_fifo_add(&self) -> Vc1FifoAddR {
        Vc1FifoAddR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Size of the FIFO allocated for virtual channel 1.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    pub fn vc1_fifo_size(&self) -> Vc1FifoSizeR {
        Vc1FifoSizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Address of the space allocated in the FIFO for virtual channel 2.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    pub fn vc2_fifo_add(&self) -> Vc2FifoAddR {
        Vc2FifoAddR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Size of the FIFO allocated for virtual channel 2.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    pub fn vc2_fifo_size(&self) -> Vc2FifoSizeR {
        Vc2FifoSizeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Address of the space allocated in the FIFO for virtual channel 3.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    pub fn vc3_fifo_add(&self) -> Vc3FifoAddR {
        Vc3FifoAddR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Size of the FIFO allocated for virtual channel 3.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    pub fn vc3_fifo_size(&self) -> Vc3FifoSizeR {
        Vc3FifoSizeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Address of the space allocated in the FIFO for virtual channel 0.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    #[must_use]
    pub fn vc0_fifo_add(&mut self) -> Vc0FifoAddW<Csi2TxFifoVcSizeSpec> {
        Vc0FifoAddW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Size of the FIFO allocated for virtual channel 0.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    #[must_use]
    pub fn vc0_fifo_size(&mut self) -> Vc0FifoSizeW<Csi2TxFifoVcSizeSpec> {
        Vc0FifoSizeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Address of the space allocated in the FIFO for virtual channel 1.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    #[must_use]
    pub fn vc1_fifo_add(&mut self) -> Vc1FifoAddW<Csi2TxFifoVcSizeSpec> {
        Vc1FifoAddW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2TxFifoVcSizeSpec> {
        Reserved1W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Size of the FIFO allocated for virtual channel 1.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    #[must_use]
    pub fn vc1_fifo_size(&mut self) -> Vc1FifoSizeW<Csi2TxFifoVcSizeSpec> {
        Vc1FifoSizeW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Address of the space allocated in the FIFO for virtual channel 2.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    #[must_use]
    pub fn vc2_fifo_add(&mut self) -> Vc2FifoAddW<Csi2TxFifoVcSizeSpec> {
        Vc2FifoAddW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2TxFifoVcSizeSpec> {
        Reserved3W::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Size of the FIFO allocated for virtual channel 2.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    #[must_use]
    pub fn vc2_fifo_size(&mut self) -> Vc2FifoSizeW<Csi2TxFifoVcSizeSpec> {
        Vc2FifoSizeW::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Address of the space allocated in the FIFO for virtual channel 3.The valid values are from 0 to 7 for a size of the FIFO of 256x33bits corresponding to 0, 32, 64,... for the entry address."]
    #[inline(always)]
    #[must_use]
    pub fn vc3_fifo_add(&mut self) -> Vc3FifoAddW<Csi2TxFifoVcSizeSpec> {
        Vc3FifoAddW::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2TxFifoVcSizeSpec> {
        Reserved2W::new(self, 27)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Size of the FIFO allocated for virtual channel 3.The valid values are from 0 to 8 for a size of the FIFO of 256x33bits corresponding to 0x33bits, 32x33bits, 64x33bits..."]
    #[inline(always)]
    #[must_use]
    pub fn vc3_fifo_size(&mut self) -> Vc3FifoSizeW<Csi2TxFifoVcSizeSpec> {
        Vc3FifoSizeW::new(self, 28)
    }
}
#[doc = "Defines the corresponding memory entries allocated for each virtual channel. The virtual channel shall be disabled in order to allocate/un-allocate some entries in the TX FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_fifo_vc_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_fifo_vc_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TxFifoVcSizeSpec;
impl crate::RegisterSpec for Csi2TxFifoVcSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_tx_fifo_vc_size::R`](R) reader structure"]
impl crate::Readable for Csi2TxFifoVcSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_tx_fifo_vc_size::W`](W) writer structure"]
impl crate::Writable for Csi2TxFifoVcSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TX_FIFO_VC_SIZE to value 0"]
impl crate::Resettable for Csi2TxFifoVcSizeSpec {
    const RESET_VALUE: u32 = 0;
}
